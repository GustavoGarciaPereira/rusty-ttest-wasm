use wasm_bindgen::prelude::*;
use js_sys::Uint8ClampedArray;

// ═══════════════════════════════════════════════════════════════════════════════
//  Fire & Smoke 2D — transient Navier-Stokes with Boussinesq buoyancy
//
//  Follows the same conventions as `backward_step.rs` (staggered grid, flat
//  row-major indexing) but uses Chorin's fractional-step projection instead of
//  the SIMPLE loop: each frame performs advection + diffusion + source terms,
//  then a Poisson solve for pressure with a fixed number of Gauss-Seidel
//  sweeps, then velocity correction.
//
//  Grid layout (identical sizes to backward_step):
//    u(i,j) — east face of cell (i,j)  →  (nx+1) × ny   idx = j·(nx+1) + i
//    v(i,j) — north face of cell (i,j) →  nx × (ny+1)   idx = j·nx + i
//    p, temp, smoke, div — cell centres →  nx × ny      idx = j·nx + i
// ═══════════════════════════════════════════════════════════════════════════════

// ─── Physical constants ───────────────────────────────────────────────────────

const BETA: f32 = 0.5;        // thermal expansion coefficient (Boussinesq)
const G: f32 = 9.81;          // gravity [m/s²]
const NU: f32 = 1.0e-4;       // kinematic viscosity (momentum diffusion)
const TEMP_DIFF: f32 = 0.1;   // thermal diffusivity
const SMOKE_DIFF: f32 = 0.01; // smoke diffusivity
const COOLING_RATE: f32 = 5.0; // Newton cooling toward ambient [1/s]
const FIRE_HEAT: f32 = 200.0;  // heat source [K/s]
const FIRE_SMOKE: f32 = 15.0;  // smoke source [1/s]
const FIRE_LEFT: f32 = 0.4;    // fire region: i in [nx·0.4, nx·0.6)
const FIRE_RIGHT: f32 = 0.6;
const FIRE_J_START: usize = 1; // fire region: j in [1, 5)
const FIRE_J_END: usize = 5;

const POISSON_ITERS: usize = 20; // Gauss-Seidel sweeps per projection
const MAX_GRID: usize = 256;     // per-dimension guard (WASM memory)
const MIN_GRID: usize = 8;
const MAX_SUBSTEPS: usize = 512; // stability cap for very large dt
const DEFAULT_DT: f32 = 1.0 / 60.0;
const TEMP_RENDER_RANGE: f32 = 12.0; // ΔT that maps to white in the palette
const SMOKE_RENDER_RANGE: f32 = 0.6; // smoke density that maps to full soot

// ─── Struct ───────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct FireSmoke {
    nx: usize,
    ny: usize,
    dx: f32,
    dy: f32,

    // Velocities (staggered — same sizes as backward_step)
    u: Vec<f32>,   // (nx+1) × ny
    v: Vec<f32>,   // nx × (ny+1)

    // Pressure and divergence (cell centres)
    p: Vec<f32>,   // nx × ny
    div: Vec<f32>, // nx × ny

    // Scalar fields (cell centres)
    temp: Vec<f32>,  // temperature [K]
    smoke: Vec<f32>, // smoke density

    // Parameters
    dt: f32,
    temp_amb: f32,
}

#[wasm_bindgen]
impl FireSmoke {
    /// Create a new simulation on an `nx × ny` grid (domain is unit-sized).
    pub fn new(nx: usize, ny: usize) -> Result<FireSmoke, String> {
        if nx < MIN_GRID || ny < MIN_GRID || nx > MAX_GRID || ny > MAX_GRID {
            return Err(format!("nx and ny must be in [{MIN_GRID}, {MAX_GRID}]"));
        }
        let temp_amb = 20.0;
        Ok(FireSmoke {
            nx,
            ny,
            dx: 1.0 / nx as f32,
            dy: 1.0 / ny as f32,
            u: vec![0.0; (nx + 1) * ny],
            v: vec![0.0; nx * (ny + 1)],
            p: vec![0.0; nx * ny],
            div: vec![0.0; nx * ny],
            temp: vec![temp_amb; nx * ny],
            smoke: vec![0.0; nx * ny],
            dt: DEFAULT_DT,
            temp_amb,
        })
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn temp_amb(&self) -> f32 {
        self.temp_amb
    }

    pub fn set_temp_amb(&mut self, t: f32) {
        self.temp_amb = t;
    }

    /// Advance the simulation by `dt` seconds (Chorin projection).
    /// Internally sub-steps to keep advection (CFL) and explicit diffusion
    /// stable for any positive `dt`.
    pub fn step(&mut self, dt: f32) -> Result<(), String> {
        if !dt.is_finite() || dt <= 0.0 {
            return Err("dt must be a positive finite number".to_string());
        }
        self.dt = dt;
        self.update(dt);
        Ok(())
    }

    /// Reset all fields (velocities, pressure, smoke; temperature back to ambient).
    pub fn reset(&mut self) {
        for x in self.u.iter_mut() {
            *x = 0.0;
        }
        for x in self.v.iter_mut() {
            *x = 0.0;
        }
        for x in self.p.iter_mut() {
            *x = 0.0;
        }
        for x in self.div.iter_mut() {
            *x = 0.0;
        }
        self.temp.fill(self.temp_amb);
        self.smoke.fill(0.0);
    }

    // ── Field accessors (JS reads these every frame) ─────────────────────────

    pub fn temp(&self) -> Vec<f32> {
        self.temp.clone()
    }

    pub fn smoke(&self) -> Vec<f32> {
        self.smoke.clone()
    }

    pub fn velocity_u(&self) -> Vec<f32> {
        self.u.clone()
    }

    pub fn velocity_v(&self) -> Vec<f32> {
        self.v.clone()
    }

    /// Render temperature + smoke as an RGBA image (nx × ny × 4 bytes),
    /// zero-copy into a `Uint8ClampedArray` for the Canvas.
    pub fn render(&self) -> Uint8ClampedArray {
        let data = self.render_data();
        Uint8ClampedArray::from(&data[..])
    }
}

// ─── Core simulation (private) ────────────────────────────────────────────────

impl FireSmoke {
    #[inline]
    fn update(&mut self, dt: f32) {
        let n_sub = self.substep_count(dt);
        let h = dt / n_sub as f32;
        for _ in 0..n_sub {
            self.substep(h);
        }
    }

    /// Number of sub-steps needed so that each one satisfies the CFL limit and
    /// the explicit-diffusion stability limit.
    #[inline]
    fn substep_count(&self, dt: f32) -> usize {
        let mut max_speed: f32 = 1e-6;
        for &x in self.u.iter().chain(self.v.iter()) {
            let a = x.abs();
            if a > max_speed {
                max_speed = a;
            }
        }
        let k_max = TEMP_DIFF.max(SMOKE_DIFF).max(NU);
        let cell = self.dx.min(self.dy);
        let dt_adv = 0.5 * cell / max_speed;
        let dt_diff = 0.25 * cell * cell / k_max;
        let dt_max = dt_adv.min(dt_diff).min(0.05);
        let n = (dt / dt_max).ceil() as usize;
        n.clamp(1, MAX_SUBSTEPS)
    }

    /// One stable time step: fractional-step Chorin projection.
    #[inline]
    fn substep(&mut self, dt: f32) {
        // 1. Advection (upwind)
        self.advect_velocities(dt);
        advect_scalar(
            &mut self.temp,
            &self.u,
            &self.v,
            self.nx,
            self.ny,
            self.dx,
            self.dy,
            dt,
        );
        advect_scalar(
            &mut self.smoke,
            &self.u,
            &self.v,
            self.nx,
            self.ny,
            self.dx,
            self.dy,
            dt,
        );

        // 2. Diffusion (explicit Laplacian, clamped Neumann at walls)
        self.diffuse_velocities(dt);
        diffuse_scalar(
            &mut self.temp,
            TEMP_DIFF,
            self.nx,
            self.ny,
            self.dx,
            self.dy,
            dt,
        );
        diffuse_scalar(
            &mut self.smoke,
            SMOKE_DIFF,
            self.nx,
            self.ny,
            self.dx,
            self.dy,
            dt,
        );

        // 3. Fire source at the base centre
        self.apply_sources(dt);

        // 4. Newton cooling toward ambient
        self.apply_cooling(dt);

        // 5. Buoyancy (Boussinesq body force on v)
        self.apply_buoyancy(dt);

        // 6. Pressure projection: ∇²p = ∇·u* / dt, then u = u* − dt·∇p
        self.calc_divergence();
        self.solve_pressure(POISSON_ITERS);
        self.correct_velocity(dt);

        // 7. Boundary conditions
        self.apply_boundary_conditions();
    }

    // ── Advection (first-order upwind, staggered) ────────────────────────────

    #[inline]
    fn advect_velocities(&mut self, dt: f32) {
        let nx = self.nx;
        let ny = self.ny;
        let dx = self.dx;
        let dy = self.dy;
        let nxp1 = nx + 1;
        let u = &self.u;
        let v = &self.v;
        let mut un = u.clone();
        let mut vn = v.clone();

        // u-nodes: interior faces i in [1, nx-1), all rows
        for j in 0..ny {
            let jd = if j > 0 { j - 1 } else { 0 };
            let ju = (j + 1).min(ny - 1);
            for i in 1..nx - 1 {
                let idx = j * nxp1 + i;
                // x: self-advection, upwind
                let dux = if u[idx] > 0.0 {
                    u[idx] - u[j * nxp1 + i - 1]
                } else {
                    u[j * nxp1 + i + 1] - u[idx]
                };
                // y: v averaged on the 4 faces around the u-node
                let v_avg = 0.25
                    * (v[j * nx + i - 1]
                        + v[j * nx + i]
                        + v[(j + 1) * nx + i - 1]
                        + v[(j + 1) * nx + i]);
                let duy = if v_avg > 0.0 {
                    u[idx] - u[jd * nxp1 + i]
                } else {
                    u[ju * nxp1 + i] - u[idx]
                };
                un[idx] = u[idx] - dt * (u[idx] * dux / dx + v_avg * duy / dy);
            }
        }

        // v-nodes: interior faces j in [1, ny-1), all columns
        for j in 1..ny - 1 {
            for i in 0..nx {
                let idx = j * nx + i;
                let il = if i > 0 { i - 1 } else { 0 };
                let ir = (i + 1).min(nx - 1);
                // x: u averaged on the 4 faces around the v-node
                let u_avg = 0.25
                    * (u[(j - 1) * nxp1 + i]
                        + u[(j - 1) * nxp1 + i + 1]
                        + u[j * nxp1 + i]
                        + u[j * nxp1 + i + 1]);
                let dvx = if u_avg > 0.0 {
                    v[idx] - v[j * nx + il]
                } else {
                    v[j * nx + ir] - v[idx]
                };
                // y: self-advection, upwind
                let dvy = if v[idx] > 0.0 {
                    v[idx] - v[(j - 1) * nx + i]
                } else {
                    v[(j + 1) * nx + i] - v[idx]
                };
                vn[idx] = v[idx] - dt * (u_avg * dvx / dx + v[idx] * dvy / dy);
            }
        }

        self.u = un;
        self.v = vn;
    }

    // ── Diffusion ─────────────────────────────────────────────────────────────

    #[inline]
    fn diffuse_velocities(&mut self, dt: f32) {
        let nx = self.nx;
        let ny = self.ny;
        let nxp1 = nx + 1;
        let kx = NU * dt / (self.dx * self.dx);
        let ky = NU * dt / (self.dy * self.dy);

        let mut un = self.u.clone();
        for j in 0..ny {
            let jd = if j > 0 { j - 1 } else { 0 };
            let ju = (j + 1).min(ny - 1);
            for i in 1..nx - 1 {
                let idx = j * nxp1 + i;
                let lap = (self.u[j * nxp1 + i + 1] - 2.0 * self.u[idx] + self.u[j * nxp1 + i - 1])
                    * kx
                    + (self.u[ju * nxp1 + i] - 2.0 * self.u[idx] + self.u[jd * nxp1 + i]) * ky;
                un[idx] = self.u[idx] + lap;
            }
        }

        let mut vn = self.v.clone();
        for j in 1..ny - 1 {
            for i in 0..nx {
                let idx = j * nx + i;
                let il = if i > 0 { i - 1 } else { 0 };
                let ir = (i + 1).min(nx - 1);
                let lap = (self.v[j * nx + ir] - 2.0 * self.v[idx] + self.v[j * nx + il]) * kx
                    + (self.v[(j + 1) * nx + i] - 2.0 * self.v[idx] + self.v[(j - 1) * nx + i])
                        * ky;
                vn[idx] = self.v[idx] + lap;
            }
        }

        self.u = un;
        self.v = vn;
    }

    // ── Sources and cooling ───────────────────────────────────────────────────

    /// Inject heat and soot in the fire region: i in [nx·0.4, nx·0.6), j in [1, 5).
    #[inline]
    fn apply_sources(&mut self, dt: f32) {
        let nx = self.nx;
        let i0 = (nx as f32 * FIRE_LEFT) as usize;
        let i1 = (nx as f32 * FIRE_RIGHT) as usize;
        let heat = dt * FIRE_HEAT;
        let soot = dt * FIRE_SMOKE;
        for j in FIRE_J_START..FIRE_J_END.min(self.ny) {
            for i in i0..i1.min(nx) {
                let idx = j * nx + i;
                self.temp[idx] += heat;
                self.smoke[idx] += soot;
            }
        }
    }

    /// Newton cooling: dT/dt = −rate·(T − T_amb).
    #[inline]
    fn apply_cooling(&mut self, dt: f32) {
        let factor = dt * COOLING_RATE;
        for t in self.temp.iter_mut() {
            *t -= factor * (*t - self.temp_amb);
        }
    }

    /// Boussinesq body force: v += dt·β·(T_face − T_amb)·g.
    /// Temperature is interpolated from the two cells straddling each v-face.
    #[inline]
    fn apply_buoyancy(&mut self, dt: f32) {
        let nx = self.nx;
        let force = dt * BETA * G;
        for j in 0..self.ny - 1 {
            for i in 0..nx {
                let tf = 0.5 * (self.temp[j * nx + i] + self.temp[(j + 1) * nx + i]);
                self.v[j * nx + i] += force * (tf - self.temp_amb);
            }
        }
    }

    // ── Pressure projection (Chorin) ──────────────────────────────────────────

    /// Divergence at cell centres: ∇·u = (u_e − u_w)/dx + (v_n − v_s)/dy.
    #[inline]
    fn calc_divergence(&mut self) {
        let nx = self.nx;
        let ny = self.ny;
        let nxp1 = nx + 1;
        let inv_dx = 1.0 / self.dx;
        let inv_dy = 1.0 / self.dy;
        for j in 0..ny {
            for i in 0..nx {
                let div = (self.u[j * nxp1 + i + 1] - self.u[j * nxp1 + i]) * inv_dx
                    + (self.v[(j + 1) * nx + i] - self.v[j * nx + i]) * inv_dy;
                self.div[j * nx + i] = div;
            }
        }
    }

    /// Solve ∇²p = ∇·u* / dt by Gauss-Seidel with Neumann (∂p/∂n = 0) walls.
    /// The mean divergence is removed so the Neumann problem is consistent
    /// (the constant of p is arbitrary for the projection).
    #[inline]
    fn solve_pressure(&mut self, iters: usize) {
        let nx = self.nx;
        let ny = self.ny;
        let inv_dx2 = 1.0 / (self.dx * self.dx);
        let inv_dy2 = 1.0 / (self.dy * self.dy);
        let denom = 2.0 * (inv_dx2 + inv_dy2);
        let mean_div = self.div.iter().sum::<f32>() / (nx * ny) as f32;
        for _ in 0..iters {
            for j in 0..ny {
                for i in 0..nx {
                    let idx = j * nx + i;
                    let il = if i > 0 { i - 1 } else { 0 };
                    let ir = (i + 1).min(nx - 1);
                    let jd = if j > 0 { j - 1 } else { 0 };
                    let ju = (j + 1).min(ny - 1);
                    let rhs = (self.div[idx] - mean_div) / self.dt;
                    let lap = (self.p[j * nx + ir] + self.p[j * nx + il]) * inv_dx2
                        + (self.p[ju * nx + i] + self.p[jd * nx + i]) * inv_dy2;
                    self.p[idx] = (lap - rhs) / denom;
                }
            }
        }
    }

    /// u = u* − dt·∇p (interior faces only; walls are re-pinned afterwards).
    #[inline]
    fn correct_velocity(&mut self, dt: f32) {
        let nx = self.nx;
        let ny = self.ny;
        let nxp1 = nx + 1;
        let gx = dt / self.dx;
        let gy = dt / self.dy;
        for j in 0..ny {
            for i in 1..nx {
                let idx = j * nxp1 + i;
                self.u[idx] -= gx * (self.p[j * nx + i] - self.p[j * nx + i - 1]);
            }
        }
        for j in 1..ny {
            for i in 0..nx {
                let idx = j * nx + i;
                self.v[idx] -= gy * (self.p[j * nx + i] - self.p[(j - 1) * nx + i]);
            }
        }
    }

    // ── Boundary conditions ───────────────────────────────────────────────────

    /// Free-slip walls (normal velocity zero) on the sides and floor; open top
    /// (zero gradient) for velocities and scalars; adiabatic walls for T.
    #[inline]
    fn apply_boundary_conditions(&mut self) {
        let nx = self.nx;
        let ny = self.ny;
        let nxp1 = nx + 1;

        // Side walls: u = 0 (no penetration)
        for j in 0..ny {
            self.u[j * nxp1] = 0.0;
            self.u[j * nxp1 + nx] = 0.0;
        }
        // Floor: v = 0
        for i in 0..nx {
            self.v[i] = 0.0;
        }
        // Open top: zero-gradient for v and u
        for i in 0..nx {
            self.v[ny * nx + i] = self.v[(ny - 1) * nx + i];
        }
        for i in 1..nx {
            self.u[(ny - 1) * nxp1 + i] = self.u[(ny - 2) * nxp1 + i];
        }
        // Scalars: zero-gradient everywhere (adiabatic walls, smoke escape at top)
        for j in 0..ny {
            let row = j * nx;
            self.temp[row] = self.temp[row + 1];
            self.temp[row + nx - 1] = self.temp[row + nx - 2];
            self.smoke[row] = self.smoke[row + 1];
            self.smoke[row + nx - 1] = self.smoke[row + nx - 2];
        }
        for i in 0..nx {
            self.temp[i] = self.temp[nx + i];
            self.temp[(ny - 1) * nx + i] = self.temp[(ny - 2) * nx + i];
            self.smoke[i] = self.smoke[nx + i];
            self.smoke[(ny - 1) * nx + i] = self.smoke[(ny - 2) * nx + i];
        }
    }

    // ── Rendering (pure Rust, testable natively) ──────────────────────────────

    fn render_data(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.nx * self.ny * 4];
        for j in 0..self.ny {
            // Flip vertically: physical j=0 (floor) must appear at the BOTTOM
            // of the image (canvas y grows downward).
            let row_out = (self.ny - 1 - j) * self.nx;
            for i in 0..self.nx {
                let idx = j * self.nx + i;
                let t = ((self.temp[idx] - self.temp_amb) / TEMP_RENDER_RANGE).clamp(0.0, 1.0);
                let s = (self.smoke[idx] / SMOKE_RENDER_RANGE).clamp(0.0, 1.0);
                let (r, g, b) = fire_color(t);
                // Soot darkens the fire and adds grey
                let dark = 1.0 - 0.6 * s;
                let o = (row_out + i) * 4;
                data[o] = (r as f32 * dark + 30.0 * s) as u8;
                data[o + 1] = (g as f32 * dark + 30.0 * s) as u8;
                data[o + 2] = (b as f32 * dark + 34.0 * s) as u8;
                data[o + 3] = 255;
            }
        }
        data
    }
}

// ─── Free helper functions ────────────────────────────────────────────────────

/// Upwind advection of a cell-centred scalar field q by the staggered (u, v).
#[inline]
fn advect_scalar(
    q: &mut [f32],
    u: &[f32],
    v: &[f32],
    nx: usize,
    ny: usize,
    dx: f32,
    dy: f32,
    dt: f32,
) {
    let nxp1 = nx + 1;
    let mut qn = q.to_vec();
    for j in 0..ny {
        let jd = if j > 0 { j - 1 } else { 0 };
        let ju = (j + 1).min(ny - 1);
        for i in 0..nx {
            let idx = j * nx + i;
            let il = if i > 0 { i - 1 } else { 0 };
            let ir = (i + 1).min(nx - 1);
            let uc = 0.5 * (u[j * nxp1 + i] + u[j * nxp1 + i + 1]);
            let vc = 0.5 * (v[j * nx + i] + v[(j + 1) * nx + i]);
            let dqx = if uc > 0.0 {
                q[idx] - q[j * nx + il]
            } else {
                q[j * nx + ir] - q[idx]
            };
            let dqy = if vc > 0.0 {
                q[idx] - q[jd * nx + i]
            } else {
                q[ju * nx + i] - q[idx]
            };
            qn[idx] = q[idx] - dt * (uc * dqx / dx + vc * dqy / dy);
        }
    }
    q.copy_from_slice(&qn);
}

/// Explicit Laplacian diffusion of a cell-centred scalar field.
/// Clamped neighbours implement Neumann (zero-gradient) walls.
#[inline]
fn diffuse_scalar(q: &mut [f32], k: f32, nx: usize, ny: usize, dx: f32, dy: f32, dt: f32) {
    let kx = k * dt / (dx * dx);
    let ky = k * dt / (dy * dy);
    let mut qn = q.to_vec();
    for j in 0..ny {
        let jd = if j > 0 { j - 1 } else { 0 };
        let ju = (j + 1).min(ny - 1);
        for i in 0..nx {
            let idx = j * nx + i;
            let il = if i > 0 { i - 1 } else { 0 };
            let ir = (i + 1).min(nx - 1);
            let lap = (q[j * nx + ir] - 2.0 * q[idx] + q[j * nx + il]) * kx
                + (q[ju * nx + i] - 2.0 * q[idx] + q[jd * nx + i]) * ky;
            qn[idx] = q[idx] + lap;
        }
    }
    q.copy_from_slice(&qn);
}

/// Fire palette: dark → red → orange → yellow → white.
#[inline]
fn fire_color(t: f32) -> (u8, u8, u8) {
    if t <= 0.25 {
        lerp((15, 15, 28), (220, 50, 10), t / 0.25)
    } else if t <= 0.5 {
        lerp((220, 50, 10), (255, 140, 0), (t - 0.25) / 0.25)
    } else if t <= 0.75 {
        lerp((255, 140, 0), (255, 230, 80), (t - 0.5) / 0.25)
    } else {
        lerp((255, 230, 80), (255, 255, 255), (t - 0.75) / 0.25)
    }
}

#[inline]
fn lerp(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (a.0 as f32 + (b.0 as f32 - a.0 as f32) * t) as u8,
        (a.1 as f32 + (b.1 as f32 - a.1 as f32) * t) as u8,
        (a.2 as f32 + (b.2 as f32 - a.2 as f32) * t) as u8,
    )
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_validates_grid() {
        assert!(FireSmoke::new(0, 10).is_err());
        assert!(FireSmoke::new(10, 3).is_err());
        assert!(FireSmoke::new(300, 10).is_err());
        assert!(FireSmoke::new(40, 30).is_ok());
    }

    #[test]
    fn test_step_rejects_bad_dt() {
        let mut sim = FireSmoke::new(40, 30).unwrap();
        assert!(sim.step(0.0).is_err());
        assert!(sim.step(-1.0).is_err());
        assert!(sim.step(f32::NAN).is_err());
        assert!(sim.step(1.0 / 60.0).is_ok());
    }

    /// Place a hot bubble at the centre and verify the buoyancy force drives
    /// an upward velocity after 10 steps — proves the Boussinesq coupling.
    #[test]
    fn test_hot_bubble_rises() {
        let mut sim = FireSmoke::new(40, 30).unwrap();
        let cx = 20;
        let cy = 15;
        for j in (cy - 1)..=(cy + 1) {
            for i in (cx - 1)..=(cx + 1) {
                sim.temp[j * sim.nx + i] = sim.temp_amb + 100.0;
            }
        }
        for _ in 0..10 {
            sim.step(1.0 / 60.0).unwrap();
        }
        let mut v_up = 0.0_f32;
        for j in 0..sim.ny {
            for i in (cx - 2)..(cx + 3) {
                v_up += sim.v[j * sim.nx + i];
            }
        }
        assert!(v_up > 0.0, "buoyancy should drive upward flow, v_sum = {v_up}");
    }

    /// The fire source must heat the region above the floor and emit smoke.
    #[test]
    fn test_fire_source_heats_and_smokes() {
        let mut sim = FireSmoke::new(40, 30).unwrap();
        for _ in 0..10 {
            sim.step(1.0 / 60.0).unwrap();
        }
        let mut hot = 0usize;
        let mut smoke_total = 0.0_f32;
        for j in FIRE_J_START..FIRE_J_END {
            for i in 0..sim.nx {
                let idx = j * sim.nx + i;
                if sim.temp[idx] > sim.temp_amb + 1.0 {
                    hot += 1;
                }
                smoke_total += sim.smoke[idx];
            }
        }
        assert!(hot > 0, "fire region should heat up above ambient");
        assert!(smoke_total > 0.0, "fire should emit smoke");
    }

    #[test]
    fn test_render_shape() {
        let sim = FireSmoke::new(40, 30).unwrap();
        let data = sim.render_data();
        assert_eq!(data.len(), 40 * 30 * 4);
        // Ambient cells render dark, not pure black
        assert_eq!(data[3], 255); // alpha
        assert!(data[0] >= 10);
    }

    /// Regression: the render must be vertically flipped so the physical floor
    /// (j=0, where the fire sits) appears at the BOTTOM of the image.
    #[test]
    fn test_render_orientation_floor_at_bottom() {
        let mut sim = FireSmoke::new(40, 30).unwrap();
        // Heat a cell on the floor (j=0) and one at the top (j=ny-1)
        sim.temp[0 * sim.nx + 1] = sim.temp_amb + 100.0; // floor
        sim.temp[(sim.ny - 1) * sim.nx + 2] = sim.temp_amb + 100.0; // ceiling
        let data = sim.render_data();
        // In the image, the floor cell must be in the LAST row (bottom)…
        let bottom = ((sim.ny - 1) * sim.nx + 1) * 4;
        let top = (0 * sim.nx + 1) * 4;
        assert!(
            data[bottom] > 100,
            "floor heat must appear at image bottom, got r={}",
            data[bottom]
        );
        assert!(
            data[top] < 60,
            "floor cell must NOT appear at image top, got r={}",
            data[top]
        );
        // …and the ceiling cell must be in the FIRST row (top).
        let bottom_c = ((sim.ny - 1) * sim.nx + 2) * 4;
        let top_c = (0 * sim.nx + 2) * 4;
        assert!(
            data[top_c] > 100,
            "ceiling heat must appear at image top, got r={}",
            data[top_c]
        );
        assert!(
            data[bottom_c] < 60,
            "ceiling cell must NOT appear at image bottom, got r={}",
            data[bottom_c]
        );
    }
}

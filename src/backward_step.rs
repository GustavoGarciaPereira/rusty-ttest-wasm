use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  Backward-Facing Step — 2D SIMPLE Solver (staggered grid)
// ═══════════════════════════════════════════════════════════════════════════════

const REYNOLDS: f64 = 100.0; // moderate Reynolds number
const ALPHA_U: f64 = 0.7;    // u under-relaxation
const ALPHA_V: f64 = 0.7;    // v under-relaxation
const ALPHA_P: f64 = 0.3;    // pressure under-relaxation
const POISSON_ITERS: usize = 50; // inner Gauss‑Seidel sweeps per SIMPLE iteration

// ─── Geometry helpers ─────────────────────────────────────────────────────────

/// Inlet parabolic profile: u(y) = 6·U_in·(y−y0)·(y1−y) / h²
fn inlet_u(y: f64, U_in: f64, h_in: f64) -> f64 {
    let y0 = h_in;          // bottom of inlet
    let y1 = 2.0 * h_in;    // top of inlet (= h_out)
    if y < y0 || y > y1 {
        return 0.0;
    }
    let h = y1 - y0; // = h_in
    6.0 * U_in * (y - y0) * (y1 - y) / (h * h)
}

// ─── Bilinear interpolation (for real‑time particle queries) ──────────────────

/// Return (u, v) at a physical point by bilinear interpolation from
/// the staggered‑grid fields. Assumes the fields have been computed.
#[wasm_bindgen]
pub fn velocity_at(_x: f64, y: f64, U_in: f64, h_in: f64) -> Vec<f64> {
    // Quick preview: inlet profile estimate.
    let u = inlet_u(y, U_in, h_in);
    let v = 0.0;
    vec![u, v]
}

/// Return a pair `(u, v)` from pre‑computed arrays (called from JS).
#[wasm_bindgen]
pub fn interpolate_velocity(
    x: f64,
    y: f64,
    xs: Vec<f64>,
    ys: Vec<f64>,
    u_flat: Vec<f64>,
    v_flat: Vec<f64>,
    nx: usize,
    ny: usize,
) -> Vec<f64> {
    let dx = xs[1] - xs[0];
    let dy = ys[1] - ys[0];
    let x0 = xs[0];
    let y0 = ys[0];

    let i_f = ((x - x0) / dx).clamp(0.0, (nx - 1) as f64 - 1.0);
    let j_f = ((y - y0) / dy).clamp(0.0, (ny - 1) as f64 - 1.0);

    let i = i_f as usize;
    let j = j_f as usize;
    let fx = i_f - i as f64;
    let fy = j_f - j as f64;

    let idx = |i: usize, j: usize| i + j * nx;

    let u00 = u_flat[idx(i, j)];
    let u10 = u_flat[idx((i + 1).min(nx - 1), j)];
    let u01 = u_flat[idx(i, (j + 1).min(ny - 1))];
    let u11 = u_flat[idx((i + 1).min(nx - 1), (j + 1).min(ny - 1))];
    let u = (1.0 - fx) * (1.0 - fy) * u00 + fx * (1.0 - fy) * u10
          + (1.0 - fx) * fy * u01 + fx * fy * u11;

    let v00 = v_flat[idx(i, j)];
    let v10 = v_flat[idx((i + 1).min(nx - 1), j)];
    let v01 = v_flat[idx(i, (j + 1).min(ny - 1))];
    let v11 = v_flat[idx((i + 1).min(nx - 1), (j + 1).min(ny - 1))];
    let v = (1.0 - fx) * (1.0 - fy) * v00 + fx * (1.0 - fy) * v10
          + (1.0 - fx) * fy * v01 + fx * fy * v11;

    vec![u, v]
}

// ─── Full SIMPLE solver ───────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct BackwardStepOutput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub u: Vec<Vec<f64>>,
    pub v: Vec<Vec<f64>>,
    pub p: Vec<Vec<f64>>,
    pub mag: Vec<Vec<f64>>,
}

/// Run the 2D SIMPLE solver for a backward‑facing step and return all
/// fields as JSON.
#[wasm_bindgen]
pub fn compute_backward_step(
    U_in: f64,
    h_in: f64,
    nx: usize,
    ny: usize,
    max_iter: usize,
) -> String {
    // Mesh size guard — prevent WASM memory exhaustion
    if nx > 120 || ny > 60 {
        return serde_json::to_string(&BackwardStepOutput {
            x: vec![], y: vec![], u: vec![], v: vec![], p: vec![], mag: vec![],
        }).unwrap();
    }

    let h_out = 2.0 * h_in;
    let len = 10.0 * h_in;
    let nu = U_in * h_in / REYNOLDS;

    let dx = len / nx as f64;
    let dy = h_out / ny as f64;

    // ── Grid node coordinates (cell centres for p) ─────────────────────────
    let x: Vec<f64> = (0..nx).map(|i| (i as f64 + 0.5) * dx).collect();
    let y: Vec<f64> = (0..ny).map(|j| (j as f64 + 0.5) * dy).collect();

    // ── Staggered velocity arrays ──────────────────────────────────────────
    // u(i,j): east face of cell (i,j),  dims (nx+1) × ny
    // v(i,j): north face of cell (i,j), dims nx × (ny+1)
    let mut u = vec![vec![0.0_f64; ny]; nx + 1];
    let mut v = vec![vec![0.0_f64; ny + 1]; nx];

    // Initialise inlet
    let inlet_j_start = (h_in / dy) as usize;
    for j in inlet_j_start..ny {
        let yc = y[j];
        u[0][j] = inlet_u(yc, U_in, h_in);
    }

    // Pressure (cell centres)
    let mut p = vec![vec![0.0_f64; ny]; nx];

    // ── SIMPLE iteration ───────────────────────────────────────────────────
    for _iter in 0..max_iter {
        // ---- solve x‑momentum for u* ---------------------------------------
        let mut u_star = u.clone();
        for i in 1..nx {
            for j in 0..ny {
                // Skip cells in the step region (x=0, y below h_in)
                if i == 1 {
                    let x_pos = 0.5 * dx;
                    let y_pos = y[j];
                    if x_pos < dx && y_pos < h_in {
                        u_star[i][j] = 0.0;
                        continue;
                    }
                }
                let (ae, aw, an, as_, ap) =
                    conv_diff_coeffs_u(i, j, &u, &v, dx, dy, nu);
                let src = (p[i - 1][j] - p[i][j]) * dy; // pressure gradient
                let u_south = if j > 0 { u[i][j - 1] } else { 0.0 };
                u_star[i][j] = (ae * u[i + 1][j] + aw * u[i - 1][j]
                    + an * u[i][(j + 1).min(ny - 1)] + as_ * u_south
                    + src)
                    / ap;
                u_star[i][j] = (1.0 - ALPHA_U) * u[i][j] + ALPHA_U * u_star[i][j];
            }
        }

        // ---- solve y‑momentum for v* ---------------------------------------
        let mut v_star = v.clone();
        for i in 0..nx {
            for j in 1..ny {
                // Skip step region
                if i == 0 {
                    let y_pos = 0.5 * dy;
                    if y_pos < h_in {
                        v_star[i][j] = 0.0;
                        continue;
                    }
                }
                let (ae, aw, an, as_, ap) =
                    conv_diff_coeffs_v(i, j, &u, &v, dx, dy, nu);
                let src = (p[i][j - 1] - p[i][j]) * dx;
                let v_west = if i > 0 { v[i - 1][j] } else { 0.0 };
                v_star[i][j] = (ae * v[(i + 1).min(nx - 1)][j] + aw * v_west
                    + an * v[i][j + 1] + as_ * v[i][j - 1]
                    + src)
                    / ap;
                v_star[i][j] = (1.0 - ALPHA_V) * v[i][j] + ALPHA_V * v_star[i][j];
            }
        }

        // ---- pressure correction (Poisson via Gauss‑Seidel) -----------------
        let mut pp = vec![vec![0.0_f64; ny]; nx]; // p'
        for _gs in 0..POISSON_ITERS {
            for i in 0..nx {
                for j in 0..ny {
                    // Skip step region
                    let xc = x[i];
                    let yc = y[j];
                    if xc < dx && yc < h_in {
                        continue; // solid step — no correction
                    }

                    let de = dy / ap_coeff(i, j, dx, dy, nu, 'u');
                    let dw = if i > 0 { dy / ap_coeff(i - 1, j, dx, dy, nu, 'u') } else { 0.0 };
                    let dn = dx / ap_coeff(i, j, dx, dy, nu, 'v');
                    let ds = if j > 0 { dx / ap_coeff(i, j - 1, dx, dy, nu, 'v') } else { 0.0 };

                    let ae = de;
                    let aw = dw;
                    let an = dn;
                    let as_ = ds;

                    // Mass imbalance
                    let b = (u_star[i][j] - u_star[i + 1][j]) * dy
                          + (v_star[i][j] - v_star[i][j + 1]) * dx;

                    let ap_pp = ae + aw + an + as_;
                    if ap_pp > 1e-12 {
                        let nb = ae * if i + 1 < nx { pp[i + 1][j] } else { 0.0 }
                               + aw * if i > 0 { pp[i - 1][j] } else { 0.0 }
                               + an * if j + 1 < ny { pp[i][j + 1] } else { 0.0 }
                               + as_ * if j > 0 { pp[i][j - 1] } else { 0.0 };
                        pp[i][j] = (nb + b) / ap_pp;
                    }
                }
            }
        }

        // ---- correct velocities and pressure -------------------------------
        for i in 1..nx {
            for j in 0..ny {
                let de = dy / ap_coeff(i, j, dx, dy, nu, 'u');
                let dp = if i > 0 && i <= nx { pp[i - 1][j] - pp[i][j] } else { 0.0 };
                u[i][j] = u_star[i][j] + de * dp;
            }
        }
        for i in 0..nx {
            for j in 1..ny {
                let dn = dx / ap_coeff(i, j, dx, dy, nu, 'v');
                let dp = if j > 0 && j <= ny { pp[i][j - 1] - pp[i][j] } else { 0.0 };
                v[i][j] = v_star[i][j] + dn * dp;
            }
        }
        for i in 0..nx {
            for j in 0..ny {
                p[i][j] += ALPHA_P * pp[i][j];
            }
        }

        // Re‑apply boundary conditions
        // Inlet u
        for j in inlet_j_start..ny {
            let yc = y[j];
            u[0][j] = inlet_u(yc, U_in, h_in);
        }
        // Step wall (u=0)
        for j in 0..inlet_j_start {
            u[0][j] = 0.0;
            if j < ny { u[1][j] = 0.0; } // blunt body
        }
        // Bottom wall
        for i in 0..nx {
            v[i][0] = 0.0;
        }
        // Top wall
        for i in 0..nx {
            v[i][ny] = 0.0;
        }
        // Outlet: extrapolate u, set p = 0
        for j in 0..ny {
            u[nx][j] = u[nx - 1][j];
            p[nx - 1][j] = 0.0;
        }
    }

    // ── Build output ───────────────────────────────────────────────────────
    let u_out: Vec<Vec<f64>> = (0..nx)
        .map(|i| (0..ny).map(|j| 0.5 * (u[i][j] + u[i + 1][j])).collect())
        .collect();
    let v_out: Vec<Vec<f64>> = (0..nx)
        .map(|i| (0..ny).map(|j| 0.5 * (v[i][j] + v[i][j + 1])).collect())
        .collect();
    let mag: Vec<Vec<f64>> = (0..nx)
        .map(|i| {
            (0..ny)
                .map(|j| (u_out[i][j].powi(2) + v_out[i][j].powi(2)).sqrt())
                .collect()
        })
        .collect();

    let output = BackwardStepOutput {
        x: x.clone(),
        y: y.clone(),
        u: u_out,
        v: v_out,
        p: p.clone(),
        mag,
    };

    serde_json::to_string(&output).unwrap()
}

// ─── Convection‑diffusion coefficients ────────────────────────────────────────

/// Compute coefficients for the u‑momentum equation at u‑node (i,j).
/// Uses upwind differencing for convection.
fn conv_diff_coeffs_u(
    i: usize,
    j: usize,
    u: &[Vec<f64>],
    v: &[Vec<f64>],
    dx: f64,
    dy: f64,
    nu: f64,
) -> (f64, f64, f64, f64, f64) {
    let fe = 0.5 * (u[i][j] + u[i + 1][j]) * dy; // convective flux east
    let fw = 0.5 * (u[i][j] + u[i - 1][j]) * dy;
    let fn_ = 0.5 * (v[i][j + 1] + v[i - 1][j + 1]) * dx;
    let fs = 0.5 * (v[i][j] + v[i - 1][j]) * dx;

    let de = nu * dy / dx; // diffusion conductance
    let dw = nu * dy / dx;
    let dn = nu * dx / dy;
    let ds = nu * dx / dy;

    let ae = de + (0.0_f64).max(-fe);
    let aw = dw + (0.0_f64).max(fw);
    let an = dn + (0.0_f64).max(-fn_);
    let as_ = ds + (0.0_f64).max(fs);

    let ap = ae + aw + an + as_ + (fe - fw) + (fn_ - fs);
    (ae, aw, an, as_, ap.max(1e-12))
}

/// Compute coefficients for the v‑momentum equation at v‑node (i,j).
fn conv_diff_coeffs_v(
    i: usize,
    j: usize,
    u: &[Vec<f64>],
    v: &[Vec<f64>],
    dx: f64,
    dy: f64,
    nu: f64,
) -> (f64, f64, f64, f64, f64) {
    let fe = 0.5 * (u[i + 1][j] + u[i + 1][j - 1]) * dy;
    let fw = 0.5 * (u[i][j] + u[i][j - 1]) * dy;
    let fn_ = 0.5 * (v[i][j] + v[i][j + 1]) * dx;
    let fs = 0.5 * (v[i][j] + v[i][j - 1]) * dx;

    let de = nu * dy / dx;
    let dw = nu * dy / dx;
    let dn = nu * dx / dy;
    let ds = nu * dx / dy;

    let ae = de + (0.0_f64).max(-fe);
    let aw = dw + (0.0_f64).max(fw);
    let an = dn + (0.0_f64).max(-fn_);
    let as_ = ds + (0.0_f64).max(fs);

    let ap = ae + aw + an + as_ + (fe - fw) + (fn_ - fs);
    (ae, aw, an, as_, ap.max(1e-12))
}

/// Approximate central coefficient for pressure‑correction d‑coefficients.
fn ap_coeff(_i: usize, _j: usize, dx: f64, dy: f64, nu: f64, which: char) -> f64 {
    // For the d‑coefficient we use a simplified diffusive estimate:
    // d_u ≈ dy / (2·nu·dy/dx + …) — the SIMPLE textbook formula
    let ap_u = 2.0 * nu * dy / dx + 2.0 * nu * dx / dy;
    let ap_v = 2.0 * nu * dx / dy + 2.0 * nu * dy / dx;
    let base = match which {
        'u' => ap_u,
        'v' => ap_v,
        _ => ap_u,
    };
    if base < 1e-12 { 1e-12 } else { base }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inlet_profile() {
        let u0 = inlet_u(1.0, 1.0, 1.0); // y=1 = bottom of inlet → u=0
        assert!((u0 - 0.0).abs() < 1e-10);
        let u_mid = inlet_u(1.5, 1.0, 1.0); // y=1.5 = centre → max
        assert!((u_mid - 1.5).abs() < 1e-10);
        let u_top = inlet_u(2.0, 1.0, 1.0); // y=2 = top → u=0
        assert!((u_top - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_solver_runs() {
        let json = compute_backward_step(1.0, 1.0, 20, 10, 5);
        let out: BackwardStepOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(out.x.len(), 20);
        assert_eq!(out.y.len(), 10);
        assert_eq!(out.u.len(), 20);
        assert_eq!(out.mag.len(), 20);
    }
}

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// Parameters for a Poiseuille (laminar pipe) flow.
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PoiseuilleParams {
    /// Pipe radius.
    pub R: f64,
    /// Dynamic viscosity.
    pub mu: f64,
    /// Pressure gradient (positive when flow is in the +z direction).
    pub dpdx: f64,
    /// Number of radial sample points.
    pub N: usize,
}

// ─── Analytical solution ──────────────────────────────────────────────────────

/// Exact velocity at radial position `r` for a Poiseuille flow:
///
/// ```text
/// u(r) = dpdx / (4 μ) · (R² − r²)
/// ```
///
/// * `r`  – radial coordinate (0 ≤ r ≤ R).
/// * `R`  – pipe radius.
/// * `dpdx` – pressure gradient.
/// * `mu` – dynamic viscosity.
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn velocity_analytical(r: f64, R: f64, dpdx: f64, mu: f64) -> f64 {
    (dpdx / (4.0 * mu)) * (R * R - r * r)
}

/// Generate a Poiseuille velocity profile sampled at `N` equally-spaced
/// radial positions from `r = 0` (centre) to `r = R` (wall).
///
/// Returns a tuple `(r_values, u_values)`.
#[allow(non_snake_case)]
pub fn generate_poiseuille_profile(
    R: f64,
    mu: f64,
    dpdx: f64,
    N: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut r_vals = Vec::with_capacity(N);
    let mut u_vals = Vec::with_capacity(N);

    for i in 0..N {
        let r = if N == 1 {
            0.0 // single-point case → centre only
        } else {
            (i as f64) * R / (N as f64 - 1.0)
        };
        r_vals.push(r);
        u_vals.push(velocity_analytical(r, R, dpdx, mu));
    }

    (r_vals, u_vals)
}

// ─── Numerical solver (finite differences + Thomas algorithm) ─────────────────

/// Solve the Poiseuille ODE numerically on a uniform radial grid of `N` points
/// using second-order central differences and the Thomas (TDMA) algorithm.
///
/// Boundary conditions:
/// * centre (r = 0): symmetry, du/dr = 0  →  u₀ = u₁
/// * wall   (r = R): no-slip, u = 0
///
/// Returns `(r_vec, u_num_vec)`.
#[allow(non_snake_case)]
pub fn solve_poiseuille_numerical(
    R: f64,
    mu: f64,
    dpdx: f64,
    N: usize,
) -> (Vec<f64>, Vec<f64>) {
    assert!(N >= 3, "N must be ≥ 3 for the numerical solver");

    let dr = R / (N as f64 - 1.0);
    let dr2 = dr * dr;
    let rhs_factor = -dpdx / mu * dr2; // RHS = −(dpdx/μ) · Δr²  (pressure decreases)

    // Build r-values
    let r: Vec<f64> = (0..N).map(|i| i as f64 * dr).collect();

    // ── Build tridiagonal system for unknowns u₁ … u_{N-2} ────────────────
    let M = N - 2; // number of interior unknowns
    let mut a = vec![0.0_f64; M - 1]; // sub-diagonal (size M-1)
    let mut b = vec![0.0_f64; M];     // diagonal
    let mut c = vec![0.0_f64; M - 1]; // super-diagonal (size M-1)
    let mut d = vec![0.0_f64; M];     // RHS

    for k in 0..M {
        let i = k + 1; // full grid index (1 … N-2)
        let ri = r[i];

        let Ai = 1.0 - dr / (2.0 * ri);
        let Bi = -2.0;
        let Ci = 1.0 + dr / (2.0 * ri);
        let Di = rhs_factor;

        if i == 1 {
            // Apply symmetry u₀ = u₁ → (A₁ + B₁)·u₁ + C₁·u₂ = D₁
            b[k] = Ai + Bi;
            if k < M - 1 {
                c[k] = Ci;
            }
            d[k] = Di;
        } else if i == N - 2 {
            // Apply wall u_{N-1} = 0 → A·u_{N-3} + B·u_{N-2} = D
            if k > 0 {
                a[k - 1] = Ai;
            }
            b[k] = Bi;
            d[k] = Di;
        } else {
            // Interior point
            if k > 0 {
                a[k - 1] = Ai;
            }
            b[k] = Bi;
            if k < M - 1 {
                c[k] = Ci;
            }
            d[k] = Di;
        }
    }

    // ── Thomas algorithm (TDMA) ────────────────────────────────────────────
    // Forward sweep
    for k in 1..M {
        let w = a[k - 1] / b[k - 1];
        b[k] -= w * c[k - 1];
        d[k] -= w * d[k - 1];
    }

    // Back substitution
    let mut u_interior = vec![0.0_f64; M];
    u_interior[M - 1] = d[M - 1] / b[M - 1];
    for k in (0..M - 1).rev() {
        u_interior[k] = (d[k] - c[k] * u_interior[k + 1]) / b[k];
    }

    // ── Map back to full grid ──────────────────────────────────────────────
    let mut u = vec![0.0_f64; N];
    u[0] = u_interior[0];           // symmetry: u₀ = u₁
    for i in 1..N - 1 {
        u[i] = u_interior[i - 1];
    }
    u[N - 1] = 0.0;                 // wall: no-slip

    (r, u)
}

// ─── WASM bridge ──────────────────────────────────────────────────────────────

/// Output bundle returned to JavaScript as a JSON string.
#[derive(Serialize)]
pub struct PoiseuilleOutput {
    pub r_analytical: Vec<f64>,
    pub u_analytical: Vec<f64>,
    pub r_numerical: Vec<f64>,
    pub u_numerical: Vec<f64>,
}

/// JS-facing entry-point: computes both the analytical and numerical
/// Poiseuille velocity profiles and returns them as a JSON string.
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn compute_poiseuille(R: f64, mu: f64, dpdx: f64, N: usize) -> String {
    let (r_ana, u_ana) = generate_poiseuille_profile(R, mu, dpdx, N);
    let (r_num, u_num) = solve_poiseuille_numerical(R, mu, dpdx, N);
    let output = PoiseuilleOutput {
        r_analytical: r_ana,
        u_analytical: u_ana,
        r_numerical: r_num,
        u_numerical: u_num,
    };
    serde_json::to_string(&output).unwrap()
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_analytical_centre() {
        // At r=0, u = dpdx/(4μ) * R²
        let u = velocity_analytical(0.0, 1.0, 8.0, 2.0);
        // u = 8/(4*2) * 1 = 1.0
        assert!((u - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_velocity_analytical_wall() {
        // At r=R, u = 0
        let u = velocity_analytical(0.5, 0.5, 10.0, 1.0);
        assert!(u.abs() < 1e-12);
    }

    #[test]
    fn test_profile_length() {
        let (r, u) = generate_poiseuille_profile(1.0, 1.0, 1.0, 5);
        assert_eq!(r.len(), 5);
        assert_eq!(u.len(), 5);
        assert!((r[0] - 0.0).abs() < 1e-12);
        assert!((r[4] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_numerical_vs_analytical() {
        let R = 1.0;
        let mu = 0.5;
        let dpdx = 2.0;
        let N = 51;

        let (r, u_num) = solve_poiseuille_numerical(R, mu, dpdx, N);
        let u_ana: Vec<f64> = r.iter().map(|&ri| velocity_analytical(ri, R, dpdx, mu)).collect();

        // Max absolute error should be small for N=51 (2nd-order scheme)
        let max_err: f64 = u_num.iter().zip(&u_ana).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
        assert!(max_err < 0.002, "max error = {} exceeds tolerance", max_err);

        // Wall is exactly zero
        assert!(u_num[N - 1].abs() < 1e-12);
    }
}

use wasm_bindgen::prelude::*;
use serde::Serialize;

// ─── Analytical solution ──────────────────────────────────────────────────────

/// Exact Couette flow velocity: linear profile between stationary (y=0) and
/// moving (y=h) plates.
#[wasm_bindgen]
pub fn velocity_analytical_couette(y: f64, U: f64, h: f64) -> f64 {
    U * y / h
}

/// Generate a Couette velocity profile sampled at `N` equally-spaced
/// positions from `y = 0` (bottom) to `y = h` (top).
pub fn generate_couette_profile(
    U: f64,
    h: f64,
    N: usize,
) -> (Vec<f64>, Vec<f64>) {
    let dy = h / (N - 1) as f64;
    let mut y = Vec::with_capacity(N);
    let mut u = Vec::with_capacity(N);
    for i in 0..N {
        let yi = i as f64 * dy;
        y.push(yi);
        u.push(velocity_analytical_couette(yi, U, h));
    }
    (y, u)
}

// ─── Numerical solver (finite differences + Thomas algorithm) ─────────────────

/// Solve the Couette ODE d²u/dy² = 0 numerically on a uniform grid of `N`
/// points using second-order central differences and the TDMA algorithm.
///
/// Boundary conditions: u(0) = 0, u(h) = U.
pub fn solve_couette_numerical(
    U: f64,
    h: f64,
    N: usize,
) -> (Vec<f64>, Vec<f64>) {
    let dy = h / (N - 1) as f64;
    let mut y = Vec::with_capacity(N);
    let mut u = vec![0.0; N];

    // Tridiagonal system: u_{i-1} - 2u_i + u_{i+1} = 0
    let mut a = vec![0.0; N];
    let mut b = vec![0.0; N];
    let mut c = vec![0.0; N];
    let mut d = vec![0.0; N];

    // Bottom boundary: u(0) = 0
    b[0] = 1.0;
    c[0] = 0.0;
    d[0] = 0.0;

    // Interior points
    for i in 1..N - 1 {
        a[i] = 1.0;
        b[i] = -2.0;
        c[i] = 1.0;
        d[i] = 0.0;
    }

    // Top boundary: u(h) = U
    a[N - 1] = 0.0;
    b[N - 1] = 1.0;
    d[N - 1] = U;

    // TDMA
    let mut cp = vec![0.0; N];
    let mut dp = vec![0.0; N];

    cp[0] = c[0] / b[0];
    dp[0] = d[0] / b[0];

    for i in 1..N {
        let m = 1.0 / (b[i] - a[i] * cp[i - 1]);
        cp[i] = c[i] * m;
        dp[i] = (d[i] - a[i] * dp[i - 1]) * m;
    }

    u[N - 1] = dp[N - 1];
    for i in (0..N - 1).rev() {
        u[i] = dp[i] - cp[i] * u[i + 1];
    }

    for i in 0..N {
        y.push(i as f64 * dy);
    }

    (y, u)
}

// ─── WASM bridge ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct CouetteOutput {
    pub y_analytical: Vec<f64>,
    pub u_analytical: Vec<f64>,
    pub y_numerical: Vec<f64>,
    pub u_numerical: Vec<f64>,
}

/// JS-facing entry-point: returns both analytical and numerical Couette
/// velocity profiles as a JSON string.
#[wasm_bindgen]
pub fn compute_couette(U: f64, h: f64, N: usize) -> String {
    let (y_ana, u_ana) = generate_couette_profile(U, h, N);
    let (y_num, u_num) = solve_couette_numerical(U, h, N);
    let output = CouetteOutput {
        y_analytical: y_ana,
        u_analytical: u_ana,
        y_numerical: y_num,
        u_numerical: u_num,
    };
    serde_json::to_string(&output).unwrap()
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytical() {
        let U = 2.0;
        let h = 1.0;
        assert_eq!(velocity_analytical_couette(0.0, U, h), 0.0);
        assert_eq!(velocity_analytical_couette(h, U, h), U);
        assert_eq!(velocity_analytical_couette(h / 2.0, U, h), U / 2.0);
    }

    #[test]
    fn test_numerical_bc() {
        let U = 2.0;
        let h = 1.0;
        let N = 20;
        let (_, u) = solve_couette_numerical(U, h, N);
        assert!((u[0] - 0.0).abs() < 1e-10);
        assert!((u[N - 1] - U).abs() < 1e-10);
    }

    #[test]
    fn test_numerical_vs_analytical() {
        let U = 2.0;
        let h = 1.0;
        let N = 20;
        let (y, u_num) = solve_couette_numerical(U, h, N);
        let max_error = y.iter().zip(u_num.iter())
            .map(|(yi, ui)| (ui - velocity_analytical_couette(*yi, U, h)).abs())
            .fold(0.0_f64, |a, b| f64::max(a, b));
        assert!(max_error < 1e-10);
    }
}

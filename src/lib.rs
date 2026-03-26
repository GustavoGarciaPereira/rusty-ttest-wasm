use statrs::distribution::{ContinuousCDF, StudentsT};
use wasm_bindgen::prelude::*;

// ─── Utility functions ────────────────────────────────────────────────────────

fn parse_csv(s: &str) -> Result<Vec<f64>, String> {
    s.split(',')
        .map(|v| {
            v.trim()
                .parse::<f64>()
                .map_err(|e| format!("Parse error: {}", e))
        })
        .collect()
}

/// Sample mean.
fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

/// Sample standard deviation (Bessel's correction: n-1).
fn std_dev(data: &[f64], sample_mean: f64) -> f64 {
    let n = data.len() as f64;
    let variance = data.iter().map(|x| (x - sample_mean).powi(2)).sum::<f64>() / (n - 1.0);
    variance.sqrt()
}

/// Two-tailed p-value: 2 × (1 − CDF(|t|)) under Student's t with `df` degrees of freedom.
fn two_tailed_p_value(t: f64, df: f64) -> f64 {
    let dist = StudentsT::new(0.0, 1.0, df).expect("valid df");
    2.0 * (1.0 - dist.cdf(t.abs()))
}

// ─── One-sample t-test ────────────────────────────────────────────────────────

pub struct OneSampleResultInner {
    pub mean: f64,
    pub std_dev: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub n: usize,
}

/// t = (x̄ − μ) / (s / √n),  df = n − 1
fn one_sample_inner(data: &str, mu: f64) -> Result<OneSampleResultInner, String> {
    let values = parse_csv(data)?;
    let n = values.len();
    if n < 2 {
        return Err("one_sample_t_test requires n ≥ 2".to_string());
    }
    let xbar = mean(&values);
    let s = std_dev(&values, xbar);
    let se = s / (n as f64).sqrt();
    if se == 0.0 {
        return Err("Standard error is zero (all values are identical)".to_string());
    }
    let t = (xbar - mu) / se;
    let df = (n - 1) as f64;
    Ok(OneSampleResultInner { mean: xbar, std_dev: s, t_score: t, p_value: two_tailed_p_value(t, df), n })
}

#[wasm_bindgen]
pub struct OneSampleResult {
    pub mean: f64,
    pub std_dev: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub n: usize,
}

#[wasm_bindgen]
pub fn one_sample_t_test(data: &str, mu: f64) -> Result<OneSampleResult, JsValue> {
    one_sample_inner(data, mu)
        .map(|r| OneSampleResult {
            mean: r.mean, std_dev: r.std_dev, t_score: r.t_score, p_value: r.p_value, n: r.n,
        })
        .map_err(|e| JsValue::from_str(&e))
}

// ─── Independent two-sample t-test (Welch) ───────────────────────────────────

pub struct IndependentResultInner {
    pub mean_a: f64,
    pub mean_b: f64,
    pub std_dev_a: f64,
    pub std_dev_b: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub df: f64,
    pub n_a: usize,
    pub n_b: usize,
}

/// Welch-Satterthwaite df:
/// df = (v1 + v2)² / (v1²/(n1−1) + v2²/(n2−1))   where vi = si²/ni
fn welch_df(sa: f64, na: usize, sb: f64, nb: usize) -> f64 {
    let va = sa * sa / na as f64;
    let vb = sb * sb / nb as f64;
    (va + vb).powi(2) / (va.powi(2) / (na - 1) as f64 + vb.powi(2) / (nb - 1) as f64)
}

/// Welch's t-test: t = (x̄₁ − x̄₂) / √(s₁²/n₁ + s₂²/n₂)
fn independent_inner(group_a: &str, group_b: &str) -> Result<IndependentResultInner, String> {
    let a = parse_csv(group_a)?;
    let b = parse_csv(group_b)?;
    let na = a.len();
    let nb = b.len();
    if na < 2 { return Err("Group A requires n ≥ 2".to_string()); }
    if nb < 2 { return Err("Group B requires n ≥ 2".to_string()); }
    let xa = mean(&a);
    let xb = mean(&b);
    let sa = std_dev(&a, xa);
    let sb = std_dev(&b, xb);
    let se = ((sa * sa / na as f64) + (sb * sb / nb as f64)).sqrt();
    if se == 0.0 {
        return Err("Standard error is zero (both groups have zero variance)".to_string());
    }
    let t = (xa - xb) / se;
    let df = welch_df(sa, na, sb, nb);
    Ok(IndependentResultInner {
        mean_a: xa, mean_b: xb, std_dev_a: sa, std_dev_b: sb,
        t_score: t, p_value: two_tailed_p_value(t, df), df,
        n_a: na, n_b: nb,
    })
}

#[wasm_bindgen]
pub struct IndependentResult {
    pub mean_a: f64,
    pub mean_b: f64,
    pub std_dev_a: f64,
    pub std_dev_b: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub df: f64,
    pub n_a: usize,
    pub n_b: usize,
}

#[wasm_bindgen]
pub fn independent_t_test(group_a: &str, group_b: &str) -> Result<IndependentResult, JsValue> {
    independent_inner(group_a, group_b)
        .map(|r| IndependentResult {
            mean_a: r.mean_a, mean_b: r.mean_b, std_dev_a: r.std_dev_a, std_dev_b: r.std_dev_b,
            t_score: r.t_score, p_value: r.p_value, df: r.df, n_a: r.n_a, n_b: r.n_b,
        })
        .map_err(|e| JsValue::from_str(&e))
}

// ─── Paired t-test ────────────────────────────────────────────────────────────

pub struct PairedResultInner {
    pub mean_before: f64,
    pub mean_after: f64,
    pub mean_diff: f64,
    pub std_dev_diff: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub n: usize,
}

/// Paired t-test: t = d̄ / (s_d / √n),  df = n − 1
fn paired_inner(before: &str, after: &str) -> Result<PairedResultInner, String> {
    let b = parse_csv(before)?;
    let a = parse_csv(after)?;
    if b.len() != a.len() {
        return Err(format!(
            "paired_t_test requires equal-length arrays (before={}, after={})",
            b.len(), a.len()
        ));
    }
    let n = b.len();
    if n < 2 { return Err("paired_t_test requires n ≥ 2".to_string()); }
    let diffs: Vec<f64> = b.iter().zip(a.iter()).map(|(bi, ai)| bi - ai).collect();
    let d_bar = mean(&diffs);
    let sd = std_dev(&diffs, d_bar);
    let se = sd / (n as f64).sqrt();
    if se == 0.0 {
        return Err("Standard error is zero (all differences are identical)".to_string());
    }
    let t = d_bar / se;
    let df = (n - 1) as f64;
    Ok(PairedResultInner {
        mean_before: mean(&b), mean_after: mean(&a),
        mean_diff: d_bar, std_dev_diff: sd, t_score: t, p_value: two_tailed_p_value(t, df), n,
    })
}

#[wasm_bindgen]
pub struct PairedResult {
    pub mean_before: f64,
    pub mean_after: f64,
    pub mean_diff: f64,
    pub std_dev_diff: f64,
    pub t_score: f64,
    pub p_value: f64,
    pub n: usize,
}

#[wasm_bindgen]
pub fn paired_t_test(before: &str, after: &str) -> Result<PairedResult, JsValue> {
    paired_inner(before, after)
        .map(|r| PairedResult {
            mean_before: r.mean_before, mean_after: r.mean_after,
            mean_diff: r.mean_diff, std_dev_diff: r.std_dev_diff,
            t_score: r.t_score, p_value: r.p_value, n: r.n,
        })
        .map_err(|e| JsValue::from_str(&e))
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    // ── parse_csv ──────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_csv_valid() {
        let v = parse_csv("1.0, 2.5, 3.0").unwrap();
        assert_eq!(v, vec![1.0, 2.5, 3.0]);
    }

    #[test]
    fn test_parse_csv_invalid() {
        assert!(parse_csv("1.0, abc, 3.0").is_err());
    }

    // ── One-sample ─────────────────────────────────────────────────────────────

    #[test]
    fn test_one_sample_basic() {
        // data: [2,4,6,8,10], mu=5 → x̄=6, t≈0.7071, df=4
        let r = one_sample_inner("2,4,6,8,10", 5.0).unwrap();
        assert!(approx_eq(r.mean, 6.0, 1e-9));
        assert!(approx_eq(r.t_score, 0.7071067811865476, 1e-6));
        assert_eq!(r.n, 5);
        // p-value must be in (0, 1) and relatively large for this small t
        assert!(r.p_value > 0.0 && r.p_value < 1.0);
        assert!(r.p_value > 0.4); // t≈0.71 with df=4 → p≈0.52
    }

    #[test]
    fn test_one_sample_n1_error() {
        assert!(one_sample_inner("42.0", 0.0).is_err());
    }

    #[test]
    fn test_one_sample_zero_se_error() {
        assert!(one_sample_inner("5,5,5,5", 3.0).is_err());
    }

    // ── Independent (Welch) ────────────────────────────────────────────────────

    #[test]
    fn test_independent_basic() {
        // A=[2,4,6], B=[1,3,5] → t ≈ 0.6124, df=4 (equal variances → Welch gives df=4)
        let r = independent_inner("2,4,6", "1,3,5").unwrap();
        assert!(approx_eq(r.mean_a, 4.0, 1e-9));
        assert!(approx_eq(r.mean_b, 3.0, 1e-9));
        assert!(approx_eq(r.t_score, (3.0_f64 / 8.0_f64).sqrt(), 1e-9));
        assert!(r.p_value > 0.0 && r.p_value < 1.0);
    }

    #[test]
    fn test_independent_group_too_small() {
        assert!(independent_inner("1", "1,2,3").is_err());
        assert!(independent_inner("1,2,3", "1").is_err());
    }

    #[test]
    fn test_welch_df_equal_variance() {
        // When sa=sb and na=nb, Welch df should equal 2*(n-1)
        // sa=sb=2, na=nb=3 → va=vb=4/3
        // df = (8/3)² / (2 * (4/3)²/2) = (64/9) / (16/9) = 4
        let df = welch_df(2.0, 3, 2.0, 3);
        assert!(approx_eq(df, 4.0, 1e-9));
    }

    // ── Paired ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_paired_basic() {
        // before=[10,12,14], after=[9,11,12] → t≈4.0, df=2
        let r = paired_inner("10,12,14", "9,11,12").unwrap();
        assert!(approx_eq(r.mean_diff, 4.0 / 3.0, 1e-9));
        assert!(approx_eq(r.t_score, 4.0, 1e-6));
        // t=4 with df=2 → p≈0.0572 (two-tailed)
        assert!(r.p_value > 0.0 && r.p_value < 0.15);
    }

    #[test]
    fn test_paired_unequal_length_error() {
        assert!(paired_inner("1,2,3", "1,2").is_err());
    }

    #[test]
    fn test_paired_n1_error() {
        assert!(paired_inner("5", "3").is_err());
    }

    // ── p-value direction checks ───────────────────────────────────────────────

    #[test]
    fn test_p_value_large_t_is_small() {
        // Very large t → very small p
        let r = one_sample_inner("100,101,100,101,100,101", 0.0).unwrap();
        assert!(r.p_value < 0.001);
    }

    #[test]
    fn test_p_value_t_near_zero_is_large() {
        // t ≈ 0 → p ≈ 1
        let r = one_sample_inner("1,2,3,4,5", 3.0).unwrap();
        assert!(r.p_value > 0.9);
    }
}

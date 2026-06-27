use wasm_bindgen::prelude::*;
use js_sys::Uint8ClampedArray;
use serde::{Serialize, Deserialize};

/// A point charge in 2D space.
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Charge {
    pub x: f64,
    pub y: f64,
    /// Electric charge (positive or negative).
    pub q: f64,
}

// ─── Color utility ────────────────────────────────────────────────────────────

/// Convert HSV (hue 0–360°, saturation 0–1, value 0–1) to RGB.
/// All three channels are returned in the 0–255 range.
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

// ─── Field solver ─────────────────────────────────────────────────────────────

const SOFTENING: f64 = 0.1;

/// Compute the electric field vector (Ex, Ey) at a single point.
/// Shared helper — all field visualisations use this to avoid code duplication.
fn field_at(x: f64, y: f64, charges: &[Charge], k: f64) -> (f64, f64) {
    let mut ex = 0.0;
    let mut ey = 0.0;
    for charge in charges {
        let dx = x - charge.x;
        let dy = y - charge.y;
        let r = (dx * dx + dy * dy).sqrt().max(SOFTENING);
        let intensity = k * charge.q / (r * r);
        ex += intensity * dx / r;
        ey += intensity * dy / r;
    }
    (ex, ey)
}

/// Core field solver: compute the electric field for a set of charges and
/// return a flat RGBA buffer.  This is the reusable, non‑WASM function.
pub fn compute_field_data(
    width: usize,
    height: usize,
    charges: &[Charge],
    k: f64,
) -> Vec<u8> {
    let mut pixels: Vec<u8> = vec![0u8; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let (ex_total, ey_total) = field_at(x as f64, y as f64, charges, k);

            let angle = ey_total.atan2(ex_total).to_degrees();
            let hue = angle.rem_euclid(360.0);

            let magnitude = (ex_total * ex_total + ey_total * ey_total)
                .sqrt()
                .clamp(0.0, 1.0);
            let brightness = magnitude.sqrt();

            let (r, g, b) = hsv_to_rgb(hue, 1.0, brightness);

            let base = (y * width + x) * 4;
            pixels[base] = r;
            pixels[base + 1] = g;
            pixels[base + 2] = b;
            pixels[base + 3] = 255;
        }
    }

    pixels
}

/// Compute the electric field from a JSON string and return a flat RGBA buffer.
///
/// * `charges_json` – JSON-serialised `Vec<Charge>`.
/// * `k` – Coulomb constant (or an artistic scaling factor).
#[wasm_bindgen]
pub fn generate_field_image(
    width: usize,
    height: usize,
    charges_json: &str,
    k: f64,
) -> Vec<u8> {
    let charges: Vec<Charge> =
        serde_json::from_str(charges_json).expect("Invalid charges JSON");
    compute_field_data(width, height, &charges, k)
}

// ─── WASM bridge (JS → Rust → JS) ────────────────────────────────────────────

/// JS-facing entry-point: computes the field and returns a `Uint8ClampedArray`
/// ready for direct use with a `<canvas>` via `ImageData`.
#[wasm_bindgen]
pub fn compute_electric_field(
    width: usize,
    height: usize,
    charges_json: &str,
    k: f64,
) -> Uint8ClampedArray {
    let data = generate_field_image(width, height, charges_json, k);
    Uint8ClampedArray::from(&data[..])
}

/// Compute arrow data for vector field visualisation.
///
/// Returns a flat `Vec<f64>` where every 4 consecutive values encode one arrow:
/// `[x, y, ex, ey, …]`.  `(x, y)` is the grid-point centre; `(ex, ey)` are the
/// raw electric-field components at that point.  Points that lie within
/// `exclusion_radius` of any charge are skipped to avoid drawing over charges.
///
/// The caller (JS) computes angle via `atan2(ey, ex)` and length via
/// `clamp(sqrt(ex²+ey²) * scale, 0, maxLen)` — keeping visual tuning on the
/// front-end side.
#[wasm_bindgen]
pub fn compute_arrows(
    width: usize,
    height: usize,
    charges_json: &str,
    k: f64,
    grid_spacing: usize,
) -> Result<Vec<f64>, JsValue> {
    let charges: Vec<Charge> = serde_json::from_str(charges_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid charges JSON: {}", e)))?;

    let spacing = grid_spacing.max(10);
    let exclusion_radius = (spacing as f64 * 0.375).max(12.0);

    let mut data: Vec<f64> = Vec::new();

    let mut y = spacing as f64 / 2.0;
    while y < height as f64 {
        let mut x = spacing as f64 / 2.0;
        while x < width as f64 {
            // Skip points too close to any charge (singularity avoidance)
            let mut too_close = false;
            for charge in &charges {
                let dx = x - charge.x;
                let dy = y - charge.y;
                if (dx * dx + dy * dy).sqrt() < exclusion_radius {
                    too_close = true;
                    break;
                }
            }

            if !too_close {
                let (ex, ey) = field_at(x, y, &charges, k);
                data.push(x);
                data.push(y);
                data.push(ex);
                data.push(ey);
            }

            x += spacing as f64;
        }
        y += spacing as f64;
    }

    Ok(data)
}

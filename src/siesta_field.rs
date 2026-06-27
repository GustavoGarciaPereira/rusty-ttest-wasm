use wasm_bindgen::prelude::*;
use crate::simulation::{Charge, compute_field_data};

// ─── SIESTA .out parser ───────────────────────────────────────────────────────

/// Parse a SIESTA `.out` file and extract atomic coordinates and Mulliken
/// charges.  Returns a `Vec<Charge>` ready for the field solver.
pub fn parse_siesta_out(content: &str) -> Result<Vec<Charge>, String> {
    let mut charges: Vec<Charge> = Vec::new();

    // ── 1. Extract atomic coordinates ─────────────────────────────────────
    let mut in_coords = false;
    for line in content.lines() {
        if line.contains("siesta: Atomic coordinates (Ang):") {
            in_coords = true;
            continue;
        }
        if in_coords {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("siesta:") {
                break;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 5 {
                let x = parts[2].parse::<f64>().unwrap_or(0.0);
                let y = parts[3].parse::<f64>().unwrap_or(0.0);
                // z is ignored (2D field)
                charges.push(Charge { x, y, q: 0.0 });
            }
        }
    }

    if charges.is_empty() {
        return Err("Nenhuma coordenada atômica encontrada no arquivo.".to_string());
    }

    // ── 2. Extract Mulliken populations ────────────────────────────────────
    let mut mulliken_charges: Vec<f64> = Vec::new();
    let mut in_mulliken = false;
    for line in content.lines() {
        if line.contains("siesta: Mulliken populations:") {
            in_mulliken = true;
            continue;
        }
        if in_mulliken {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("siesta:") {
                break;
            }
            // Format: "Species: O  #1  Q = -0.4321"
            if let Some(q_start) = trimmed.find("Q =") {
                let q_str = trimmed[q_start + 3..].trim();
                if let Ok(q) = q_str.parse::<f64>() {
                    mulliken_charges.push(q);
                }
            }
        }
    }

    // ── 3. Assign charges ──────────────────────────────────────────────────
    if mulliken_charges.len() == charges.len() {
        for (i, ch) in charges.iter_mut().enumerate() {
            ch.q = mulliken_charges[i];
        }
    }
    // else: charges stay at 0.0 (fallback — JS will warn)

    Ok(charges)
}

/// Parse a SIESTA `.out` file and return a JSON array of `{x, y, z, q, sym}`.
#[wasm_bindgen]
pub fn parse_siesta_out_full(content: &str) -> String {
    #[derive(serde::Serialize)]
    struct Atom { x: f64, y: f64, z: f64, q: f64, sym: String }

    let mut atoms: Vec<Atom> = Vec::new();

    let mut in_coords = false;
    for line in content.lines() {
        if line.contains("siesta: Atomic coordinates (Ang):") { in_coords = true; continue; }
        if in_coords {
            let t = line.trim();
            if t.is_empty() || t.starts_with("siesta:") { break; }
            let parts: Vec<&str> = t.split_whitespace().collect();
            if parts.len() >= 5 {
                let sym = parts[1].to_string();
                let x = parts[2].parse::<f64>().unwrap_or(0.0);
                let y = parts[3].parse::<f64>().unwrap_or(0.0);
                let z = parts[4].parse::<f64>().unwrap_or(0.0);
                atoms.push(Atom { x, y, z, q: 0.0, sym });
            }
        }
    }

    // Mulliken
    let mut mulliken: Vec<f64> = Vec::new();
    let mut in_mull = false;
    for line in content.lines() {
        if line.contains("siesta: Mulliken populations:") { in_mull = true; continue; }
        if in_mull {
            let t = line.trim();
            if t.is_empty() || t.starts_with("siesta:") { break; }
            if let Some(qs) = t.find("Q =") {
                if let Ok(q) = t[qs+3..].trim().parse::<f64>() { mulliken.push(q); }
            }
        }
    }

    if mulliken.len() == atoms.len() {
        for (i, a) in atoms.iter_mut().enumerate() { a.q = mulliken[i]; }
    }

    serde_json::to_string(&atoms).unwrap()
}

// ─── WASM bridge ──────────────────────────────────────────────────────────────

/// Parse a SIESTA `.out` file, transform atomic coordinates to pixel space,
/// compute the electric field, and return a flat RGBA buffer.
#[wasm_bindgen]
pub fn generate_field_from_siesta(
    out_content: &str,
    width: usize,
    height: usize,
    scale: f64,
    k: f64,
) -> Vec<u8> {
    let mut charges = match parse_siesta_out(out_content) {
        Ok(c) => c,
        Err(_) => return vec![0u8; width * height * 4],
    };

    if charges.is_empty() {
        return vec![0u8; width * height * 4];
    }

    // ── Compute bounding box ───────────────────────────────────────────────
    let min_x = charges.iter().map(|c| c.x).fold(f64::INFINITY, f64::min);
    let max_x = charges.iter().map(|c| c.x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = charges.iter().map(|c| c.y).fold(f64::INFINITY, f64::min);
    let max_y = charges.iter().map(|c| c.y).fold(f64::NEG_INFINITY, f64::max);

    let range_x = max_x - min_x;
    let range_y = max_y - min_y;
    let range_max = range_x.max(range_y).max(0.1); // avoid division by zero

    // Auto‑scale to fit within 80 % of the canvas
    let auto_scale = (width.min(height) as f64 * 0.8) / range_max;
    let effective_scale = if scale > 0.0 { scale } else { auto_scale };

    // Centre the molecule in the canvas
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;

    for ch in &mut charges {
        // Map physical x → canvas x (left‑to‑right)
        ch.x = (ch.x - min_x) * effective_scale + (cx - range_x * effective_scale / 2.0);
        // Map physical y → canvas y (invert: SIESTA y goes up, canvas y goes down)
        ch.y = (max_y - ch.y) * effective_scale + (cy - range_y * effective_scale / 2.0);
    }

    compute_field_data(width, height, &charges, k)
}

// ─── 3D field solver ─────────────────────────────────────────────────────────

/// A 3D point charge (used internally for the 3D solver).
#[derive(Clone, Copy)]
struct Charge3D {
    x: f64, y: f64, z: f64, q: f64,
}

fn field_at(p: (f64, f64, f64), charges: &[Charge3D], k: f64) -> (f64, f64, f64) {
    let mut ex = 0.0; let mut ey = 0.0; let mut ez = 0.0;
    let soft = 0.1; // softening to avoid singularity
    for c in charges {
        let dx = p.0 - c.x; let dy = p.1 - c.y; let dz = p.2 - c.z;
        let r = (dx*dx + dy*dy + dz*dz).sqrt().max(soft);
        let intensity = k * c.q / (r * r);
        ex += intensity * dx / r;
        ey += intensity * dy / r;
        ez += intensity * dz / r;
    }
    (ex, ey, ez)
}

/// Compute the 3D electric field on a uniform grid. Returns a flat array:
/// `[x0,y0,z0,Ex0,Ey0,Ez0,  x1,y1,z1,Ex1,Ey1,Ez1,  ...]`.
#[wasm_bindgen]
pub fn compute_field_3d(
    charges_json: &str,
    nx: usize, ny: usize, nz: usize,
    k: f64,
) -> Vec<f64> {
    let charges: Vec<Charge> =
        serde_json::from_str(charges_json).unwrap_or_default();
    let c3d: Vec<Charge3D> = charges.iter()
        .map(|c| Charge3D { x: c.x, y: c.y, z: 0.0, q: c.q })
        .collect();

    // Use bounding box of charges to define grid extent
    let min_x = c3d.iter().map(|c| c.x).fold(f64::INFINITY, f64::min);
    let max_x = c3d.iter().map(|c| c.x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = c3d.iter().map(|c| c.y).fold(f64::INFINITY, f64::min);
    let max_y = c3d.iter().map(|c| c.y).fold(f64::NEG_INFINITY, f64::max);
    let min_z = c3d.iter().map(|c| c.z).fold(f64::INFINITY, f64::min);
    let max_z = c3d.iter().map(|c| c.z).fold(f64::NEG_INFINITY, f64::max);

    let pad = 2.0; // padding in Angstroms
    let x0 = min_x - pad; let x1 = max_x + pad;
    let y0 = min_y - pad; let y1 = max_y + pad;
    let z0 = min_z - pad; let z1 = max_z + pad;

    let mut out = Vec::with_capacity(nx * ny * nz * 6);
    for iz in 0..nz {
        let z = z0 + (iz as f64 / (nz.max(1) - 1) as f64) * (z1 - z0);
        for iy in 0..ny {
            let y = y0 + (iy as f64 / (ny.max(1) - 1) as f64) * (y1 - y0);
            for ix in 0..nx {
                let x = x0 + (ix as f64 / (nx.max(1) - 1) as f64) * (x1 - x0);
                let (ex, ey, ez) = field_at((x, y, z), &c3d, k);
                out.extend_from_slice(&[x, y, z, ex, ey, ez]);
            }
        }
    }
    out
}

/// Trace field lines from positive charges using RK4 integration.
/// Returns a flat array of 3D points `[x0,y0,z0, x1,y1,z1, ...]`.
/// Each line is terminated by a `[f64::NAN; 3]` sentinel.
#[wasm_bindgen]
pub fn trace_field_lines(
    charges_json: &str,
    step_size: f64,
    max_steps: usize,
    k: f64,
) -> Vec<f64> {
    let charges: Vec<Charge> =
        serde_json::from_str(charges_json).unwrap_or_default();
    let c3d: Vec<Charge3D> = charges.iter()
        .map(|c| Charge3D { x: c.x, y: c.y, z: 0.0, q: c.q })
        .collect();

    let mut out: Vec<f64> = Vec::new();

    for c in &c3d {
        if c.q <= 0.0 { continue; } // start lines only from positive charges

        // Start slightly offset from the charge centre in 6 directions
        let offsets = [
            (step_size, 0.0, 0.0), (-step_size, 0.0, 0.0),
            (0.0, step_size, 0.0), (0.0, -step_size, 0.0),
            (0.0, 0.0, step_size), (0.0, 0.0, -step_size),
        ];

        for &(ox, oy, oz) in &offsets {
            let mut x = c.x + ox;
            let mut y = c.y + oy;
            let mut z = c.z + oz;
            out.push(x); out.push(y); out.push(z);

            for _ in 0..max_steps {
                let (ex, ey, ez) = field_at((x, y, z), &c3d, k);
                let mag = (ex*ex + ey*ey + ez*ez).sqrt();
                if mag < 1e-9 { break; }

                // RK4 step
                let dt = step_size / mag;
                let k1 = (ex, ey, ez);
                let (e2x, e2y, e2z) = field_at(
                    (x + 0.5*dt*k1.0, y + 0.5*dt*k1.1, z + 0.5*dt*k1.2), &c3d, k);
                let (e3x, e3y, e3z) = field_at(
                    (x + 0.5*dt*e2x, y + 0.5*dt*e2y, z + 0.5*dt*e2z), &c3d, k);
                let (e4x, e4y, e4z) = field_at(
                    (x + dt*e3x, y + dt*e3y, z + dt*e3z), &c3d, k);

                x += dt / 6.0 * (k1.0 + 2.0*e2x + 2.0*e3x + e4x);
                y += dt / 6.0 * (k1.1 + 2.0*e2y + 2.0*e3y + e4y);
                z += dt / 6.0 * (k1.2 + 2.0*e2z + 2.0*e3z + e4z);

                out.push(x); out.push(y); out.push(z);

                // Stop if field becomes too weak or we hit a negative charge
                let (efx, efy, efz) = field_at((x, y, z), &c3d, k);
                if (efx*efx + efy*efy + efz*efz).sqrt() < 0.01 { break; }

                // Check proximity to any negative charge
                let mut stop = false;
                for cn in &c3d {
                    if cn.q >= 0.0 { continue; }
                    let d2 = (x-cn.x)*(x-cn.x) + (y-cn.y)*(y-cn.y) + (z-cn.z)*(z-cn.z);
                    if d2 < step_size * step_size { stop = true; break; }
                }
                if stop { break; }
            }
            // Sentinel to separate lines
            out.push(f64::NAN); out.push(f64::NAN); out.push(f64::NAN);
        }
    }
    out
}

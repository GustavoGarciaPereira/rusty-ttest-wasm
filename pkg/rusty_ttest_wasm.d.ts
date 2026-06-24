/* tslint:disable */
/* eslint-disable */

/**
 * A point charge in 2D space.
 */
export class Charge {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Electric charge (positive or negative).
     */
    q: number;
    x: number;
    y: number;
}

export class IndependentResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    df: number;
    mean_a: number;
    mean_b: number;
    n_a: number;
    n_b: number;
    p_value: number;
    std_dev_a: number;
    std_dev_b: number;
    t_score: number;
}

export class OneSampleResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    mean: number;
    n: number;
    p_value: number;
    std_dev: number;
    t_score: number;
}

export class PairedResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    mean_after: number;
    mean_before: number;
    mean_diff: number;
    n: number;
    p_value: number;
    std_dev_diff: number;
    t_score: number;
}

/**
 * Parameters for a Poiseuille (laminar pipe) flow.
 */
export class PoiseuilleParams {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Number of radial sample points.
     */
    N: number;
    /**
     * Pipe radius.
     */
    R: number;
    /**
     * Pressure gradient (positive when flow is in the +z direction).
     */
    dpdx: number;
    /**
     * Dynamic viscosity.
     */
    mu: number;
}

/**
 * Run the 2D SIMPLE solver for a backward‑facing step and return all
 * fields as JSON.
 */
export function compute_backward_step(U_in: number, h_in: number, nx: number, ny: number, max_iter: number): string;

/**
 * JS-facing entry-point: returns both analytical and numerical Couette
 * velocity profiles as a JSON string.
 */
export function compute_couette(U: number, h: number, N: number): string;

/**
 * JS-facing entry-point: computes the field and returns a `Uint8ClampedArray`
 * ready for direct use with a `<canvas>` via `ImageData`.
 */
export function compute_electric_field(width: number, height: number, charges_json: string, k: number): Uint8ClampedArray;

/**
 * JS-facing entry-point: computes both the analytical and numerical
 * Poiseuille velocity profiles and returns them as a JSON string.
 */
export function compute_poiseuille(R: number, mu: number, dpdx: number, N: number): string;

/**
 * Compute the electric field produced by a set of point charges across a
 * `width × height` pixel grid and return a flat RGBA byte buffer.
 *
 * * `charges_json` – JSON-serialised `Vec<Charge>`.
 * * `k` – Coulomb constant (or an artistic scaling factor).
 */
export function generate_field_image(width: number, height: number, charges_json: string, k: number): Uint8Array;

export function independent_t_test(group_a: string, group_b: string): IndependentResult;

/**
 * Return a pair `(u, v)` from pre‑computed arrays (called from JS).
 */
export function interpolate_velocity(x: number, y: number, xs: Float64Array, ys: Float64Array, u_flat: Float64Array, v_flat: Float64Array, nx: number, ny: number): Float64Array;

export function one_sample_t_test(data: string, mu: number): OneSampleResult;

export function paired_t_test(before: string, after: string): PairedResult;

/**
 * Exact velocity at radial position `r` for a Poiseuille flow:
 *
 * ```text
 * u(r) = dpdx / (4 μ) · (R² − r²)
 * ```
 *
 * * `r`  – radial coordinate (0 ≤ r ≤ R).
 * * `R`  – pipe radius.
 * * `dpdx` – pressure gradient.
 * * `mu` – dynamic viscosity.
 */
export function velocity_analytical(r: number, R: number, dpdx: number, mu: number): number;

/**
 * Exact Couette flow velocity: linear profile between stationary (y=0) and
 * moving (y=h) plates.
 */
export function velocity_analytical_couette(y: number, U: number, h: number): number;

/**
 * Return (u, v) at a physical point by bilinear interpolation from
 * the staggered‑grid fields. Assumes the fields have been computed.
 */
export function velocity_at(_x: number, y: number, U_in: number, h_in: number): Float64Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_get_independentresult_df: (a: number) => number;
    readonly __wbg_get_independentresult_mean_a: (a: number) => number;
    readonly __wbg_get_independentresult_mean_b: (a: number) => number;
    readonly __wbg_get_independentresult_n_a: (a: number) => number;
    readonly __wbg_get_independentresult_n_b: (a: number) => number;
    readonly __wbg_get_independentresult_p_value: (a: number) => number;
    readonly __wbg_get_independentresult_std_dev_a: (a: number) => number;
    readonly __wbg_get_independentresult_std_dev_b: (a: number) => number;
    readonly __wbg_get_independentresult_t_score: (a: number) => number;
    readonly __wbg_get_onesampleresult_n: (a: number) => number;
    readonly __wbg_get_pairedresult_n: (a: number) => number;
    readonly __wbg_independentresult_free: (a: number, b: number) => void;
    readonly __wbg_onesampleresult_free: (a: number, b: number) => void;
    readonly __wbg_pairedresult_free: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_df: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_mean_a: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_mean_b: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_n_a: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_n_b: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_p_value: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_std_dev_a: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_std_dev_b: (a: number, b: number) => void;
    readonly __wbg_set_independentresult_t_score: (a: number, b: number) => void;
    readonly __wbg_set_onesampleresult_n: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_n: (a: number, b: number) => void;
    readonly independent_t_test: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly one_sample_t_test: (a: number, b: number, c: number) => [number, number, number];
    readonly paired_t_test: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly __wbg_set_onesampleresult_mean: (a: number, b: number) => void;
    readonly __wbg_set_onesampleresult_p_value: (a: number, b: number) => void;
    readonly __wbg_set_onesampleresult_std_dev: (a: number, b: number) => void;
    readonly __wbg_set_onesampleresult_t_score: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_mean_after: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_mean_before: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_mean_diff: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_p_value: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_std_dev_diff: (a: number, b: number) => void;
    readonly __wbg_set_pairedresult_t_score: (a: number, b: number) => void;
    readonly __wbg_get_onesampleresult_mean: (a: number) => number;
    readonly __wbg_get_onesampleresult_p_value: (a: number) => number;
    readonly __wbg_get_onesampleresult_std_dev: (a: number) => number;
    readonly __wbg_get_onesampleresult_t_score: (a: number) => number;
    readonly __wbg_get_pairedresult_mean_after: (a: number) => number;
    readonly __wbg_get_pairedresult_mean_before: (a: number) => number;
    readonly __wbg_get_pairedresult_mean_diff: (a: number) => number;
    readonly __wbg_get_pairedresult_p_value: (a: number) => number;
    readonly __wbg_get_pairedresult_std_dev_diff: (a: number) => number;
    readonly __wbg_get_pairedresult_t_score: (a: number) => number;
    readonly compute_backward_step: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly interpolate_velocity: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => [number, number];
    readonly velocity_at: (a: number, b: number, c: number, d: number) => [number, number];
    readonly __wbg_charge_free: (a: number, b: number) => void;
    readonly __wbg_get_charge_q: (a: number) => number;
    readonly __wbg_get_charge_x: (a: number) => number;
    readonly __wbg_get_charge_y: (a: number) => number;
    readonly __wbg_set_charge_q: (a: number, b: number) => void;
    readonly __wbg_set_charge_x: (a: number, b: number) => void;
    readonly __wbg_set_charge_y: (a: number, b: number) => void;
    readonly compute_electric_field: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly generate_field_image: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly compute_couette: (a: number, b: number, c: number) => [number, number];
    readonly velocity_analytical_couette: (a: number, b: number, c: number) => number;
    readonly __wbg_get_poiseuilleparams_N: (a: number) => number;
    readonly __wbg_get_poiseuilleparams_R: (a: number) => number;
    readonly __wbg_get_poiseuilleparams_dpdx: (a: number) => number;
    readonly __wbg_get_poiseuilleparams_mu: (a: number) => number;
    readonly __wbg_poiseuilleparams_free: (a: number, b: number) => void;
    readonly __wbg_set_poiseuilleparams_N: (a: number, b: number) => void;
    readonly __wbg_set_poiseuilleparams_R: (a: number, b: number) => void;
    readonly __wbg_set_poiseuilleparams_dpdx: (a: number, b: number) => void;
    readonly __wbg_set_poiseuilleparams_mu: (a: number, b: number) => void;
    readonly compute_poiseuille: (a: number, b: number, c: number, d: number) => [number, number];
    readonly velocity_analytical: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

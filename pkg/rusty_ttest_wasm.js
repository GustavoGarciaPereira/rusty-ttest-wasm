/* @ts-self-types="./rusty_ttest_wasm.d.ts" */

/**
 * A point charge in 2D space.
 */
export class Charge {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChargeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_charge_free(ptr, 0);
    }
    /**
     * Electric charge (positive or negative).
     * @returns {number}
     */
    get q() {
        const ret = wasm.__wbg_get_charge_q(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_charge_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_charge_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * Electric charge (positive or negative).
     * @param {number} arg0
     */
    set q(arg0) {
        wasm.__wbg_set_charge_q(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_charge_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_charge_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Charge.prototype[Symbol.dispose] = Charge.prototype.free;

export class IndependentResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IndependentResult.prototype);
        obj.__wbg_ptr = ptr;
        IndependentResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IndependentResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_independentresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get df() {
        const ret = wasm.__wbg_get_independentresult_df(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mean_a() {
        const ret = wasm.__wbg_get_independentresult_mean_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mean_b() {
        const ret = wasm.__wbg_get_independentresult_mean_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get n_a() {
        const ret = wasm.__wbg_get_independentresult_n_a(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get n_b() {
        const ret = wasm.__wbg_get_independentresult_n_b(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get p_value() {
        const ret = wasm.__wbg_get_independentresult_p_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get std_dev_a() {
        const ret = wasm.__wbg_get_independentresult_std_dev_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get std_dev_b() {
        const ret = wasm.__wbg_get_independentresult_std_dev_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get t_score() {
        const ret = wasm.__wbg_get_independentresult_t_score(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set df(arg0) {
        wasm.__wbg_set_independentresult_df(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mean_a(arg0) {
        wasm.__wbg_set_independentresult_mean_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mean_b(arg0) {
        wasm.__wbg_set_independentresult_mean_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n_a(arg0) {
        wasm.__wbg_set_independentresult_n_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n_b(arg0) {
        wasm.__wbg_set_independentresult_n_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set p_value(arg0) {
        wasm.__wbg_set_independentresult_p_value(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set std_dev_a(arg0) {
        wasm.__wbg_set_independentresult_std_dev_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set std_dev_b(arg0) {
        wasm.__wbg_set_independentresult_std_dev_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set t_score(arg0) {
        wasm.__wbg_set_independentresult_t_score(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) IndependentResult.prototype[Symbol.dispose] = IndependentResult.prototype.free;

export class OneSampleResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(OneSampleResult.prototype);
        obj.__wbg_ptr = ptr;
        OneSampleResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        OneSampleResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_onesampleresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get mean() {
        const ret = wasm.__wbg_get_onesampleresult_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get n() {
        const ret = wasm.__wbg_get_onesampleresult_n(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get p_value() {
        const ret = wasm.__wbg_get_onesampleresult_p_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get std_dev() {
        const ret = wasm.__wbg_get_onesampleresult_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get t_score() {
        const ret = wasm.__wbg_get_onesampleresult_t_score(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set mean(arg0) {
        wasm.__wbg_set_onesampleresult_mean(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n(arg0) {
        wasm.__wbg_set_onesampleresult_n(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set p_value(arg0) {
        wasm.__wbg_set_onesampleresult_p_value(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set std_dev(arg0) {
        wasm.__wbg_set_onesampleresult_std_dev(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set t_score(arg0) {
        wasm.__wbg_set_onesampleresult_t_score(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) OneSampleResult.prototype[Symbol.dispose] = OneSampleResult.prototype.free;

export class PairedResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PairedResult.prototype);
        obj.__wbg_ptr = ptr;
        PairedResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PairedResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pairedresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get mean_after() {
        const ret = wasm.__wbg_get_pairedresult_mean_after(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mean_before() {
        const ret = wasm.__wbg_get_pairedresult_mean_before(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mean_diff() {
        const ret = wasm.__wbg_get_pairedresult_mean_diff(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get n() {
        const ret = wasm.__wbg_get_pairedresult_n(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get p_value() {
        const ret = wasm.__wbg_get_pairedresult_p_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get std_dev_diff() {
        const ret = wasm.__wbg_get_pairedresult_std_dev_diff(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get t_score() {
        const ret = wasm.__wbg_get_pairedresult_t_score(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set mean_after(arg0) {
        wasm.__wbg_set_pairedresult_mean_after(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mean_before(arg0) {
        wasm.__wbg_set_pairedresult_mean_before(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mean_diff(arg0) {
        wasm.__wbg_set_pairedresult_mean_diff(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n(arg0) {
        wasm.__wbg_set_pairedresult_n(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set p_value(arg0) {
        wasm.__wbg_set_pairedresult_p_value(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set std_dev_diff(arg0) {
        wasm.__wbg_set_pairedresult_std_dev_diff(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set t_score(arg0) {
        wasm.__wbg_set_pairedresult_t_score(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) PairedResult.prototype[Symbol.dispose] = PairedResult.prototype.free;

/**
 * JS-facing entry-point: computes the field and returns a `Uint8ClampedArray`
 * ready for direct use with a `<canvas>` via `ImageData`.
 * @param {number} width
 * @param {number} height
 * @param {string} charges_json
 * @param {number} k
 * @returns {Uint8ClampedArray}
 */
export function compute_electric_field(width, height, charges_json, k) {
    const ptr0 = passStringToWasm0(charges_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compute_electric_field(width, height, ptr0, len0, k);
    return ret;
}

/**
 * Compute the electric field produced by a set of point charges across a
 * `width × height` pixel grid and return a flat RGBA byte buffer.
 *
 * * `charges_json` – JSON-serialised `Vec<Charge>`.
 * * `k` – Coulomb constant (or an artistic scaling factor).
 * @param {number} width
 * @param {number} height
 * @param {string} charges_json
 * @param {number} k
 * @returns {Uint8Array}
 */
export function generate_field_image(width, height, charges_json, k) {
    const ptr0 = passStringToWasm0(charges_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.generate_field_image(width, height, ptr0, len0, k);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {string} group_a
 * @param {string} group_b
 * @returns {IndependentResult}
 */
export function independent_t_test(group_a, group_b) {
    const ptr0 = passStringToWasm0(group_a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(group_b, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.independent_t_test(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return IndependentResult.__wrap(ret[0]);
}

/**
 * @param {string} data
 * @param {number} mu
 * @returns {OneSampleResult}
 */
export function one_sample_t_test(data, mu) {
    const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.one_sample_t_test(ptr0, len0, mu);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return OneSampleResult.__wrap(ret[0]);
}

/**
 * @param {string} before
 * @param {string} after
 * @returns {PairedResult}
 */
export function paired_t_test(before, after) {
    const ptr0 = passStringToWasm0(before, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(after, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.paired_t_test(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PairedResult.__wrap(ret[0]);
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_from_slice_d16553cb630d3573: function(arg0, arg1) {
            const ret = new Uint8ClampedArray(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./rusty_ttest_wasm_bg.js": import0,
    };
}

const ChargeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_charge_free(ptr >>> 0, 1));
const IndependentResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_independentresult_free(ptr >>> 0, 1));
const OneSampleResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_onesampleresult_free(ptr >>> 0, 1));
const PairedResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pairedresult_free(ptr >>> 0, 1));

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('rusty_ttest_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };

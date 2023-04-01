
function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    (new Uint32Array(wasm.memory.buffer)).set(arg, ptr / 4);
    const WASM_VECTOR_LEN = arg.length;
    return {ptr, WASM_VECTOR_LEN};
}
/**
* @param {Uint32Array} elevation
* @returns {number}
*/
function trap(wasm, elevation) {
    const {ptr0, len0} = passArray32ToWasm0(elevation, wasm.__wbindgen_malloc);
    const ret = wasm.trap(ptr0, len0);
    return ret >>> 0;
}


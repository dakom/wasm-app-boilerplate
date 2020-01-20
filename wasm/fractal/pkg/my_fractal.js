(function() {
    const __exports = {};
    let wasm;

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    let WASM_VECTOR_LEN = 0;

    function passArray8ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 1);
        getUint8Memory0().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }
    /**
    * @param {Uint8Array} pixels
    * @param {Uint8Array} palette
    * @param {number} width
    * @param {number} height
    * @param {number} max_iterations
    */
    __exports.update_pixels = function(pixels, palette, width, height, max_iterations) {
        try {
            var ptr0 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passArray8ToWasm0(palette, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.update_pixels(ptr0, len0, ptr1, len1, width, height, max_iterations);
        } finally {
            pixels.set(getUint8Memory0().subarray(ptr0 / 1, ptr0 / 1 + len0));
            wasm.__wbindgen_free(ptr0, len0 * 1);
        }
    };

    /**
    * @param {Uint8Array} palette
    */
    __exports.cycle_palette = function(palette) {
        try {
            var ptr0 = passArray8ToWasm0(palette, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.cycle_palette(ptr0, len0);
        } finally {
            palette.set(getUint8Memory0().subarray(ptr0 / 1, ptr0 / 1 + len0));
            wasm.__wbindgen_free(ptr0, len0 * 1);
        }
    };

    function init(module) {
        if (typeof module === 'undefined') {
            let src;
            if (self.document === undefined) {
                src = self.location.href;
            } else {
                src = self.document.currentScript.src;
            }
            module = src.replace(/\.js$/, '_bg.wasm');
        }
        let result;
        const imports = {};

        if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

            const response = fetch(module);
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                result = WebAssembly.instantiateStreaming(response, imports)
                .catch(e => {
                    return response
                    .then(r => {
                        if (r.headers.get('Content-Type') != 'application/wasm') {
                            console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                            return r.arrayBuffer();
                        } else {
                            throw e;
                        }
                    })
                    .then(bytes => WebAssembly.instantiate(bytes, imports));
                });
            } else {
                result = response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            }
        } else {

            result = WebAssembly.instantiate(module, imports)
            .then(result => {
                if (result instanceof WebAssembly.Instance) {
                    return { instance: result, module };
                } else {
                    return result;
                }
            });
        }
        return result.then(({instance, module}) => {
            wasm = instance.exports;
            init.__wbindgen_wasm_module = module;

            return wasm;
        });
    }

    self.wasm_fractal = Object.assign(init, __exports);

})();

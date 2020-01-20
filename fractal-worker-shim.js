/**
 * workers-as-modules is not supported in most browsers so far
 * and the default wasm_bindgen is to use proper es modules
 * so this little shim essentially allows loading the wasm as a worker
 * see the --no-modules flag docs at https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
 * also note that it's build with --no-modules-global-var wasm_core to avoid global namespace collisions
 */

self.importScripts("./wasm/fractal/pkg/my_fractal.js");
const {update_pixels, cycle_palette} = wasm_fractal;

let maxIterations;
let dataBuf;
let palette;

(async () => { 
    await wasm_fractal("./wasm/fractal/pkg/my_fractal_bg.wasm");

    

    /**
     *  Wasm is ready, now setup communication with the main thread
     *  only 2 types of events are processed:
     *  1. READY (for setup)
     *  2. EVENT (for events sent from the main thread)
     */
    self.onmessage = msg => {
        if(msg.data) {
            if(msg.data.type === "START") {
                maxIterations = msg.data.maxIterations;
            }
            else if(msg.data.type === "UPDATE") {
                dataBuf = msg.data.dataBuf;
                paletteBuf = msg.data.paletteBuf;
                const img = new Uint8ClampedArray(dataBuf);
                const palette = new Uint8ClampedArray(paletteBuf);
                update_pixels(img, palette, msg.data.width, msg.data.height, maxIterations);
                cycle_palette(palette);

                self.postMessage({
                    type: "DRAW",
                    dataBuf,
                    paletteBuf,
                }, [dataBuf, paletteBuf]);
            }
        }
    };

    //tell the main thread we're ready
    self.postMessage({
        type: "READY"
    });
})();
/**
 * workers-as-modules is not supported in most browsers so far
 * and the default wasm_bindgen is to use proper es modules
 * so this little shim essentially allows loading the wasm as a worker
 * see the --no-modules flag docs at https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
 * also note that it's build with --no-modules-global-var wasm_core to avoid global namespace collisions
 */

self.importScripts("./wasm/fractal/pkg/my_fractal.js");
const {update_pixels} = wasm_fractal;

const MAX_ITERATIONS = 30;

let palettes = [];
let dataBuf;

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
                for (let i = palettes.length; i <= MAX_ITERATIONS; i++) {
                    palettes.push(Math.floor(Math.random() * 0xFF));
                    palettes.push(Math.floor(Math.random() * 0xFF));
                    palettes.push(Math.floor(Math.random() * 0xFF));
                    palettes.push(0xFF);
                }
            }
            else if(msg.data.type === "UPDATE") {
                dataBuf = msg.data.dataBuf;
                let imgBuf = new Uint8ClampedArray(dataBuf);
                imgBuf.fill(0);
                update_pixels(imgBuf, palettes, msg.data.width, msg.data.height, MAX_ITERATIONS);
                palettes.unshift(palettes.pop());

                //console.log(palettes.reduce((curr, acc) => curr.concat(acc)));

                self.postMessage({
                    type: "DRAW",
                    dataBuf,
                }, [dataBuf]);
            }
        }
    };

    //tell the main thread we're ready
    self.postMessage({
        type: "READY"
    });
})();
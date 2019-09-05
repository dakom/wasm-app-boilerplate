/**
 * workers-as-modules is not supported in most browsers so far
 * and the default wasm_bindgen is to use proper es modules
 * so this little shim essentially allows loading the wasm as a worker
 * see the --no-modules flag docs at https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
 */

self.importScripts("./pkg/my_app.js");
const {run} = wasm_bindgen;

(async () => { 
    await wasm_bindgen("./pkg/my_app_bg.wasm");

    let send_event;

    /**
     *  Wasm is ready, now setup communication with the main thread
     *  only 2 types of events are processed:
     *  1. READY (for setup)
     *  2. EVENT (for events sent from the main thread)
     */
    self.onmessage = msg => {
        if(msg.data) {
            if(msg.data.type === "READY") {
                send_event = run(on_ui_state, on_render_state, on_audio_state);
            } else if(msg.data.type === "EVENT") {
                if(msg.data.data.event_type == null) {
                    console.warn("MISSING event_type!");
                } else {
                    const {event_type, data} = msg.data.data;
                    send_event(event_type, data);
                }
            }
        }
    };

    //tell the main thread we're ready
    self.postMessage({
        type: "READY"
    });
})();

function on_ui_state(data) {
    self.postMessage({ type: "UI_STATE", data });
}
function on_render_state(data) {
    self.postMessage({ type: "RENDER_STATE", data });
}
function on_audio_state(data) {
    self.postMessage({ type: "AUDIO_STATE", data });
}
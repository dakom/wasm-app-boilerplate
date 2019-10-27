import { init_core_sender, send_bridge_event_to_ts, send_state_event } from "@events/events";
import { get_audio_context } from "@utils/audio";
import { load_wasm } from "@utils/wasm";
import {render_ui} from "@ui/ui";
import { get_window_size } from "@utils/window";
import "./index.css";

//just to track initial setup/loading
const wasm_loaded = {
    core: false,
    //workers
    fractal: false,
}

//will be set when core wasm is ready
//won't be called until start button is pressed since it needs the audio context
//debug mode will bypass that and start right away
let init_core;

//interface to worker wasm - begin loading it immediately
const fractal_worker = new Worker("fractal-worker-shim.js");
fractal_worker.onmessage = (msg: MessageEvent) => {
    if(msg.data.type === "READY") {
        wasm_loaded.fractal = true;
        try_init_main();
    }
}

//also load the core wasm immediately
load_wasm("wasm/core/pkg/my_core", "wasm_core")
    .then(_init_core => {
        wasm_loaded.core = true;
        init_core = _init_core;
        try_init_main();
    })

//will be called successfully when everything is ready - after that, it's up to state management
function try_init_main() {
    if(!Object.values(wasm_loaded).every(x => x === true)) {
        return; 
    }

    send_state_event("READY");

}

//always render the ui
const on_tick = () => {
    requestAnimationFrame(on_tick);
    render_ui();
}
requestAnimationFrame(on_tick);


//this will be called via send_init_event() which itself is called inside of a state transition
export const start_main = () => {
    const canvas_dom_element = document.getElementById("canvas");
    const { width, height } = get_window_size();

    //when the core has finished loading, it'll send an event (via send_bridge_event_to_ts which is just send_bridge_event on the rust side)
    //that event will cause a state transition and then we're off to the races
    init_core_sender(init_core(canvas_dom_element, get_audio_context(), width, height, send_bridge_event_to_ts));
}
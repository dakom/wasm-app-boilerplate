import { init_core_sender, send_bridge_event_from_core_to_ts_unchecked, send_state_event, send_bridge_event, BridgeEvent } from "@events/events";
import { get_audio_context } from "@utils/audio";
import {render_ui} from "@ui/ui";
import { get_window_size } from "@utils/window";
import "./index.css";
import { debug_settings } from "@config/config";

//render the ui until main game loop starts
//after that point it'll be driven by main loop ticks
let _init_ui_loop = true;
let render_ui_until_game_loop = () => {
    if(_init_ui_loop) {
        render_ui();
        requestAnimationFrame(render_ui_until_game_loop);
    } 
}
requestAnimationFrame(render_ui_until_game_loop);


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
    } else if(msg.data.type === "DRAW") {
        draw_fractal(msg.data);
    }

}

(window as any).load_wasm(core => {
    wasm_loaded.core = true;
    init_core = core.run;
    try_init_main();
});

//will be called successfully when everything is ready - after that, it's up to state management
function try_init_main() {
    if(!Object.values(wasm_loaded).every(x => x === true)) {
        return; 
    }

    send_state_event("READY");
}


//this will be called via state_transition_event() which itself is called inside of a state transition
export const start_main = () => {

    _init_ui_loop = false;

    const canvas_dom_element = document.getElementById("canvas");
    const { width, height } = get_window_size();
    window.onresize = () => {
        send_bridge_event([BridgeEvent.WindowSize, get_window_size()]);
    }

    //when the core has finished loading, it'll send an event (via send_bridge_event_to_ts which is just send_bridge_event on the rust side)
    //that event will cause a state transition and then we're off to the races
    init_core_sender(init_core(canvas_dom_element, get_audio_context(), width, height, send_bridge_event_from_core_to_ts_unchecked));

    init_fractal_worker();
}

let fractal_data:{
    width: number;
    height: number;
    imgData: ImageData;
    dataBuf: ArrayBuffer;
    paletteBuf: ArrayBuffer;
};
function init_fractal_worker() {
    fractal_worker.postMessage({type: "START", maxIterations: debug_settings.maxFractalIterations});
    draw_fractal(null);
}
function draw_fractal(update) {
    const changed = prep_fractal_data();
    if(!changed) {
        fractal_data.dataBuf = update.dataBuf;
        fractal_data.paletteBuf = update.paletteBuf;
        fractal_data.imgData.data.set(new Uint8ClampedArray(fractal_data.dataBuf));
    }

    const {width, height, imgData, dataBuf, paletteBuf} = fractal_data;

    fractal_worker.postMessage({type: "UPDATE", width, height, dataBuf, paletteBuf}, [dataBuf, paletteBuf]);
    send_bridge_event([BridgeEvent.BgTexture, imgData]);
}

function prep_fractal_data() {
    const { width, height } = get_window_size();
    if(!fractal_data || fractal_data.width !== width || fractal_data.height !== height) {
        const imgData = new ImageData(width, height);
        const dataBuf = new ArrayBuffer(imgData.data.byteLength);

        const palette = new Uint8ClampedArray(debug_settings.maxFractalIterations * 4);
        for (let i = 0; i <= palette.length;) {
            palette[i++] = Math.floor(Math.random() * 0xFF);
            palette[i++] = Math.floor(Math.random() * 0xFF);
            palette[i++] = Math.floor(Math.random() * 0xFF);
            palette[i++] = Math.floor(0xFF * .5); 
        }
        fractal_data = {
            paletteBuf: palette.buffer,
            imgData,
            dataBuf,
            width,
            height
        }
        return true;
    } 
    return false;
}
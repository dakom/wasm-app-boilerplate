import {render as renderHtml} from "lit-html";
import {init_events, send_event, CoreEvent} from "@events/events";
import {get_ui_state, set_ui_state, ui} from "@ui/ui";
//import {set_audio_state, get_audio_state, update_audio} from "audio/audio";

const app_worker = new Worker("core-worker-shim.js");
const ui_dom_element= document.getElementById("ui");
const canvas_dom_element= document.getElementById("canvas");

/**
 * Tell the event sender where we're sending to
 */
init_events(app_worker);

//these really just exists in Rust
//only reason we need it here is because rendering has to be on main thread
//so we need to shuttle it between worker and wasm
let webgl_render_state:any;
let renderWebGl:(state:any) => void = () => {};

let audio_state:any;
let renderAudio:(state:any) => void = () => {};

//just a helper
const get_window_size = () => ({
    width: window.innerWidth,
    height: window.innerHeight
});

/**
 * Initialize communication with the worker
 * Only two types of incoming events are processed:
 * 1. READY - just to kick things off
 * 2. STATE (for each state type) - when the worker has sent us a new state that needs to be dispatched
 * 
 */
app_worker.onmessage = (msg:MessageEvent) => {
    if(msg.data) {
        if(msg.data.type === "READY") {
            window.onresize = () => {
                const windowSize = get_window_size();
                send_event([CoreEvent.WindowSize, windowSize]);
            }


            const windowSize = get_window_size();
            app_worker.postMessage({
                type: "READY",
                windowSize
            });
        } else if(msg.data.type === "UI_STATE") {
            set_ui_state(msg.data.data);
        } else if(msg.data.type === "RENDER_STATE") {
            webgl_render_state = msg.data.data;
        } else if(msg.data.type === "AUDIO_STATE") {
            audio_state = msg.data.data;
        }
    }
}

/**
 * 
 * Load the renderer WASM into this thread
 * It'll give us the render function which we call
 * Every tick, if there's a fresh render_state
 */

import("../../../_static/wasm/renderer/pkg/my_renderer")
    .then(({run}) => {
        const {width, height} = get_window_size();
        renderWebGl = run(canvas_dom_element, width, height);
    });

//same with audio
import("../../../_static/wasm/audio/pkg/my_audio")
    .then(({run}) => {
        renderAudio = run();
    });

/**
 * The main graphics loop 
 * If there are fresh renderer or ui states (received from app thread), render and wipe them 
 */
let last = performance.now();
const onTick = (now) => {
    requestAnimationFrame(onTick);

    let local_last = performance.now();

    if(webgl_render_state) {
        renderWebGl(webgl_render_state);
        webgl_render_state = undefined;
    }

    if(get_ui_state()) {
        renderHtml(ui(), ui_dom_element);
        set_ui_state(undefined);
    }

    if(audio_state) {
        renderAudio(audio_state);
        audio_state = undefined;
    }

    const local_now = performance.now();
    const frame_time = now - last;
    const local_time = local_now - local_last;
    //rough remaining frame budget.. don't take too literally to the millisecond, but if it's <3 or so, watch out!
    //console.log(frame_time - local_time);

    last = now;
    local_last = local_now;
}
requestAnimationFrame(onTick);
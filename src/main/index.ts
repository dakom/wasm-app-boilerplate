import {render as renderHtml} from "lit-html";
import {init_ui_events} from "ui/events";
import {get_ui_state, set_ui_state} from "ui/state";
import {set_audio_state, get_audio_state, update_audio} from "./audio/audio";

import {ui} from "ui/ui";
const app_worker = new Worker("core-worker-shim.js");
const ui_dom_element= document.getElementById("ui");
const canvas_dom_element= document.getElementById("canvas");

//this really just exists in Rust
//only reason we need it here is because rendering has to be on main thread
//so we need to shuttle it between worker and wasm
let render_state:any;
/**
 * Tell the event sender where we're sending to
 */
init_ui_events(app_worker);

/**
 * Initialize communication with the worker
 * Only two types of incoming events are processed:
 * 1. READY - just to kick things off
 * 2. UI_STATE - when the worker has sent us a new ui state
 * 
 */
app_worker.onmessage = (msg:MessageEvent) => {
    if(msg.data) {
        if(msg.data.type === "READY") {
            app_worker.postMessage({
                type: "READY"
            });
        } else if(msg.data.type === "UI_STATE") {
            set_ui_state(msg.data.data);
        } else if(msg.data.type === "RENDER_STATE") {
            render_state = msg.data.data;
        } else if(msg.data.type === "AUDIO_STATE") {
            set_audio_state(msg.data.data);
        }
    }
}

/**
 * 
 * Load the renderer WASM into this thread
 * It'll give us the render function which we call
 * Every tick, if there's a fresh render_state
 */

let render:(state:any) => void = () => {};
import("../../_static/wasm/renderer/pkg/my_renderer")
    .then(({run}) => {
        render = run(canvas_dom_element);
    });

/**
 * The main graphics loop 
 * If there are fresh renderer or ui states (received from app thread), render and wipe them 
 */
const onTick = () => {
    requestAnimationFrame(onTick);

    const now = performance.now();

    if(render_state) {
        render(render_state);
        render_state = undefined;
    }

    if(get_ui_state()) {
        renderHtml(ui(), ui_dom_element);
        set_ui_state(undefined);
    }

    if(get_audio_state()) {
        update_audio();
        set_audio_state(undefined);
    }

    //console.log(performance.now() - now);
}
requestAnimationFrame(onTick);
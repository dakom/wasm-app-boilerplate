
import {init_events, send_event, send_event_unchecked, CoreEvent} from "@events/events";
import {get_ui_state, set_ui_state, State as UiState} from "@state/state";
import {ui, renderUi} from "@ui/ui";

//import {set_audio_state, get_audio_state, update_audio} from "audio/audio";

const app_worker = new Worker("core-worker-shim.js");
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

//this is legitimately used in this thread
let ui_state:UiState;

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
app_worker.onmessage = (msg: MessageEvent) => {
    if (msg.data && msg.data.type) {
        switch (msg.data.type) {
            case "READY": 
            {
                window.onresize = () => {
                    const windowSize = get_window_size();
                    send_event([CoreEvent.WindowSize, windowSize]);
                }

                const windowSize = get_window_size();
                app_worker.postMessage({
                    type: "READY",
                    windowSize
                });


                /**
                 * 
                 * Load the renderer WASM into this thread
                 * It'll give us the render function which we call
                 * Every tick, if there's a fresh render_state
                 * 
                 * It's only imported once the worker is ready so it can send events right away
                 */

                import("../../_static/wasm/renderer/pkg/my_renderer")
                    .then(({ run }) => {
                        const { width, height } = get_window_size();
                        renderWebGl = run(canvas_dom_element, width, height, send_event_unchecked);
                    });

            } break;

            case "UI_STATE": ui_state = msg.data.data; break;
            case "RENDER_STATE": webgl_render_state = msg.data.data; break;
            case "AUDIO_STATE": audio_state = msg.data.data; break;
        }
    }
}

//AudioContext must be created through explicit user action
//So this callback is passed down to UI
//Loading audio also depends on the context... but the renderer loading isn't held up by this
export const onStarted= () => {
    const ctor = (window as any).AudioContext || (window as any).webkitAudioContext;
    const ctx = new ctor();

    //same with audio
    import("../../_static/wasm/audio/pkg/my_audio")
        .then(({ run }) => {
            renderAudio = run(send_event_unchecked, ctx);
        });

    send_event(CoreEvent.Started);
}
/**
 * The main graphics loop 
 * If there are fresh renderer or ui states (received from app thread), render and wipe them 
 */
const onTick = () => {
    requestAnimationFrame(onTick);

    const start = performance.now();
    if(webgl_render_state) {
        renderWebGl(webgl_render_state);
        webgl_render_state = undefined;
    }

    if(audio_state) {
        renderAudio(audio_state);
        audio_state = undefined;
    }
    if(ui_state) {
        renderUi(ui_state);
        ui_state = undefined;
    }
    //not perfect but gives rough idea
    const budget = 1000 / 60;
    const taken = performance.now() - start;
    const perc_taken = (taken / budget) * 100;
    const perc_remaining = 100 - perc_taken;
    //console.log(Math.round(perc_remaining) + "% of the frame budget left");

}
requestAnimationFrame(onTick);
import {render as _render} from "lit-html";
import {init_ui_events} from "ui/events";
import {ui_state, set_ui_state} from "ui/state";
import {ui} from "ui/ui";

const app_worker = new Worker("app/worker-shim.js");
const dom_render_target = document.getElementById("app");
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
        }
    }
}

/**
 * The main render loop - will skip rendering if the ui_state is not available
 * After a successful render, ui_state is cleared
 */
const onTick = () => {
    requestAnimationFrame(onTick);

    if(ui_state()) {
        _render(ui(), dom_render_target);
        set_ui_state(undefined);
    }
}
requestAnimationFrame(onTick);

import {init_core_sender, send_io_event_to_core, send_io_event_to_ts, IoEvent} from "@events/events";
import {State} from "@state/state";
import {renderUi} from "@ui/ui";
import {get_window_size} from "@utils/window";
import {get_audio_context} from "@utils/audio";
import {load_wasm} from "@utils/wasm";
import { debugSettings } from "@config/config";
import MainLoop from "mainloop.js";
import "./index.css";

const fractal_worker = new Worker("fractal-worker-shim.js");
fractal_worker.onmessage = (msg: MessageEvent) => {
    console.log(msg);
}

load_wasm("wasm/core/pkg/my_core", "wasm_core")
    .then(run => {
        console.log("CORE LOADED");
        /*
        const canvas_dom_element = document.getElementById("canvas");
        const { width, height } = get_window_size();

        init_core_sender(run(canvas_dom_element, get_audio_context(), width, height, send_io_event_to_ts));
        */
    })

import {get_service} from "@state/state";
import {start_main} from "../entry/index";

export const send_state_event = get_service().send;

export const send_init_event = (evt:"start") => {
    switch(evt) {
        case "start": {
            start_main();
        }
    }
}
/**
 * Customize this for all the event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust BridgeEventIndex!
export enum BridgeEvent {
    LoopBegin,
    LoopUpdate,
    LoopDraw,
    LoopEnd,
    ToggleAudio,
    Speed,
    WindowSize,
    RendererLoaded,
    AudioLoaded,
    Started
}

type ValidEvents = 
    [BridgeEvent.LoopBegin, [number, number]]
    | [BridgeEvent.LoopUpdate, number]
    | [BridgeEvent.LoopDraw, number]
    | [BridgeEvent.LoopEnd, [number, boolean]]
    | BridgeEvent.ToggleAudio
    | BridgeEvent.Started
    | [BridgeEvent.Speed, number]
    | [BridgeEvent.WindowSize, WindowSize]

interface WindowSize{
    width: number;
    height: number;
}

type EventSender = (evt_type:BridgeEvent, evt_data?:any) => void;

//not exported
let send_bridge_event_to_core_unchecked:EventSender;
const send_bridge_event_to_ts_unchecked:EventSender = (evt_type:BridgeEvent, evt_data?:any) => {
    console.log(evt_type, evt_data);
}

//exported interfaces
export const init_core_sender = (fn:EventSender) => {
    send_bridge_event_to_core_unchecked = fn;
}

export const send_bridge_event = (event:ValidEvents) => {
    if(typeof event === "number") {
        send_bridge_event_to_core_unchecked(event);
    } else {
        send_bridge_event_to_core_unchecked(event[0], event[1]);
    }
}

export const send_bridge_event_to_ts = (event:ValidEvents) => {
    
}

//wasm_worker.postMessage({ type: "EVENT", evt_type, evt_data})

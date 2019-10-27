import {get_service} from "@state/state";
import {start_main} from "../entry/index";
import {BridgeEvent, ValidBridgeEvents} from "./bridge-events";
export * from "./bridge-events";

//could come from anywhere - sends to the typescript statechart
export const send_state_event = get_service().send;

//could be called from anywhere - sends to the wasm/core
export const send_bridge_event = (event:ValidBridgeEvents) => {
    if(typeof event === "number") {
        send_bridge_event_to_core_unchecked(event);
    } else {
        send_bridge_event_to_core_unchecked(event[0], event[1]);
    }
}

//inherently via state transitions
export const state_transition_event = (evt:"start") => {
    switch(evt) {
        case "start": {
            start_main();
        }
    }
}

//from wasm/core
export const send_bridge_event_from_core_to_ts_unchecked:EventSender = (evt_type:BridgeEvent, evt_data?:any) => {
    switch(evt_type) {
        case BridgeEvent.AssetsLoaded: send_state_event("ASSETS_LOADED"); break;
    }
}

//Needed for glue
export const init_core_sender = (fn:EventSender) => {
    send_bridge_event_to_core_unchecked = fn;
}
let send_bridge_event_to_core_unchecked:EventSender;

//just to help the type checker 
type EventSender = (evt_type:BridgeEvent, evt_data?:any) => void;

/**
 * Customize this for all the event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust IoEventIndex!
export enum IoEvent {
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
    [IoEvent.LoopBegin, [number, number]]
    | [IoEvent.LoopUpdate, number]
    | [IoEvent.LoopDraw, number]
    | [IoEvent.LoopEnd, [number, boolean]]
    | IoEvent.ToggleAudio
    | IoEvent.Started
    | [IoEvent.Speed, number]
    | [IoEvent.WindowSize, WindowSize]

interface WindowSize{
    width: number;
    height: number;
}

type EventSender = (evt_type:IoEvent, evt_data?:any) => void;

let send_io_event_to_core_unchecked:EventSender;

export const init_core_sender = (fn:EventSender) => {
    send_io_event_to_core_unchecked = fn;
}

export const send_io_event_to_core = (event:ValidEvents) => {
    if(typeof event === "number") {
        send_io_event_to_core_unchecked(event);
    } else {
        send_io_event_to_core_unchecked(event[0], event[1]);
    }
}

export const send_io_event_to_ts = (event:ValidEvents) => {
    
}

const send_io_event_to_ts_unchecked:EventSender = (evt_type:IoEvent, evt_data?:any) => {
    console.log(evt_type, evt_data);
}
//wasm_worker.postMessage({ type: "EVENT", evt_type, evt_data})

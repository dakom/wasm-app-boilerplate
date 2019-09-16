
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


let wasm_worker:Worker;

export const init_events= (_wasm_worker:Worker) => wasm_worker = _wasm_worker;

export const send_event = (event:ValidEvents) => {
    if(typeof event === "number") {
        send_event_unchecked(event);
    } else {
        send_event_unchecked(event[0], event[1]);
    }
}

export const send_event_unchecked = (evt_type:IoEvent, evt_data?:any) => {
    wasm_worker.postMessage({ type: "EVENT", evt_type, evt_data})
}

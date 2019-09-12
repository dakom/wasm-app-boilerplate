
/**
 * Customize this for all the event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust UiEvent!
export enum CoreEvent {
    ToggleAudio,
    SetVelocity,
    WindowSize,
    RendererLoaded,
    AudioLoaded,
    Started
}

type ValidEvents = 
    CoreEvent.ToggleAudio
    | CoreEvent.Started
    | [CoreEvent.SetVelocity, number]
    | [CoreEvent.WindowSize, WindowSize]

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

export const send_event_unchecked = (evt_type:CoreEvent, evt_data?:any) => {
    wasm_worker.postMessage({ type: "EVENT", evt_type, evt_data})
}

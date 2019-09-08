/**
 * Customize this for all the event types
 * If there are any complex objects, create structs on the Rust side too!
 */
type ValidEvents = 
    CoreEvent.ToggleAudio
    | [CoreEvent.SetVelocity, number]

//The order of these must match the Rust UiEvent!
export enum CoreEvent {
    ToggleAudio,
    SetVelocity,
}

let wasm_worker:Worker;

export const init_events= (_wasm_worker:Worker) => wasm_worker = _wasm_worker;

export const send_event = (event:ValidEvents) => {
    const data = typeof event === "number" 
        ? {event_type: event}
        : {event_type: event[0], data: event[1]};

    wasm_worker.postMessage({ type: "EVENT", data });
}
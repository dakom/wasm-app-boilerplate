/**
 * Customize this for all the ui types
 * If there are any complex objects, create structs on the Rust side too!
 */
type ValidEvents = 
    UiEvent.AppendText 
    | [UiEvent.UpdateInput, string]

//The order of these must match the Rust UiEvent!
export enum UiEvent {
    AppendText,
    UpdateInput,
}

let wasm_worker:Worker;

export const init_ui_events = (_wasm_worker:Worker) => wasm_worker = _wasm_worker;

export const ui_event = (event:ValidEvents) => {
    const data = typeof event === "number" 
        ? {event_type: event}
        : {event_type: event[0], data: event[1]};

    wasm_worker.postMessage({ type: "UI_EVENT", data });
}

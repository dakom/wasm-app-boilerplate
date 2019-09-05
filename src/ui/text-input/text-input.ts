import {html} from "lit-html";
import {get_ui_state} from "ui/state";
import {ui_event, UiEvent} from "ui/events";

import "./text-input.css";

export const text_input = () => {
    const value = get_ui_state().textInput;

    const onSubmit = () => ui_event(UiEvent.AppendText);
    const onInput = evt => ui_event([UiEvent.UpdateInput, evt.target.value]);
    const onKeyUp = ({key}) => {
        if(key === "Enter") {
            onSubmit();
        }
    } 

    return html`
        <div class="text-input">
            <input type="text" .value=${value} @input=${onInput} @keyup=${onKeyUp} placeholder="insert text" ></input>
            <button @click=${onSubmit}>send</button>
        </div>
    `
}
import {html} from "lit-html";
import {get_ui_state} from "~/ui/ui";
import {send_event, CoreEvent} from "~/events/events";

import "./text-input.css";

export const text_input = () => {
    const value = get_ui_state().textInput;

    const onSubmit = () => send_event(CoreEvent.AppendText);
    const onInput = evt => send_event([CoreEvent.UpdateInput, evt.target.value]);
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
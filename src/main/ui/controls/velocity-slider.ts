import {html} from "lit-html";
import {get_ui_state} from "~/ui/ui";
import {send_event, CoreEvent} from "~/events/events";

import "./velocity-slider.css";

export const velocity_slider = () => {
    const {speed} = get_ui_state();
    const onInput = evt => {
        send_event([CoreEvent.SetVelocity, parseFloat(evt.target.value)]);
    }

    return html`
        <div class="velocity-slider">
            <div class="label">Current speed: ${speed} </div>
            <input type="range" min="0" max="1" value=".5" step=".0001" @input=${onInput}>
        </div>
    `
}
import {html} from "lit-html";
import {get_state} from "@state/state";
import {send_bridge_event, BridgeEvent} from "@events/events";

import "./speed-slider.css";

export const speed_slider = () => {
    const {speed} = get_state();

    const onInput = evt => {
        send_bridge_event([BridgeEvent.Speed, parseFloat(evt.target.value)]);
    }

    return html`
        <div class="velocity-slider">
            <div class="label">Smiley speed: ${speed} </div>
            <input type="range" min="0" max="1" value="${speed}" step=".0001" @input=${onInput}>
        </div>
    `
}
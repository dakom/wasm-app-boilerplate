import {html} from "lit-html";
import {get_ui_state} from "@state/state";
import {send_event, CoreEvent} from "@events/events";

import "./audio-toggle.css";

export const audio_toggle = () => {
    const {audioActive} = get_ui_state();

    const onClick = () => send_event(CoreEvent.ToggleAudio);

    return html`
        <div class="audio-toggle">
            ${audioActive 
                ? html`<i class="fas fa-volume-up fa-3x" @click=${onClick} ></i>`
                : html`<i class="fas fa-volume-mute fa-3x" @click=${onClick} ></i>`
            }
        </div>
    `
}
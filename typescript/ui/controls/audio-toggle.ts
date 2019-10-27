import {html} from "lit-html";
import {get_state} from "@state/state";
import {send_io_event_to_core, IoEvent} from "@events/events";

import "./audio-toggle.css";

export const audio_toggle = () => {
    const {audio_active} = get_state();

    const onClick = () => send_io_event_to_core(IoEvent.ToggleAudio);

    return html`
        <div class="audio-toggle">
            ${audio_active
                ? html`<i class="fas fa-volume-up fa-3x" @click=${onClick} ></i>`
                : html`<i class="fas fa-volume-mute fa-3x" @click=${onClick} ></i>`
            }
        </div>
    `
}
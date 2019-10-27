import {html} from "lit-html";
import {get_state} from "@state/state";
import {send_bridge_event, BridgeEvent, send_state_event} from "@events/events";

import "./audio-toggle.css";

export const audio_toggle = () => {
    const {audio_active} = get_state().context;

    const onClick = () => {
        send_bridge_event(BridgeEvent.ToggleAudio);
        send_state_event("TOGGLE_AUDIO");
    }

    return html`
        <div class="audio-toggle">
            ${audio_active
                ? html`<i class="fas fa-volume-up fa-3x" @click=${onClick} ></i>`
                : html`<i class="fas fa-volume-mute fa-3x" @click=${onClick} ></i>`
            }
        </div>
    `
}
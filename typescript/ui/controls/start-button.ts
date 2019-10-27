import {html} from "lit-html";
import {send_state_event} from "@events/events";

export const start_button = () => {
    return html`
    <div class="start" @click=${() => send_state_event("START")}>START</div>
    `;
}
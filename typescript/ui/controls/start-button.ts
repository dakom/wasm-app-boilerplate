import {html} from "lit-html";
import {onStarted} from "../../entry";

export const start_button = () => {
    return html`
    <div class="start" @click=${onStarted}>START</div>
    `;
}
import {html} from "lit-html";
import {text_input} from "ui/text-input/text-input";
import {results} from "ui/results/results";
import {get_ui_state} from "./state";
import "./ui.css";

export const ui = () => {
    return html`
        <div class="container">
            <div class="content">
                ${text_input()}
                ${results()}
            </div>
        </div>
    `;
}
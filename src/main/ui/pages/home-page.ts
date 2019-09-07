import {html} from "lit-html";
import {text_input} from "~/ui/text-input/text-input";
import {results} from "~/ui/results/results";

export const home = () => html`
    <div class="container">
        <div class="content">
            ${text_input()}
            ${results()}
        </div>
    </div>
`;
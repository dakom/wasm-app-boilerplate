import {html} from "lit-html";
import {get_ui_state} from "ui/state";
import "./results.css";

export const results = () => {
    const results = get_ui_state().results;
    return !results.length ? null : list_results(results);
}

const list_results = (results:Array<string>) => html`
    <div class="results">
        <header>Received:</header>
        <ul>
            ${results.map(result => html`
                <li>${result}</li>
            `)}
        </ul>
    </div>
`
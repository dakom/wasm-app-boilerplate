import { get_state } from "@state/state";
import { audio_toggle } from "@ui/controls/audio-toggle";
import { speed_slider } from "@ui/controls/speed-slider";
import { start_button } from "@ui/controls/start-button";
import { html } from "lit-html";
import {if_state_html} from "@utils/xstate";
import "./home-page.css";

export const home = () => {
    const state_map = new Map();
    state_map.set(["init", "start_loading"], loading);
    state_map.set("waiting", waiting);

    return html`
        <div class="ui">
            ${if_state_html(state_map) (get_state())}
        </div>
    `
}

const waiting= () => html`
    <div class="home-page-container">
        <div class="home-page-contents">
            ${start_button()}
        </div>
    </div>
`
const loading = () => html`
    <div class="home-page-container">
        <div class="home-page-contents">
            <h1>Loading...</h1>
        </div>
    </div>
`

const ready = () => html`
    <div class="ui">
        <div class="home-page-header">
            ${audio_toggle()}
        </div>
        <div class="home-page-container">
            <div class="home-page-contents">
                ${speed_slider()}
            </div>
        </div>
    </div>
`

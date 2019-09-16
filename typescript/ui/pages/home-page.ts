import {html} from "lit-html";
import {speed_slider} from "@ui/controls/speed-slider";
import {audio_toggle} from "@ui/controls/audio-toggle";
import {get_state, InitPhase} from "@state/state";
import {start_button} from "@ui/controls/start-button";
import "./home-page.css";
import { readSync } from "fs";

export const home = () => html`
    <div class="ui">
    ${
        get_state().init_phase === InitPhase.Waiting ? waiting()
        : get_state().init_phase === InitPhase.Loading ? loading()
        : ready()
    }
    </div>
`

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

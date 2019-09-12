import {html} from "lit-html";
import {velocity_slider} from "@ui/controls/velocity-slider";
import {audio_toggle} from "@ui/controls/audio-toggle";
import {onStarted} from "../../entry";
import {get_ui_state, InitPhase} from "@state/state";
import "./home-page.css";
import { readSync } from "fs";

export const home = () => html`
    <div class="ui">
    ${
        get_ui_state().initPhase === InitPhase.Waiting ? waiting()
        : get_ui_state().initPhase === InitPhase.Loading ? loading()
        : ready()
    }
    </div>
`

const waiting= () => html`
    <div class="home-page-container">
        <div class="home-page-contents">
            <div class="start" @click=${onStarted}>START</div>
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
                ${velocity_slider()}
            </div>
        </div>
    </div>
`

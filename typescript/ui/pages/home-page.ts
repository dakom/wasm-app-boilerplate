import {html} from "lit-html";
import {velocity_slider} from "@ui/controls/velocity-slider";
import {audio_toggle} from "@ui/controls/audio-toggle";
import {get_ui_state} from "@state/state";
import "./home-page.css";
import { readSync } from "fs";

export const home = () => 
    !get_ui_state().allLoaded 
        ? html`
            <div>
                <div class="home-page-container">
                    <div class="home-page-contents">
                        <h1>Loading...</h1>
                    </div>
                </div>
            </div>
            `
        : html`
            <div>
                <div class="home-page-header">
                    ${audio_toggle()}
                </div>
                <div class="home-page-container">
                    <div class="home-page-contents">
                        ${velocity_slider()}
                    </div>
                </div>
            </div>
        `;
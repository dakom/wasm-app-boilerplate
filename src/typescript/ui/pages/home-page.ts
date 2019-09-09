import {html} from "lit-html";
import {velocity_slider} from "@ui/controls/velocity-slider";
import {audio_toggle} from "@ui/controls/audio-toggle";
import "./home-page.css";

export const home = () => html`
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
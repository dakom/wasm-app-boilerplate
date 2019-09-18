import {home} from "@ui/pages/home-page"
import {render} from "lit-html";
import {set_state, State} from "@state/state";
import "./ui.css";

const ui_dom_element= document.getElementById("ui");

export const renderUi = (state:State, interpolation:number) => {
    set_state(state);
    render(ui(), ui_dom_element);
}

export const ui = () => {
    //router stuff could happen here
    return home();
}
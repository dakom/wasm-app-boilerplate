import {home} from "@ui/pages/home-page"
import {render} from "lit-html";
import "./ui.css";

const ui_dom_element= document.getElementById("ui");

export const render_ui = () => {
    render(ui(), ui_dom_element);
}

const ui = () => {
    //router stuff could happen here
    return home();
}
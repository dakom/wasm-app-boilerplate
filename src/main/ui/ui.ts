import {home} from "~/ui/pages/home-page"
import "./ui.css";

export interface UiState {
    textInput: string;
    results: Array<string>;
    interpolation: number;
}

let state:UiState;

export const get_ui_state = ():Readonly<UiState> => state;
export const set_ui_state = (_state:UiState) => state = _state;

export const ui = () => {
    //router stuff could happen here
    return home();
}
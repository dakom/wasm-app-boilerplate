export interface UiState {
    textInput: string;
    results: Array<string>;
}

let state:UiState;

export const ui_state = ():Readonly<UiState> => state;
export const set_ui_state = (_state:UiState) => state = _state;

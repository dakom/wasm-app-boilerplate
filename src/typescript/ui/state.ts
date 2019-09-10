export interface UiState {
    audioActive: boolean;
    speed: number;
    allLoaded: boolean;
}

let state:UiState;

export const get_ui_state = ():Readonly<UiState> => state;
export const set_ui_state = (_state:UiState) => state = _state;
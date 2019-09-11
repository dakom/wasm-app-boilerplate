export interface State {
    audioActive: boolean;
    speed: number;
    allLoaded: boolean;
}

let state:State;

export const get_ui_state = ():Readonly<State> => state;
export const set_ui_state = (_state:State) => state = _state;
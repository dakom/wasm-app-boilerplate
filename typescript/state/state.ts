/**
 * Make sure this matches state in rust shared!
 */

export interface State {
    audioActive: boolean;
    speed: number;
    initPhase: number;
}

//These need to match the order on the rust side
export enum InitPhase {
    Waiting,
    Loading,
    Ready
}

let state:State;

export const get_ui_state = ():Readonly<State> => state;
export const set_ui_state = (_state:State) => state = _state;
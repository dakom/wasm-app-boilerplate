export interface State {
    audio_active: boolean, 
    renderer_active: boolean,
    speed: number,
    window_width: number,
    window_height: number,
    ball_position_x: number,
    ball_position_y: number,
    collision: boolean
}

export type StateSetter = (old:State) => State;

let _state:State;

export const get_state = () => _state;
export const set_state = (fn:StateSetter) => _state = fn(_state);
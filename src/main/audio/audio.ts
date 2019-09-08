interface AudioState {
    isActive: boolean;
    interpolation: number;
}

let state:AudioState;
let previous:AudioState;

export const set_audio_state = (_state:AudioState) => {
    state = _state;
}

export const get_audio_state = () => state;


export const update_audio = () => {
    if(!previous || previous.isActive !== state.isActive) {
        console.log("AUDIO TOGGLE:", state.isActive);
        previous = {...state};
    }
}

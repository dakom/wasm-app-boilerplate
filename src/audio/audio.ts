interface AudioState {
    interpolation: number;
}

let state:AudioState;

export const set_audio_state = (_state:AudioState) => {
    state = _state;
}

export const get_audio_state = () => state;

export const update_audio = () => {
    if(state) {
        //See if need to play audio
    }
}

interface AudioUpdate {
    flag: boolean;
}

let update:AudioUpdate;
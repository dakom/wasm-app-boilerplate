interface AudioState {
    interpolation: number;
}

let state:AudioState;

export const set_audio_state = (_state:AudioState) => {
    state = _state;
}

export const get_audio_state = () => state;


let previous:AudioState;
export const update_audio = () => {

    const diff = get_audio_diff();
    if(diff) {
        //console.log("DOING AUDIO STUFF", diff);
        previous = Object.assign({}, state);
    }
}

const get_audio_diff = ():Partial<AudioState> => {
    if(!previous) {
        return {
            ...state
        }
    } else {
        if(previous.interpolation !== state.interpolation) {
            return {
                interpolation: state.interpolation
            }
        }
    }
}
import { interpret, Machine, spawn, assign } from "xstate";
import {state_transition_event} from "@events/events";
import {debug_settings} from "@config/config";

const machine = Machine({
    id: "main",
    initial: "init",
    context: {
        speed: .5,
        window_width: 0,
        window_height: 0,
        audio_active: true 
    },
    states: {
        init: {
            on: { 
                READY: [
                    {target: "start_loading", cond: () => debug_settings.skipStart},
                    {target: "waiting"},
                ]
            }
        },
        waiting: {
            on: {
                START: "start_loading"
            }
        },

        start_loading: {
            onEntry: () => state_transition_event("start"),
            on: {
                ASSETS_LOADED: "running"
            }
        },

        running: {
            on: {
                TOGGLE_AUDIO: {
                    actions: assign({
                        audio_active: (ctx:any) => !ctx.audio_active
                    }) as any
                }
            }
        }
    }
});

let _service;

export const get_service = () => {
    if(!_service) {
        _service = 
            interpret(machine).start();
    }

    return _service;
}

export const get_state = () => get_service().state;
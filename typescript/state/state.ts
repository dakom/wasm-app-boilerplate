import { interpret, Machine, spawn, assign } from "xstate";
import {send_init_event} from "@events/events";
import {debug_settings} from "@config/config";

const machine = Machine({
    id: "main",
    initial: "init",
    context: {
        speed: .5,
        window_width: 0,
        window_height: 0,
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
        }
    }
});

let _service;

export const get_service = () => {
    if(!_service) {
        _service = 
            interpret(machine)
                .onTransition(state => {
                    if(state.changed) {
                        if(state.matches("start_loading") && (state.history.matches("waiting") || state.history.matches("init"))) {
                            send_init_event("start");
                        }
                    }
                    console.log(state.value);
                })
                .start();
    }

    return _service;
}

export const get_state = () => get_service().state;
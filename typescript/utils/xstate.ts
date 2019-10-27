import { TemplateResult } from "lit-html";
import { EventObject, State } from "xstate";
import { find_map, find_object } from "./common";

export const if_state_fn = <TContext, TEvent extends EventObject, T>(lookup:{[key:string]: () => T} | Map<string | Array<string>, () => T>) => (state:State<TContext, TEvent>):() => T => {
    const maybe_fn = lookup instanceof Map
        ?  find_map <string | Array<string>, () => T>(ks => 
            Array.isArray(ks)
                ?  ks.findIndex(k => state.matches(k)) !== -1
                :  state.matches(ks)
            ) (lookup)
        : find_object<() => any> (k => state.matches(k)) (lookup);

    const res = maybe_fn;
    if(res === undefined) {
        console.error(`NO VALID STATE!!! [${state.value.toString()}]`);
        return () => null;
    };

    return res;
}
export const if_state_exec = <TContext, TEvent extends EventObject, T>(lookup:{[key:string]: () => T} | Map<string | Array<string>, () => T>) => (state:State<TContext, TEvent>):T => {
    const fn = if_state_fn (lookup) (state);
    return fn === undefined ? undefined : fn();
}

export const if_state_html = <TContext, TEvent extends EventObject>(lookup:{[key:string]: () => TemplateResult} | Map<string | Array<string>, () => TemplateResult>) => (state:State<TContext, TEvent>):TemplateResult => 
    if_state_exec(lookup) (state);

[![Build Status](https://travis-ci.org/dakom/wasm-app-boilerplate.svg?branch=master)](https://travis-ci.org/dakom/wasm-app-boilerplate)

## [LIVE DEMO](https://dakom.github.io/wasm-app-boilerplate)

# Dataflow
![flowchart](https://i.imgur.com/u4GFKsM.png)

# Types for Events and State

In order to keep the type checker happy where it counts, events are managed both in Typescript and in Rust as follows:

### Events
1. Update `ValidEvents` and `CoreEvent` in [main events](src/main/events/events.ts)
2. Update `Event` in [worker core events](src/crates/core/src/events/events.rs)

Note - the events are unified in that they are all sent TO one place (core)

### State
1. There is a local-only crate called [shared](src/crates/shared) which has feature flags for state that each crate needs to know about 
2. Core should have them all enabled (it sends state TO every other crate)
3. Each other crate only needs to enable its specific feature (in Cargo.toml where it lists the `shared` crate as a dependency)
2. Ui must be updated [in typescript](src/main/ui/ui.ts)

It takes some effort to keep these in sync, but once it's all setup, the whole state and event flow will work with 100% strict compile-time checks both on the typescript side and on rust!

(note - if using a serialization format like flatbuffers, then the above could be obsolete as the per-language definitions are generated from the common files)

# Managing application state in WASM

The core mechanism in the demo here is the Shipyard Enity Component System, and with just a bare minimum example needed to shuttle the events and state back and forth across all areas.

The ECS could easily be swapped with a different approach altogether - functional reactive programming, statecharts, etc.

Whatever it is, it's updated internally in a game loop - and ultimately sends out discrete state updates for all the dependants (`ui`, `webgl renderer`, and `audio`)

These state updates are _not_ synonymous with the entire application state (though it can be - whatever floats your boat!)

They also don't need to be sent/extracted at the same frequency as eachother (e.g. the ui could be sent when events come in as opposed to a raf loop)

It might be tempting to consider completely different state managements, each in their own worker, but it's more likely that there needs to be some coherence between them (e.g. audio gets triggered on collision, UI sends an event to update inventory, etc.) so there is one "Source of Truth" and the split is done before sending. 

(note - there are potential avenues to optimize here, like only sending deltas or serializing to a binary format like flatbuffers... but that all comes at a computational cost and would need profiling to see if it's really worthwhile, so neither of those are included here. Rather, it's simple serde-powered JS Objects <-> Rust structs)

# HTML rendering of ui state

The DOM is re-rendered via lit-html every frame-tick if there's a fresh ui state.

`get_ui_state()` can be queried from anywhere, and `events` can be sent from anywhere. They have no inherent relationship to the dom hierarchy. 

In many apps, especially those that are most similar to traditional websites, it makes sense for application state and ui state to be the same thing- and indeed keeping them the same is as simple setting `ui state` from the entire application state on the Rust side - so that is doable from this starting point. However, they are still technically separate, on purpose, since it happens often enough that they aren't 1:1, and I personally found coping with that reality very frustrating in several popular frameworks. 

### Ui State vs. DOM state 

The following really applies everywhere, but it's only necessary to show it in terms of UI and it can then be extrapolated for WebGL, audio, etc.:

**The rule of thumb is that asynchronous callbacks should never depend on `get_ui_state()`. Either use a locally cached copy or get it from the element.**

In other words, `state` must be considered only synchronously! There's no guarantee that the state seen at the time of a render is the same as the state seen when an async callback fires, or even that it exists at all at that point!

Additionally, the html spec allows setting an element's attribute without affecting its property, and an element may be updated by user interaction before the latest state is flushed.

These are all _good_ things since it means the ui is more responsive and it avoids data-race conditions by making it more explicit where values come from. I wish there were a way to enforce this on a compiler level in JS but I don't see how - so it requires knowing the usage pattern explained here:

A locally cached copy is an _explicit_ choice, and valid, though often not the right one as far as DOM elements are concerned... it's usually better to use the event itself if that's the intended target.

Consider the following example... Assume that when this was rendered, `ui_state.textInput` was "hello world" and that the user pressed the "!" key causing an update:


```
const text_input = () => {
    const value = get_ui_state().textInput;
    const onSubmit = () => send_event("appendText");

    const onInput = evt => {
        console.log(value); // "hello world"
        console.log(evt.target.value); // "hello world!"
        console.log(get_ui_state().textInput); //TypeError: Cannot read property textInput of undefined
    }

    return html`
        <div class="text-input">
            <input type="text" .value=${value} @input=${onInput} placeholder="insert text" ></input>
            <button @click=${onSubmit}>send</button>
        </div>
    `
}
```

Even though `value` was set from `get_ui_state().textInput`, the latter is now undefined (because it was wiped between the time the function was called and the time the callback was fired).

Also `evt.target.value` is different than `value` even though it is bound to it. If `onInput` sends a state change, then perhaps it would have matched `value`, but there's a small window of possible difference. Better to use the `evt.target.value`, after all - that's the actual event data! 

# WebGL / Audio

WebGl uses awsm_web to manage gl state but it's kept to a very small proof of concept here. Of course, sky's the limit!

Same idea applies to audio 

# Directory Structure

The only real gotcha is that it's a nicer IDE experience to jump into the individual folders, and getting typescript to play nicely meant copying an additional tsconfig.json into `src/main/`. No biggie though.

# Build Scripts 

Some cargo binaries are expected to be there, like watchexec and wasm-bindgen-cli (installed via cargo install)

Also rust, the toolchain, wasm target, etc.

Lastly - `wasm-opt` should be on the path. Simplest is to download and extract from the [binaryen releases](https://github.com/WebAssembly/binaryen/releases) and add it to your path.

Other than that, `npm` is used as the task runner. There's lots of minutia handled via sub-scripts and those are prefaced with an underscore.

The only ones that are really run directly are those without an underscore (e.g. `npm start`)

To do a complete build including copying to a dist folder, `npm run bundle:local`, but in a CI environment the copy step might be different (hence the additional bundle option) 

All if this is setup in Travis for simple CI/CD to `gh-pages`, just set the `GITHUB_TOKEN` as an environment variable


## Development 

On first run, the sources will need to compile which will take a while. Subsequent recompiles are _much_ faster.

The worker JS itself is actually a very small file in the static dir... no reason to mess with that at all

When the Rust/WASM recompiles, it places the wasm in the static directory too. This will be cleaned via `npm run clean`

Webpack is configured to watch for changes in the static dir (this is a speedup compared to having webpack watch rust sources).

Lastly, Rust is setup via its own watcher (watchexec) to recompile when its sources change, and this is configured as an npm script

So there are multiple processes that run in parallel and both are launched at `npm start`:

1. Webpack (with various settings): for typescript, core bundling, and static folder changes
2. Watchexec -> npm -> rustc/wasm_bindgen/etc: for rust compilation (per each rust crate)

In this way, the typescript reloading can be super fast and take advantage of HMR, and the Rust won't trigger false positives as the source changes and has compiler errors.

## Deployment

The dev server used the static dir via a configuration option, at deployment its copied manually
This happens via the `cpy` npm script for local sanity checks, but it's a little finicky for production
Rather, shell scripts are used in Travis (or whatever CI) with the couple lines to copy the required folders into `dist`

The production settings for both typescript and rust are optimized for the deployment version too.
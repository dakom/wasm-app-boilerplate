[![Build Status](https://github.com/dakom/wasm-app-boilerplate/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/wasm-app-boilerplate/actions)

## [LIVE DEMO](https://dakom.github.io/wasm-app-boilerplate)

# Dataflow
![flowchart](https://i.imgur.com/7tJoLKg.png)

# Files and directories

* [crates/core/src](crates/core/src) - the worker core (driven by [shipyard ECS](https://github.com/leudz/shipyard)) 
* [crates/renderer/src](crates/renderer/src) - webgl renderer output (driven by [awsm_web](https://github.com/dakom/awsm)) 
* [crates/audio/src](crates/audio/src) - audio output (plain web-sys) 
* [crates/shared/src](crates/shared/src) - state and events on the rust side 
* [typescript/ui](typescript/ui) - the rendering of ui state
* [typescript/events](typescript/events) - events on the typescript side 
* [typescript/state](typescript/state) - state on the typescript side 
* [typescript/utils](typescript/utils) - utils for typescript 
* [typescript/config](typescript/config) - config for typescript 
* [_static/media](_static/media) - static media 

These are pretty much locked and won't need editing:

* dev config files (webpack, tsconfig.json, cargo.toml, package.json, etc.)
* [_static](_static) other than the `media` folder
* [typescript/entry](typescript/entry)

# Shared types for Events and State

We want the events and state to be checked by the compiler on both the Typescript and Rust side, and this gets a little tricky since they are also sometimes shared between crates and modules.

Events that are sent between threads are really all I/O in some sense, so they are called `BridgeEvent`

State is unified as one struct/interface, since it is sent at a particular point in time - but it could easily be split up or be thought of as a container object.

Keeping all this in sync takes a manual change - i.e. creating a new event or state on the Rust side means changing it on the TS side

Also, for the sake of efficiency, enums should be serialized as simple integers. This is setup with examples in both directions (see event type and init phase) 

To keep things idiomatic in both languages, events are enums that own their data when going from Rust, and index-based enums with sidecar data when going from JS.

Fun!

It will seem super complicated at first, but it's really not so bad thanks to the type checking - let the compiler be your guide :)

(serialization / deserialization is via serde Rust <> JS Objects... if flatbuffers were used, it could be an optimization here due to Transferable objects, and address the overall headache of having multiple sources of truth - but it could be a performance cost due to the runtime impact. Profile and see before pulling the trigger!)

# Managing application state in WASM

The core mechanism in the demo here is the [Shipyard Enity Component System](https://github.com/leudz/shipyard), and with just a bare minimum example needed to shuttle the events and state back and forth across all areas and kick the tires where they need kickin'.

The ECS could easily be swapped with a different approach altogether - functional reactive programming, statecharts, etc. but ECS is great!

Whatever it is, it's updated in a game loop - and ultimately sends out the derived output state

# HTML rendering of ui state

The DOM is re-rendered via lit-html every frame-tick. This is fast since lit-html doesn't need to diff (short summary - the dynamic parts only need to check against _themselves_ to see if they're dirty, and if they are they know where and what to write)

`get_state()` and `send_event()` can be queried from anywhere. They have no inherent relationship to the dom hierarchy. 

### Ui State vs. DOM state 

The following really applies everywhere, but it's only necessary to show it in terms of UI and it can then be extrapolated for WebGL, audio, etc. Plus I think it's more of a footgun when it comes to UI:

**The rule of thumb is that asynchronous callbacks should never depend on `get_state()`. Either use a locally cached copy or get it from the element.**

There's no guarantee that the state seen at the time of a render is the same as the state seen when an async callback fires, or even that it exists at all at that point!

Additionally, the html spec allows setting an element's attribute without affecting its property, and an element may be updated by user interaction before the latest state is flushed.

These are all _good_ things since it means the ui is more responsive and it avoids data-race conditions by making it more explicit where values come from. I wish there were a way to enforce this on a compiler level in JS but I don't see how - so it requires knowing the usage pattern explained here:

A locally cached copy is an _explicit_ choice, and valid, though often not the right one as far as DOM elements are concerned... it's usually better to use the event itself if that's the intended target.

Consider the following example... Assume that when this was rendered, `state.textInput` was "hello world" and that the user pressed the "!" key causing an update:


```
const text_input = () => {
    const value = get_state().textInput;
    const onSubmit = () => send_event("appendText");

    const onInput = evt => {
        console.log(value); // "hello world"
        console.log(evt.target.value); // "hello world!"
        console.log(get_state().textInput); //TypeError: Cannot read property textInput of undefined
    }

    return html`
        <div class="text-input">
            <input type="text" .value=${value} @input=${onInput} placeholder="insert text" ></input>
            <button @click=${onSubmit}>send</button>
        </div>
    `
}
```

Even though `value` was set from `get_state().textInput`, the latter is now undefined (because it was wiped between the time the function was called and the time the callback was fired).

Also `evt.target.value` is different than `value` even though it is bound to it. If `onInput` sends a state change, then perhaps it would have matched `value`, but there's a small window of possible difference. Better to use the `evt.target.value`, after all - that's the actual event data! 

# WebGL / Audio

WebGl uses [awsm_web](https://github.com/dakom/awsm) to manage gl state but it's kept to a very small proof of concept here. Of course, sky's the limit!

Audio uses straight web-sys/bindgen/js-sys etc. One gotcha is that user interaction must happen before an AudioContext can be created, and an AudioContext is required for loading AudioBuffers. That's setup to work properly here via the start button callback (which does _not_ hold back the renderer from loading images).

# Requirements 

Some cargo binaries are expected to be there, like `watchexec` and `wasm-bindgen-cli` (installed via cargo install)

Also rust, the toolchain, wasm target, etc.

Lastly - `wasm-opt` should be on the path. Simplest is to download and extract from the [binaryen releases](https://github.com/WebAssembly/binaryen/releases) and add it to your path.

Other than that, `npm` is used as the task runner. There's lots of minutia handled via sub-scripts and those are prefaced with an underscore.

The only ones that are really run directly are those without an underscore (e.g. `npm start`)

To do a complete build including copying to a dist folder, `npm run bundle:local`, but in a CI environment the copy step might be different (hence the additional bundle option) 

All if this is setup in Travis for simple CI/CD to `gh-pages`, just set the `GITHUB_TOKEN` as an environment variable


## Development 

Basically, `npm start --silent` (silent makes it nicer to not get NPM errors when we're already getting Rust/TS errors)

On first run, the sources will need to compile which will take a while. Subsequent recompiles are _much_ faster.

The worker JS itself is actually a very small file in the static dir... no reason to mess with that at all

When the Rust/WASM recompiles, it places the wasm in the static directory too. This will be cleaned via `npm run clean`

Webpack is configured to watch for changes in the static dir (this is a speedup compared to having webpack watch rust sources).

Lastly, Rust is setup via its own watcher (watchexec) to recompile when its sources change, and this is configured as an npm script

The wasm which is imported via the entry (not worker) is imported as a module.

So there are multiple processes that run in parallel and both are launched at `npm start`:

1. Webpack (with various settings): for typescript, core bundling, and static folder changes
2. Watchexec -> npm -> rustc/wasm_bindgen/etc: for rust compilation (per each rust crate)

In this way, the typescript reloading can be super fast and take advantage of HMR, and the Rust won't trigger false positives as the source changes and has compiler errors.

## Test, Build, Deploy

All via Github Actions - currently deploys to Github pages on the `gh-pages` branch :)
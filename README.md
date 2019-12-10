[![Build Status](https://github.com/dakom/wasm-app-boilerplate/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/wasm-app-boilerplate/actions)

## [LIVE DEMO](https://dakom.github.io/wasm-app-boilerplate)

# What is it?

A boilerplate for high-performance web apps...

It's kinda like this:

* Rust/WASM (the heavy lifting)
  * [Shipyard ECS](https://github.com/leudz/shipyard) (entity component system)
  * [awsm_web](https://github.com/dakom/awsm/tree/master/crates/web) (webgl, audio, main loop)
* Typescript (dom/ui and message passing)
  * [XState](https://github.com/davidkpiano/xstate) (statecharts)
  * [Lit-HTML](https://github.com/Polymer/lit-html) (html rendering)

Or, as a picture:

![flowchart](https://i.imgur.com/R9D7YJa.png)

# Development

If rust (nightly), npm, and all the prerequisites below are in place- it's simply `npm install` and then `npm start`. Might be nicer to run as `npm start --silent` to get just the meat and potatoes.

## Prerequisites

The prerequisites are a bit of work to setup, but the good news is it's one-time only and overlaps with _any_ rust/wasm project

1. Add `wasm32-unknown-unknown` target to the `nightly` toolchain: `rustup target add wasm32-unknown-unknown --toolchain nightly`
2. `wasm-opt` must be made available on the PATH (for release mode only): Download from [binaryen releases](https://github.com/WebAssembly/binaryen/releases)
3. `watchexec` must be installed (for dev mode only): `cargo install watchexec`

Release builds can be packaged via `npm run bundle:local`, but it is also setup to bundle and deploy everything via github actions (see [.github/workflows/build.yml](.github/workflows/build.yml))

# Files and directories

* [typescript](typescript) - the entry point, event bridging, and dom/ui 
* [crates/core/src](crates/core/src) - the game core (driven by [shipyard ECS](https://github.com/leudz/shipyard) and [awsm_web](https://github.com/dakom/awsm))
* [crates/shared/src](crates/fractal/src) - fractal generator for worker testing 
* [_static](_static) - worker shim and static media 

# Shared types for Events

We want the event types to be checked by the compiler on both the Typescript and Rust side.

This happens through a few design decisions and a generic test:

1. Events are split into the event type (`BridgeEvent`) and the event data.
2. `BridgeEvent` is a simple index-based enum in both Rust and Typescript
3. The exact order and names must match in both languages
4. A [generic test](typescript/tests/events.spec.ts) ensures that it is all correct. 

In order to ensure that the correct data type is sent for the event type, it's handled on each language:

* Typescript: specify the pairs in `ValidEvent` and it's checked statically on the language level.
* Rust: use regular Rust types, and they are turned into `BridgeEvent` just before sending via a single `match`.

Additionally the events are (de)serialized on the Rust side with serde and derive macros.

# Managing application state in WASM

The core mechanism in the demo here is the [Shipyard Enity Component System](https://github.com/leudz/shipyard), and with just a bare minimum example needed to kick the tires where they need kickin'.

The ECS could easily be composed with a different approach altogether - functional reactive programming, statecharts, etc.

Whatever it is, it's updated in a game loop - and ultimately processes input, produces output, handles events, and keeps state.

# WebGL / Audio

WebGl uses [awsm_web](https://github.com/dakom/awsm/tree/master/crates/web) but it's kept to a very small proof of concept here. Of course, sky's the limit!

Audio uses straight web-sys/bindgen/js-sys etc. One gotcha is that user interaction must happen before an AudioContext can be created, and an AudioContext is required for loading AudioBuffers. That's setup to work properly here via the start button callback (which does _not_ hold back the renderer from loading images).

# Workers

The messaging is done through a small js shim which also gets around some bundling issues.

In order to pick something interesting but slow, the demo includes a mandelbrot fractal renderer that was sitting from some old project

On slower machines it may take a while to see the images update. On faster machines it might be too fast to show the benefit of a worker ;)

# HTML rendering of ui state

The DOM is re-rendered via lit-html every frame-tick. This is fast (relatively speaking) since lit-html doesn't need to diff (short summary - the dynamic parts only need to check against _themselves_ to see if they're dirty, and if they are they know where and what to write)

### Ui State vs. DOM state 

The following really applies everywhere, but it's more of a footgun when it comes to UI:

**The rule of thumb is that asynchronous callbacks should never depend on a global `get_state()`. Either use a locally cached copy or get it from the element.**

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


## Watching 

There are multiple processes that run in parallel and both are launched at `npm start`:

1. Webpack (with various settings): for typescript, core bundling, and static folder changes
2. Watchexec -> npm -> rustc/wasm_bindgen/etc: for rust compilation (per each rust crate)

Webpack is configured to watch for changes in the static dir (this is a speedup compared to having webpack watch rust sources).

The typescript reloading can be super fast and take advantage of HMR, while the Rust won't trigger false positives as the source changes and has compiler errors.

## Test, Build, Deploy

All via Github Actions - currently deploys to Github pages on the `gh-pages` branch :)
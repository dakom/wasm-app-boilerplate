[![Build Status](https://travis-ci.org/dakom/wasm-app-boilerplate.svg?branch=master)](https://travis-ci.org/dakom/wasm-app-boilerplate)

## [LIVE DEMO](https://dakom.github.io/wasm-app-boilerplate)

# NOTE - NOT READY YET! COME BACK IN A WEEK OR SO :)

# Dataflow
![flowchart](https://i.imgur.com/u4GFKsM.png)

# Preliminary note

There's a lot going on here compared to a typical web project....

The point of this boiler plate is to not have to think about it at all. The info below is just in case ;)

To get started, run `npm start --silent` and then open an IDE in your favorite directory (or the project as a whole) and go for it.

# Directory structure

Don't mess with it. Just expand from it. media goes in `_static/media`. Sources go where they go.

# Shared types for Events and State

We want the events and state to be checked by the compiler on both the Typescript and Rust side, and this gets a little tricky since they are also shared between crates and modules.

The shared locations are:

* Rust state and events: [shared crate](crates/shared/src) 
* Typescript events only: [shared folder](typescript/shared)

The reason the typescript state is not in shared is because there is only one destination for typescript state: ui

More generally, events are all unified as CoreEvent (i.e. they are sent TO only one place), while state is different for each destination (e.g. renderer, ui, audio)

Keeping all this in sync takes a manual change - i.e. creating a new event or state on the Rust side means changing it on the TS side

This is somewhat annoying, but short of using a language-agnostic solution like flatbuffers (which would introduce its own issues), it's necessary

Also, to keep things idiomatic in both languages, events are enums that own their data when going from Rust, and index-based enums when going from JS

Fun!

It will seem super complicated at first, but it's really not so bad thanks to the type checking - edit the shared folders and let the compiler be your guide :)

(serialization / deserialization is via serde Rust <> JS Objects... if flatbuffers were used, it could be an optimization here due to Transferable objects, or it could be a performance cost due to the runtime impact. Profile and see before pulling the trigger!)

# Managing application state in WASM

The core mechanism in the demo here is the Shipyard Enity Component System, and with just a bare minimum example needed to shuttle the events and state back and forth across all areas.

The ECS could easily be swapped with a different approach altogether - functional reactive programming, statecharts, etc.

Whatever it is, it's updated internally in a game loop - and ultimately sends out discrete state updates for all the dependants (`ui`, `webgl renderer`, and `audio`)

These state updates are _not_ synonymous with the entire application state. Application state is conceptually just one giant thing (here, kept by the ECS).

The discrete updates don't need to be sent/extracted at the same frequency as eachother (e.g. the ui could be sent when events come in as opposed to a raf loop)

It might be tempting to consider completely different state managements, each in their own worker, but it's more likely that there needs to be some coherence between them (e.g. audio gets triggered on collision, UI sends an event to update inventory, etc.) so there is one "Source of Truth" and the split is done before sending. 

# HTML rendering of ui state

The DOM is re-rendered via lit-html every frame-tick if there's a fresh ui state. This is fast since lit-html doesn't need to diff (short summary - the dynamic parts only need to check against _themselves_ to see if they're dirty, and if they are they know where and what to write)

`get_ui_state()` can be queried from anywhere, and `events` can be sent from anywhere. They have no inherent relationship to the dom hierarchy. 

In many apps, especially those that are most similar to traditional websites, it makes sense for application state and ui state to be the same thing- and indeed keeping them the same is very easy here. However, they are still technically separate, on purpose, since it happens often enough that they aren't 1:1, and I personally found coping with that reality very frustrating in several popular frameworks. 

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

# Requirements 

Some cargo binaries are expected to be there, like watchexec and wasm-bindgen-cli (installed via cargo install)

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

So there are multiple processes that run in parallel and both are launched at `npm start`:

1. Webpack (with various settings): for typescript, core bundling, and static folder changes
2. Watchexec -> npm -> rustc/wasm_bindgen/etc: for rust compilation (per each rust crate)

In this way, the typescript reloading can be super fast and take advantage of HMR, and the Rust won't trigger false positives as the source changes and has compiler errors.

## Deployment

The dev server used the static dir via a configuration option, at deployment its copied manually
This happens via the `cpy` npm script for local sanity checks, but it's a little finicky for production
Rather, shell scripts are used in Travis (or whatever CI) with the couple lines to copy the required folders into `dist`

The production settings for both typescript and rust are optimized for the deployment version too.
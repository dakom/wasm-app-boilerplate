use cfg_if::cfg_if;
use log::{Level};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_log", debug_assertions))] {
        fn init_log() {
            use console_log;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_error_panic_hook", debug_assertions))] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run(on_pixels: js_sys::Function) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    //TODO - put this in a setInterval loop
    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _request_update = Box::new({
        move |timestamp: f64| {
            {            
                let this = JsValue::NULL;
                let output = format!("timestamp {}", timestamp);
                on_pixels.call1(&this, &serde_wasm_bindgen::to_value(&output).unwrap()).unwrap();
            }
        }
    }) as Box<dyn FnMut(f64) -> ()>;

    let _request_update = Closure::wrap(_request_update);

    let request_update = _request_update.as_ref().clone();

    //forget the things that need to persist in memory 
    _request_update.forget();

    Ok(request_update)
}
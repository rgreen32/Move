mod engine;
mod stop_watch;
mod body;
mod collision;
mod grid;
mod geometry;
mod renderer;
mod simulation_runner;
use wasm_bindgen::prelude::*;
use crate::geometry::{Edge, Point, Bounds};
use crate::body::Body;



// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_object(obj: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_num(num: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_bool(num: bool);
}


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}


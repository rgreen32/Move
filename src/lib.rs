use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::{Serialize, Deserialize};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_object(obj: &JsValue);

    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_num(num: u32);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world! tesssies"));

    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Test{
    pub test: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Point{
    x: f32,
    y: f32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Edge{
    a: Point,
    b: Point
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Body{
    pub distanceX: i32, 
    pub distanceY: i32,
    pub mass: i32,
    pub height: i32,
    pub width: i32,
    pub initialVelocity: i32,
    pub isStatic: bool,
    pub points: Vec<Point>,
    pub angle: i32,
    pub sides: i32,
    pub transformedPoints: Vec<Point>,
    pub transformedEdges: Vec<Edge>
}

#[wasm_bindgen]
pub fn test_function(string: &str){
    log(&format!("This is from Rust: {}.", string))
}

#[wasm_bindgen]
pub fn detect_collision_SAT(val: &JsValue){
    log("This is from SAT function!.");
    let test_obj: Body = val.into_serde().unwrap();
    // println!("{:?}", test_obj.test);
    log(&format!("{:?}", test_obj));
}



use core::panic;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
use crate::engine::Engine;
use wasm_bindgen::JsCast;
use crate::{log, log_num};

#[wasm_bindgen]
pub struct Renderer {
    // canvas: &HtmlCanvasElement,
    engine: Engine,
    ctx: CanvasRenderingContext2d,
    windowRatio: u32,
    Y_AxisDistance: u32,
    X_AxisDistance: u32,
    heightRatio: u32,
    widthRatio: u32
}

#[wasm_bindgen]
impl Renderer {
    //How to pass javascript canvas element as param?
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, engine: Engine) -> Renderer{
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();


        let width: u32 = match window.inner_width() {
            Ok(JsValue) => JsValue.into_serde().unwrap(),
            Err(error) => panic!("problem getting window width..")
        };


        let height: u32 = match window.inner_height() {
            Ok(JsValue) => JsValue.into_serde().unwrap(),
            Err(error) => panic!("problem getting window height..")
        };

        canvas.set_height(height);
        canvas.set_width(width);
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        Renderer {
            engine: engine, 
            ctx: ctx, 
            windowRatio: canvas.width(), 
            Y_AxisDistance: 100, 
            X_AxisDistance: (100 * (canvas.width()/canvas.height())),
            heightRatio: (canvas.height()/100), 
            widthRatio: (canvas.width()/2/(100 * (canvas.width()/canvas.height())))}
    }
}
use wasm_bindgen::prelude::*;
use crate::renderer::Renderer;
use crate::engine::Engine;
use crate::body::Body;
use core::panic;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
use crate::collision::CollisionDetector;
use wasm_bindgen::JsCast;
// use serde::{Serialize, Deserialize};


#[wasm_bindgen]
pub struct SimulationRunner {
    renderer: Renderer
}

#[wasm_bindgen]
impl SimulationRunner{
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, bodies: &JsValue) -> SimulationRunner {

        let window = window().unwrap();
        let document = window.document().unwrap();

        let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let canvas_width: u32 = match window.inner_width() {
            Ok(JsValue) => JsValue.into_serde().unwrap(),
            Err(error) => panic!("problem getting window width..")
        };
        let canvas_height: u32 = match window.inner_height() {
            Ok(JsValue) => JsValue.into_serde().unwrap(),
            Err(error) => panic!("problem getting window height..")
        };
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);


        let ctx: CanvasRenderingContext2d = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        let bodies: Vec<Body> = bodies.into_serde().unwrap();
        let engine = Engine {time_delta_root: js_sys::Date::now(), bodies: bodies, collision_detector: CollisionDetector{}};

        let renderer = Renderer {
            canvas_id: String::from(canvas_id),
            engine: engine, 
            ctx: ctx, 
            windowRatio: canvas.width(), 
            Y_AxisDistance: 100, 
            X_AxisDistance: (100 * (canvas.width()/canvas.height())),
            heightRatio: (canvas.height()/100), 
            widthRatio: (canvas.width()/2/(100 * (canvas.width()/canvas.height()))),
            canvas_width: canvas_width,
            canvas_height: canvas_height
        };

        renderer.run();


        return SimulationRunner{renderer: renderer}
    }
}
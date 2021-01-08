use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};
use crate::{renderer::Renderer};
use crate::engine::Engine;
use crate::body::Body;
use crate::grid::Grid;
use core::panic;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
use crate::collision::CollisionDetector;
use wasm_bindgen::JsCast;
use std::vec::Vec;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().expect("no global `window` exists") // is there a way to avoid calling this getter for every animation frame request?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub struct SimulationRunner {
    renderer: Renderer /*I want to store the Engine and Grid structs here (even by reference maybe) 
                        but wasm_bindgen limitations at the moment wont allow for it. -1/6/2021 */ 
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
        let window_width: u32 = match window.inner_width() {
            Ok(window_width) => window_width.into_serde().unwrap(),
            Err(_) => panic!("problem getting window width..")
        };
        let window_height: u32 = match window.inner_height() {
            Ok(window_height) => window_height.into_serde().unwrap(),
            Err(_) => panic!("problem getting window height..")
        };
        canvas.set_width(window_width);
        canvas.set_height(window_height);


        let ctx: CanvasRenderingContext2d = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        let bodies: Vec<Body> = bodies.into_serde().unwrap();

        let updated_bodies: Vec<Body> = bodies.into_iter().map(|mut x: Body| {
            x.init();
            return x;
        }).collect::<Vec<Body>>();

        
        let engine = Engine {time_delta_root: js_sys::Date::now(), bodies: updated_bodies, collision_detector: CollisionDetector{}};

        let mut grid = Grid::new(10, canvas.width() as f32, canvas.height() as f32);
        grid.initialize_grid();
        
        let renderer = Renderer::new(window, String::from(canvas_id), grid, engine, ctx, canvas.width(), canvas.height());

        return SimulationRunner{renderer: renderer}
    }

    pub fn start(mut self){
        self.renderer.run();

        // let f = Rc::new(RefCell::new(None));
        // let g = f.clone();
        // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        //     self.renderer.run();
        //     request_animation_frame(f.borrow().as_ref().unwrap());
        // }) as Box<dyn FnMut()>));

        // request_animation_frame(g.borrow().as_ref().unwrap());

    }
}
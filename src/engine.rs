use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::body::{Body};
use crate::collision::CollisionDetector;
// #[wasm_bindgen]
pub struct Engine {
    pub time_delta_root: f64,
    pub bodies: Vec<Body>,
    pub collision_detector: CollisionDetector
}


// #[wasm_bindgen]
impl Engine {
    // #[wasm_bindgen(constructor)]
    pub fn new(bodies: &JsValue) -> Engine{
        let bodies: Vec<Body> = bodies.into_serde().unwrap();
        Engine {time_delta_root: js_sys::Date::now(), bodies: bodies, collision_detector: CollisionDetector{}}
    }

    fn calculate_displacement(&mut self, body: &Body) -> f32 {
        // let body: Body = body.into_serde().unwrap();
        let now = js_sys::Date::now();
        let time_delta = ((now - self.time_delta_root))/(1000 as f64);
        let displacement = ((body.initialVelocity as f64) * time_delta + (0.5*-9.8*time_delta.powi(2)));
        // self.time_delta_root = now;
        // body.distanceY += displacement;
        // body.update();

        return displacement as f32;
    }

    // pub fn get_bodies(&self) -> &Vec<Body>{
        // let bodies = self.bodies.into_iter().map(JsValue::from).collect();
        // return bodies
        // return &self.bodies;
    // }

    pub fn run(&mut self){
        self.collision_detector.run(&self.bodies);
        for mut body in &mut self.bodies.iter_mut(){
            if body.isStatic == true {
                continue;
            }
            let now = js_sys::Date::now();
            let time_delta = ((now - self.time_delta_root))/(1000 as f64);
            let displacement = ((body.initialVelocity as f64) * time_delta + (0.5*-9.8*time_delta.powi(2)));
            // let displacement = Self::calculate_displacement(&mut self, &body);
            body.distanceY += displacement as f32;

            
        }

    }
}

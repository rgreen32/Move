use wasm_bindgen::prelude::*;
use crate::{body::{self, Body}, log_bool};
use crate::collision::CollisionDetector;
use crate::{log};
pub struct Engine {
    pub time_delta_root: f64,
    pub bodies: Vec<Body>,
    pub collision_detector: CollisionDetector
}


impl Engine {

    fn calculate_displacement(&mut self, body: &Body) -> f32 {
        let now = js_sys::Date::now();
        let time_delta = ((now - self.time_delta_root))/(1000 as f64);
        let displacement = ((body.initialVelocity as f64) * time_delta + (0.5*-9.8*time_delta.powi(2)));
        // self.time_delta_root = now;
        // body.distanceY += displacement;
        // body.update();

        return displacement as f32;
    }

    pub fn run(&mut self){
        self.collision_detector.run(&mut self.bodies);
        for mut body in &mut self.bodies.iter_mut(){
            if body.isStatic == true {
                continue;
            }
            let now = js_sys::Date::now();
            let time_delta = ((now - self.time_delta_root))/(1000 as f64);
            let displacement = ((body.initialVelocity as f64) * time_delta + (0.5*-9.8*time_delta.powi(2)));  //Calculating displacement here because i cant figure out call method that uses mutable self reference from another method that uses a mutable self reference.
            // let displacement = Self::calculate_displacement(&mut self, &body);
            body.distanceY += displacement as f32;
           
            body.update();
            
        }

    }
}

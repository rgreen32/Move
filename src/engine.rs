use crate::body::{self, Body};
use crate::stop_watch::StopWatch;
use crate::collision::CollisionDetector;
use crate::log;

pub struct Engine {
    pub stop_watch: StopWatch,
    pub bodies: Vec<Body>,
    pub collision_detector: CollisionDetector
}

impl Engine {

    pub fn run(&mut self){
        self.collision_detector.run(&self.bodies);
        for mut body in &mut self.bodies.iter_mut(){
            if body.is_static == true {
                continue;
            }
            // Apply gravity (want to move to separate method but cant because lifetime stuff not supported in wasm_bindgen)
            let time_delta = self.stop_watch.get_time_since_last_called();
            let (old_position_x, position_x) = Engine::apply_verlet_integration(body.old_position_x, body.position_x, time_delta as f32, 0.0);
            let (old_position_y, position_y) =  Engine::apply_verlet_integration(body.old_position_y, body.position_y, time_delta as f32, -9.8);
            body.old_position_x = old_position_x;
            body.old_position_y = old_position_y;
            body.position_x = position_x;
            body.position_y = position_y;
            // body.velocity_magnitude = updated_velocity_magnitude as f32;
            // body.velocity_magnitude = (body.velocity_magnitude as f64 + (2.0*-9.8*displacement_y).sqrt()) as f32;
            
            body.update();
            
        }

    }

    fn apply_verlet_integration(old_position: f32, position: f32, dt: f32, acceleration: f32) -> (f32, f32) {
        let new_position = position * 2.0 - old_position + (acceleration * dt * dt);
        return (position, new_position)
    }
}
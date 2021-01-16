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
            log(&format!("Time delta: {:?}", time_delta));
            let displacement_x = ((body.velocity_magnitude*body.velocity_angle.cos()) as f64 * time_delta) + (0.5*0.0*time_delta.powi(2));
            let displacement_y = ((body.velocity_magnitude*body.velocity_angle.sin()) as f64 * time_delta) + (0.5*-9.8*time_delta.powi(2));
            
            let updated_x_velocity = ((body.velocity_magnitude*body.velocity_angle.cos()) as f64 + (2.0*0.0*displacement_x).sqrt());
            let updated_y_velocity = ((body.velocity_magnitude*body.velocity_angle.sin()) as f64 + (2.0*-9.8*displacement_y).sqrt()); //How do i preserve the sign?
            log(&format!("Y Displacement: {:?}", displacement_y));
            log(&format!("Updated Y Velocity: {:?}", updated_y_velocity));
            let updated_velocity_magnitude = (updated_x_velocity.powi(2) + updated_y_velocity.powi(2)).sqrt();
            // let y_component = 
            let updated_velocity_angle = -updated_y_velocity.asin(); //hard set to negative for testing  TODO: updated_velocity_angle getting set to NaN on third request_animation loop iteration. 
            log(&format!("Updated Velocity_Angle: {:?}", updated_velocity_angle));
            log(&format!("Updated Velocity_Magnitude: {:?}", updated_velocity_magnitude));
            body.distance_x += displacement_x as f32;
            body.distance_y += displacement_y as f32;
            body.velocity_angle = updated_velocity_angle as f32;
            body.velocity_magnitude = updated_velocity_magnitude as f32;
            // body.velocity_magnitude = (body.velocity_magnitude as f64 + (2.0*-9.8*displacement_y).sqrt()) as f32;
            
            body.update();
            
        }

    }

    // fn apply_gravity(&self, body: &mut Body){
    //     let time_delta = self.stop_watch.get_time_since_last_called();
    //     let displacement = ((body.velocity_magnitude*body.velocity_angle.sin()) as f64 * time_delta) + (0.5*-9.8*time_delta.powi(2));  //Calculating displacement here because i cant figure out how to call method that uses mutable self reference from another method that uses a mutable self reference.
    //     body.distance_y += displacement as f32;
    //     body.velocity_magnitude = (body.velocity_magnitude as f64 + (2.0*-9.8*displacement).sqrt()) as f32;
    // }
}

use std::{f64::INFINITY, time::SystemTime};
// use chrono::{DateTime, offset};
// use std::time::{Duration, Instant};
use std::time::{Instant, UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
// use js_sys::*;


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
    fn log_num(num: u64);
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
pub struct Bounds {
    min: f64,
    max: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dot(&self, point: &Point) -> f64 {
        let scalar = (self.x * point.x) + (self.y * point.y);
        return scalar;
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    a: Point,
    b: Point,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub distanceX: f32,
    pub distanceY: f32,
    pub mass: f32,
    pub height: f32,
    pub width: f32,
    pub initialVelocity: f32,
    pub isStatic: bool,
    pub points: Vec<Point>,
    pub angle: f32,
    pub sides: u32,
    pub transformedPoints: Vec<Point>,
    pub transformedEdges: Vec<Edge>,
}

// impl Body {
//     fn update(&self){

//     }
// }

pub fn createAxisFromEdge(edge: &Edge) -> Point {
    let axis_proj = Point {
        x: -(edge.b.y - edge.a.y),
        y: (edge.b.x - edge.a.x),
    };
    return axis_proj;
}

pub fn bounds_overlap(body1_bounds: &Bounds, body2_bounds: &Bounds) -> bool {
    if (body1_bounds.min < body2_bounds.max && body1_bounds.min > body2_bounds.min)
        || (body1_bounds.max > body2_bounds.min && body1_bounds.max < body2_bounds.max)
        || (body2_bounds.min < body1_bounds.max && body2_bounds.min > body1_bounds.min)
        || (body2_bounds.max > body1_bounds.min && body2_bounds.max < body1_bounds.max)
    {
        return true;
    } else {
        return false;
    }
}

#[wasm_bindgen]
pub struct Engine {
    time_delta_root: f64,
    bodies: Vec<Body>
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine{
        Engine {time_delta_root: js_sys::Date::now(), bodies: vec![]}
    }

    pub fn calculate_displacement(&mut self, body: &JsValue) -> f64 {
        let body: Body = body.into_serde().unwrap();
        let now = js_sys::Date::now();
        let time_delta = ((now - self.time_delta_root))/(1000 as f64);
        let displacement = ((body.initialVelocity as f64) * time_delta + (0.5*-9.8*time_delta.powi(2)));
        // self.time_delta_root = now;
        // body.distanceY += displacement;
        // body.update();

        return displacement as f64;
    }
}


#[wasm_bindgen]
pub fn detect_collision_SAT(body1: &JsValue, body2: &JsValue) -> bool {
    // log("This is from SAT function.");
    let body1: Body = body1.into_serde().unwrap();
    let body2: Body = body2.into_serde().unwrap();

    let mut collision = true;
    for edge in body1.transformedEdges.iter() {
        // log(&format!("{:?}", edge));
        let axis_proj = createAxisFromEdge(edge);
        let mut body1_min = INFINITY;
        let mut body1_max = -INFINITY;
        let mut body2_min = INFINITY;
        let mut body2_max = -INFINITY;
        for point in body1.transformedPoints.iter() {
            let scalar = axis_proj.dot(point);
            if scalar < body1_min {
                body1_min = scalar
            }
            if scalar > body1_max {
                body1_max = scalar
            }
        }
        for point in body2.transformedPoints.iter() {
            let scalar = axis_proj.dot(point);
            if scalar < body2_min {
                body2_min = scalar;
            }
            if scalar > body2_max {
                body2_max = scalar;
            }
        }
        let body1_bounds = Bounds {
            min: body1_min,
            max: body1_max,
        };
        let body2_bounds = Bounds {
            min: body2_min,
            max: body2_max,
        };
        let bounds_overlap = bounds_overlap(&body1_bounds, &body2_bounds);
        if !bounds_overlap {
            collision = false;
            break;
        }
    }
    //project axis on to second body
    for edge in body2.transformedEdges.iter() {
        // log(&format!("{:?}", edge));
        let axis_proj = createAxisFromEdge(edge);
        let mut body1_min = INFINITY;
        let mut body1_max = -INFINITY;
        let mut body2_min = INFINITY;
        let mut body2_max = -INFINITY;
        for point in body1.transformedPoints.iter() {
            let scalar = axis_proj.dot(point);
            if scalar < body1_min {
                body1_min = scalar
            }
            if scalar > body1_max {
                body1_max = scalar
            }
        }
        for point in body2.transformedPoints.iter() {
            let scalar = axis_proj.dot(point);
            if scalar < body2_min {
                body2_min = scalar;
            }
            if scalar > body2_max {
                body2_max = scalar;
            }
        }
        let body1_bounds = Bounds {
            min: body1_min,
            max: body1_max,
        };
        let body2_bounds = Bounds {
            min: body2_min,
            max: body2_max,
        };
        let bounds_overlap = bounds_overlap(&body1_bounds, &body2_bounds);

        if !bounds_overlap {
            collision = false;
            break;
        }
    }

    return collision;
}

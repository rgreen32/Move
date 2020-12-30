use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Bounds {
    pub min: f64,
    pub max: f64,
}
#[wasm_bindgen]
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn dot(&self, point: &Point) -> f64 {
        let scalar = (self.x * point.x) + (self.y * point.y);
        return scalar;
    }
}
#[derive(Debug, Deserialize)]
pub struct Edge{
    pub a: Point,
    pub b: Point,
}

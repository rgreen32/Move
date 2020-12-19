use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bounds {
    pub min: f64,
    pub max: f64,
}
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub a: Point,
    pub b: Point,
}

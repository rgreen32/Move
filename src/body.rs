use serde::{Deserialize, Serialize};
use crate::geometry::{Point, Edge};
// use super::geometry::{Bounds, Edge, Point};

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
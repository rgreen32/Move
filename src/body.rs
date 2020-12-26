use serde::{Deserialize, Serialize};
use crate::geometry::{Point, Edge};
use std::f64::consts::{PI};
use libm::{cos, sin};
use std::iter::FromIterator;


#[derive(Deserialize)]
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
    pub transformedEdges: Vec<Edge>
}

impl Body{

    pub fn init(&mut self){
        self.calculate_shape_vectors();
        self.calculate_transformed_shape_vectors();
        self.calculate_transformed_edges();
    }

    pub fn update(&mut self){
        self.calculate_transformed_shape_vectors();
        self.calculate_transformed_edges();
    }

    fn calculate_shape_vectors(&self){
        let theta = 360/self.sides;
        let r = self.width;
        let mut points: Vec<Point> = vec![];
        for i in 0..self.sides{
            let xcomponent = cos(((theta*i) as f64 + self.angle as f64)) * (PI/180.0);
            let resultx = r as f64 * xcomponent;
            let ycomponent = sin(((theta*i) as f64 + self.angle as f64)) * (PI/180.0);
            let resulty = r as f64 * ycomponent;
            points.push(Point{x: resultx, y: resulty});
        }
    }

    fn calculate_transformed_shape_vectors(&mut self){
        let mut points: Vec<Point> = vec![];
        let origin = Point{x: self.distanceX as f64, y: self.distanceY as f64};
        for point in self.points.iter(){
            let transformedPoint = Point{x: origin.x + point.x, y: origin.y + point.y};
            points.push(transformedPoint);
        }
        self.transformedPoints = points;
    }

    fn calculate_transformed_edges(&self){
        let mut edges: Vec<Edge> = vec![];
        for (i, point) in self.transformedPoints.iter().enumerate(){
            let pointA: Point = point.clone();
            let pointB = self.transformedPoints[(i + 1) % self.transformedPoints.len()];
            let edge = Edge{a: pointA, b: pointB};
        }
    }
}

// pub struct MyCollection(Vec<Body>);

// impl MyCollection{
//     fn new() -> MyCollection {
//         MyCollection(Vec::new())
//     }

//     fn add(&mut self, elem: Body) {
//         self.0.push(elem)
// }
// }

// impl FromIterator<Body> for MyCollection{
//     fn from_iter<T: IntoIterator<Item = Body>>(iter: T) -> Self {
//         let mut c = MyCollection::new();
//         for i in iter {
//             c.add(i);
//         }
//         return c;
//     }
// }
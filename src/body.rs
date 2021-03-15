use serde::Deserialize;
use crate::geometry::{Point, Edge};
use std::f64::consts::{PI};
use libm::{cos, sin};

#[derive(Debug, Deserialize)]
pub struct Body {
    pub position_x: f32,
    pub position_y: f32,
    pub old_position_x: f32,
    pub old_position_y: f32,
    pub mass: f32,
    pub height: f32,
    pub width: f32,
    pub spatial_mask: Vec<String>,
    pub is_static: bool,
    pub points: Vec<Point>,
    pub orientation_angle: f32,
    pub sides: u32,
    pub transformed_points: Vec<Point>,
    pub transformed_edges: Vec<Edge> // for SAT collision detection
}

impl Body{

    pub fn init(&mut self){ //Have to generate these values 
        self.points = self.calculate_shape_vectors();
        self.transformed_points = self.calculate_transformed_shape_vectors();
        self.transformed_edges = self.calculate_transformed_edges();
    }

    pub fn update(&mut self){
        self.transformed_points = self.calculate_transformed_shape_vectors();
        self.transformed_edges = self.calculate_transformed_edges();
    }

    fn calculate_shape_vectors(&mut self) -> Vec<Point> {
        let theta = 360/self.sides;
        let r = self.width/2.0;
        let mut points: Vec<Point> = vec![];
        for i in 0..self.sides{
            let xcomponent = cos(((theta*i) as f64 + self.orientation_angle as f64) * (PI/180.0)) ;
            let resultx = r as f64 * xcomponent;
            let ycomponent = sin(((theta*i) as f64 + self.orientation_angle as f64) * (PI/180.0)) ;
            let resulty = r as f64 * ycomponent;
            points.push(Point{x: resultx, y: resulty});
        }
        return points;
    }

    fn calculate_transformed_shape_vectors(&mut self) -> Vec<Point> { // translates shape points into points on coordinate plane
        let mut points: Vec<Point> = vec![];
        let origin = Point{x: self.position_x as f64, y: self.position_y as f64};
        for point in self.points.iter(){
            let transformed_point = Point{x: origin.x + point.x, y: origin.y + point.y};
            points.push(transformed_point);
        }
        return points;
    }

    fn calculate_transformed_edges(&mut self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = vec![];
        for (i, point) in self.transformed_points.iter().enumerate(){
            let point_a: Point = point.clone(); //want to make these points into references to transformed points but then i would need to add liftime annotations, which conflicts heavily with deserialize trait.
            let point_b = self.transformed_points[(i + 1) % self.transformed_points.len()];
            let edge = Edge{a: point_a, b: point_b};
            edges.push(edge);
        }
        return edges;
    }

}
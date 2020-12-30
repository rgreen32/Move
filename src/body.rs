use serde::Deserialize;
use crate::geometry::{Point, Edge};
use std::f64::consts::{PI};
use libm::{cos, sin};

#[derive(Debug, Deserialize)]
pub struct Body {
    pub distance_x: f32,
    pub distance_y: f32,
    pub mass: f32,
    pub height: f32,
    pub width: f32,
    pub initial_velocity: f32,
    pub is_static: bool,
    pub points: Vec<Point>,
    pub angle: f32,
    pub sides: u32,
    pub transformed_points: Vec<Point>,
    pub transformed_edges: Vec<Edge>
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

    fn calculate_shape_vectors(&mut self){
        let theta = 360/self.sides;
        let r = self.width/2.0;
        let mut points: Vec<Point> = vec![];
        for i in 0..self.sides{
            let xcomponent = cos(((theta*i) as f64 + self.angle as f64) * (PI/180.0)) ;
            let resultx = r as f64 * xcomponent;
            let ycomponent = sin(((theta*i) as f64 + self.angle as f64) * (PI/180.0)) ;
            let resulty = r as f64 * ycomponent;
            points.push(Point{x: resultx, y: resulty});
        }
        self.points = points;
    }

    fn calculate_transformed_shape_vectors(&mut self){
        let mut points: Vec<Point> = vec![];
        let origin = Point{x: self.distance_x as f64, y: self.distance_y as f64};
        for point in self.points.iter(){
            let transformed_point = Point{x: origin.x + point.x, y: origin.y + point.y};
            points.push(transformed_point);
        }
        self.transformed_points = points;
    }

    fn calculate_transformed_edges(&mut self){
        let mut edges: Vec<Edge> = vec![];
        for (i, point) in self.transformed_points.iter().enumerate(){
            let point_a: Point = point.clone(); //want to make these points into references to transformed points but then i would need to add liftime annotations, which conflicts heavily with deserialize trait.
            let point_b = self.transformed_points[(i + 1) % self.transformed_points.len()];
            let edge = Edge{a: point_a, b: point_b};
            edges.push(edge);
        }
        self.transformed_edges = edges;
    }

    fn calculate_spatial_mask(self){
        // let origin = Point{x: self.distanceX as f64, y: self.distanceY as f64};
        for point in self.transformed_points.iter(){
            
        }
    }
}
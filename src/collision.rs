use crate::Body;
use crate::{Edge, Point, Bounds};
use std::{f64::INFINITY};
pub struct CollisionDetector{

}

impl CollisionDetector{
    pub fn run(&self, bodies: &Vec<Body>) {
        let collision = CollisionDetector::detect_collision_SAT(bodies);
        if collision{
            // let body1 = &mut bodies[0];
            // let body2 = &mut bodies[1];
            // body2.is_static = true
        }
    }
    
    fn create_axis_from_edge(edge: &Edge) -> Point {
        let axis_proj = Point {
            x: -(edge.b.y - edge.a.y),
            y: (edge.b.x - edge.a.x),
        };
        return axis_proj;
    }



    fn detect_collision_SAT(bodies: &Vec<Body>) -> bool {
        let body1 = &bodies[0];
        let body2 = &bodies[1];

        let mut collision = true;
        for edge in body1.transformed_edges.iter() {
            let axis_proj = Self::create_axis_from_edge(edge);
            let mut body1_min = INFINITY;
            let mut body1_max = -INFINITY;
            let mut body2_min = INFINITY;
            let mut body2_max = -INFINITY;
            for point in body1.transformed_points.iter() {
                let scalar = axis_proj.dot(point);
                if scalar < body1_min {
                    body1_min = scalar
                }
                if scalar > body1_max {
                    body1_max = scalar
                }
            }
            for point in body2.transformed_points.iter() {
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
            let bounds_overlap = Self::bounds_overlap(&body1_bounds, &body2_bounds);
            if !bounds_overlap {
                collision = false;
                break;
            }
        }
        //project axis on to second body
        for edge in body2.transformed_edges.iter() {
            let axis_proj = Self::create_axis_from_edge(edge);
            let mut body1_min = INFINITY;
            let mut body1_max = -INFINITY;
            let mut body2_min = INFINITY;
            let mut body2_max = -INFINITY;
            for point in body1.transformed_points.iter() {
                let scalar = axis_proj.dot(point);
                if scalar < body1_min {
                    body1_min = scalar
                }
                if scalar > body1_max {
                    body1_max = scalar
                }
            }
            for point in body2.transformed_points.iter() {
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
            let bounds_overlap = Self::bounds_overlap(&body1_bounds, &body2_bounds);

            if !bounds_overlap {
                collision = false;
                break;
            }
        }

        return collision;
    }

    fn bounds_overlap(body1_bounds: &Bounds, body2_bounds: &Bounds) -> bool {
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


}
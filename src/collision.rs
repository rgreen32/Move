use crate::Body;
use crate::{Edge, Point, Bounds};
use std::{f64::INFINITY};
pub struct CollisionDetector{

}

impl CollisionDetector{
    pub fn run(&self, bodies: &Vec<Body>) {
        let body1 = &bodies[0];
        let body2 = &bodies[1];

    }
    
    fn createAxisFromEdge(edge: &Edge) -> Point {
        let axis_proj = Point {
            x: -(edge.b.y - edge.a.y),
            y: (edge.b.x - edge.a.x),
        };
        return axis_proj;
    }



    fn detect_collision_SAT(body1: &Body, body2: &Body) -> bool {
        // log("This is from SAT function.");
        // let body1: Body = body1.into_serde().unwrap();
        // let body2: Body = body2.into_serde().unwrap();

        let mut collision = true;
        for edge in body1.transformedEdges.iter() {
            // log(&format!("{:?}", edge));
            let axis_proj = Self::createAxisFromEdge(edge);
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
            let bounds_overlap = Self::bounds_overlap(&body1_bounds, &body2_bounds);
            if !bounds_overlap {
                collision = false;
                break;
            }
        }
        //project axis on to second body
        for edge in body2.transformedEdges.iter() {
            // log(&format!("{:?}", edge));
            let axis_proj = Self::createAxisFromEdge(edge);
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
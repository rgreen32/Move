use crate::{log, log_num};
use web_sys::{CanvasRenderingContext2d, Window};
use crate::body::Body;
use crate::engine::Engine;

pub struct Renderer {
    pub window: Window, //Research: what does it mean for a struct to own another struct versus hold a reference?
    pub canvas_id: String,
    pub engine: Engine,
    pub canvas_width: u32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: u32,
    pub ctx: CanvasRenderingContext2d,
    pub window_ratio: u32,
    pub y_axis_length_meters: u32,
    pub x_axis_length_meters: u32,
    pub height_ratio: u32,
    pub width_ratio: u32
}

impl Renderer {
    // #[wasm_bindgen(constructor)] //Why does compiler panic when self param is added here?

    pub fn run(&mut self){
        self.engine.run();
        self.ctx.clear_rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);
        self.draw_axis();
        for body in &self.engine.bodies{
            self.draw_shape(&body);
        }
    }


    fn draw_shape(&self, body: &Body){
        let points = &body.transformed_points;
        for (i, point_a) in points.iter().enumerate(){
            let point_b = &points[(i + 1) % points.len()];
            self.ctx.begin_path();
            self.ctx.move_to(self.meters_to_pixels_distance_x(point_a.x), self.meters_to_pixels_distance_y(point_a.y));
            self.ctx.line_to(self.meters_to_pixels_distance_x(point_b.x), self.meters_to_pixels_distance_y(point_b.y));
            self.ctx.stroke();
        }
    }

    fn meters_to_pixels_distance_y(&self, height: f64) -> f64{
        let distance_in_pixels = self.canvas_height as f64 - (self.height_ratio as f64 * height);
        return distance_in_pixels;
    }

    fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
        let distance_in_pixels: f64;
        if distance > 0.0 {
            distance_in_pixels = (self.canvas_width/2) as f64 + self.height_ratio as f64 * distance;
        }else if distance < 0.0 {
            let distance_from_origin_pixels = -(self.height_ratio as f64) * distance;
            distance_in_pixels = (self.canvas_width/2) as f64 - distance_from_origin_pixels;
        }else{
            distance_in_pixels = (self.canvas_width/2) as f64;
        }
        return distance_in_pixels; 
    }



    fn draw_axis(&self){
        let canvas_width = self.canvas_width as f64;
        let canvas_height = self.canvas_height as f64;
        
        //draw y axis
        self.ctx.begin_path();
        self.ctx.move_to(canvas_width/2.0, 0.0);
        self.ctx.line_to(canvas_width/2.0, canvas_height);
        self.ctx.stroke();

        //draw x axis
        self.ctx.move_to(0.0, canvas_height);
        self.ctx.line_to(canvas_width, canvas_height);
        self.ctx.stroke();

        let canvas_to_y_axis_ratio = (canvas_height)/self.y_axis_length_meters as f64;
        let tick_spacing = 10.0;
        let y_axis_number_of_ticks = canvas_height/(canvas_to_y_axis_ratio*tick_spacing);
        let x_axis_number_ticks = (canvas_width/2.0)/tick_spacing;
        let x_axis_center = canvas_width/2.0;

        for i in 0..y_axis_number_of_ticks as i32 {
            let tick_margin = i as f64*(tick_spacing*canvas_to_y_axis_ratio);
            let tick_x = x_axis_center;
            let tick_y = canvas_height - tick_margin;
            self.ctx.move_to(tick_x-10.0, tick_y);
            self.ctx.line_to(tick_x+10.0, tick_y);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), x_axis_center+15.0, tick_y+3.0); //is there a shorthand for this err handling?

        }

        for i in 0..x_axis_number_ticks as i32 {
            let tick_margin = i as f64*(tick_spacing*canvas_to_y_axis_ratio);
            let tick_x = x_axis_center + tick_margin;
            let tick_y = canvas_height;
            self.ctx.move_to(tick_x, tick_y);
            self.ctx.line_to(tick_x, tick_y-10.0);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (x_axis_center + tick_margin) -3.0, canvas_height-15 as f64);
        }

        for i in (0..x_axis_number_ticks as i32){
            let tick_margin = i as f64*(tick_spacing*canvas_to_y_axis_ratio);
            let tick_x = x_axis_center - tick_margin;
            let tick_y = canvas_height;
            self.ctx.move_to(tick_x, tick_y);
            self.ctx.line_to(tick_x, tick_y-10.0);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (x_axis_center - tick_margin) -3.0, canvas_height-15 as f64);
        }
    }
}
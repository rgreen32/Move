use core::panic;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, window};
use crate::{body::{self, Body}, engine::Engine};
use crate::{log, log_num};

pub struct Renderer {
    pub window: Window,
    pub canvas_id: String,
    pub engine: Engine,
    pub canvas_width: u32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: u32,
    pub ctx: CanvasRenderingContext2d,
    pub windowRatio: u32,
    pub Y_AxisDistance: u32,
    pub X_AxisDistance: u32,
    pub heightRatio: u32,
    pub widthRatio: u32
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
        let points = &body.transformedPoints;
        for (i, pointA) in points.iter().enumerate(){
            let pointB = &points[(i + 1) % points.len()];
            self.ctx.begin_path();
            self.ctx.move_to(self.meters_to_pixels_distance_x(pointA.x), self.meters_to_pixels_distance_y(pointA.y));
            self.ctx.line_to(self.meters_to_pixels_distance_x(pointB.x), self.meters_to_pixels_distance_y(pointB.y));
            self.ctx.stroke();
        }
    }

    fn meters_to_pixels_distance_y(&self, height: f64) -> f64{
        let distance_in_pixels = self.canvas_height as f64 - (self.heightRatio as f64 * height);
        return distance_in_pixels;
    }

    fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
        let distance_in_pixels: f64;
        if distance > 0.0 {
            distance_in_pixels = (self.canvas_width/2) as f64 + self.heightRatio as f64 * distance;
        }else if distance < 0.0 {
            let distance_from_origin_pixels = -(self.heightRatio as f64) * distance;
            distance_in_pixels = (self.canvas_width/2) as f64 - distance_from_origin_pixels;
        }else{
            distance_in_pixels = (self.canvas_width/2) as f64;
        }
        return distance_in_pixels; 
    }



    fn draw_axis(&self){
        let canvas_width = self.canvas_width as f64;
        let canvas_height = self.canvas_height as f64;
        self.ctx.begin_path();
        self.ctx.move_to(canvas_width/2.0, 0.0);
        self.ctx.line_to(canvas_width/2.0, canvas_height);
        self.ctx.stroke();

        self.ctx.move_to(0.0, canvas_height);
        self.ctx.line_to(canvas_width, canvas_height);
        self.ctx.stroke();

        let ratio = (canvas_height)/self.Y_AxisDistance as f64;

        let mut count = canvas_height;
        let mut i = 0;
        while count > 0.0 {
            self.ctx.move_to((canvas_width/2.0)-10.0, count);
            self.ctx.line_to((canvas_width/2.0)+10.0, count);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (canvas_width/2.0)+15.0, count+3.0); 
            count-= ratio*10.0;
            i+=1;
        }

        let mut count: f64 = 0.0;
        let mut i = 0;
        while count < canvas_width/2.0 {
            self.ctx.move_to((canvas_width/2.0)+count as f64, canvas_height);
            self.ctx.line_to((canvas_width/2.0)+count as f64, canvas_height-10.0);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (((canvas_width/2.0)+count as f64)-3 as f64), canvas_height-15 as f64); 
            count+= ratio*10.0;
            i+=1;
        }
        let mut count: f64 = 0.0;
        let mut i = 1;
        while(count > -canvas_width/2.0){
            self.ctx.move_to((canvas_width/2.0)+count as f64, canvas_height);
            self.ctx.line_to((canvas_width/2.0)+count as f64, canvas_height-10.0);
            self.ctx.stroke();
            count-= ratio*10.0;
            self.ctx.fill_text(&i.to_string(), ((canvas_width/2.0)+count as f64)-3 as f64, canvas_height-15 as f64); 
            i+=1;
        }
    }
}
use crate::{grid::{Cell, Grid}, log, log_num};
use web_sys::{CanvasRenderingContext2d, Window};
use crate::body::Body;
use crate::engine::Engine;

pub struct Renderer {
    pub window: Window, //Research: what does it mean for a struct to own another struct versus hold a reference?
    pub canvas_id: String,
    pub grid: Grid,
    pub engine: Engine,
    pub canvas_width: u32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: u32,
    pub ctx: CanvasRenderingContext2d,
    pub window_ratio: u32,
    pub y_axis_length_meters: u32,
    pub x_axis_length_meters: u32,
    pub height_ratio: f64,
    pub width_ratio: u32
}

impl Renderer {
    // #[wasm_bindgen(constructor)] //Why does compiler panic when self param is added here?
    
    pub fn run(&mut self){
        self.engine.run();
        self.ctx.clear_rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);
        self.draw_axis();
        self.draw_grid();
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


    fn draw_grid(&self){
        for (i, cell) in self.grid.map.iter().enumerate(){
            if i == 1{
                log(&format!("cell lower Y position: {:?}", cell.position_y));
                log(&format!("cell lower Y pixel position: {:?}", self.meters_to_pixels_distance_y(cell.position_y as f64)));
                self.draw_cell(cell);
            }
            /*
            TODO Figure out why meter_to_pixel_conversion not yielding same results for grid cell and axis ticks.
                "First tick Y position: 869.4
                cell lower Y pixel position: 876.0"
​            */
        }
    }

    fn draw_cell(&self, cell: &Cell){
        self.ctx.begin_path();
        self.ctx.move_to(
            self.meters_to_pixels_distance_x(cell.position_x as f64), 
            self.meters_to_pixels_distance_y(cell.position_y as f64)
        );
        self.ctx.line_to(
            self.meters_to_pixels_distance_x((cell.position_x + 10) as f64), 
            self.meters_to_pixels_distance_y(cell.position_y as f64)
        );
        self.ctx.stroke();

    }

    fn draw_axis(&self){ // needs optimization. slowing down rendering
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
        log(&format!("Canvas ratio: {:?}", (canvas_height)/self.y_axis_length_meters as f64));
        let y_axis_number_of_ticks = canvas_height/(canvas_to_y_axis_ratio* tick_spacing) ;
        let x_axis_number_ticks = (canvas_width/2.0)/tick_spacing;
        let x_axis_center = canvas_width/2.0;
        log(&format!("Canvas height: {:?}", canvas_height));
        // log(&format!("y_axis_number_of_ticks: {:?}", y_axis_number_of_ticks));
        for i in 0..y_axis_number_of_ticks as i32 {
            let tick_margin = i as f64*(tick_spacing*canvas_to_y_axis_ratio);
            if i ==1{
                log(&format!("First tick Y position: {:?}", canvas_height - tick_margin));
                log(&format!("meters_to_pixels function: {:?}", self.meters_to_pixels_distance_y(100.0)));
                log(&format!("tick_margin: {:?}", tick_margin));
            }

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
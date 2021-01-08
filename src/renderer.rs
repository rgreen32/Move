use crate::{body, grid::{Cell, Grid, Quadrant}, log, log_num};
use web_sys::{CanvasRenderingContext2d, Window};
use wasm_bindgen::JsValue;
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
    pub y_axis_length_meters: u32,
    pub canvas_pixels_to_meters_ratio: f64,
}

impl Renderer {
    // #[wasm_bindgen(constructor)] //Why does compiler panic when self param is added here?
    pub fn new(window: Window, canvas_id: String, grid: Grid, engine: Engine,
         ctx: CanvasRenderingContext2d, canvas_width: u32, canvas_height: u32) -> Renderer{
        Renderer{
            window,
            canvas_id,
            grid,
            engine,
            ctx,
            canvas_width,
            canvas_height,
            y_axis_length_meters: 100,
            canvas_pixels_to_meters_ratio: (canvas_height as f64)/100.0 // Its important canvas height is converted to float before being divided. otherwise the resulting number gets truncated and becomes in accurate.
        }
    }

    pub fn run(&mut self){
        self.engine.run();
        self.ctx.clear_rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);
        // self.draw_axis();
        self.draw_grid();
        for body in &self.engine.bodies{
            self.draw_shape(&body);
        }
        // self.draw_cell( &Cell { id: (Quadrant::Quadrant1, 0, 0), position_x: 0, position_y: 0, strokerect_x: 0, strokerect_y: 5 }, 10)

    }


    fn draw_shape(&self, body: &Body){
        let points = &body.transformed_points;
        // log(&format!("Body: {:?}", body.transformed_points));
        for (i, point_a) in points.iter().enumerate(){
            let point_b = &points[(i + 1) % points.len()];
            self.ctx.begin_path();
            self.ctx.move_to(self.meters_to_pixels_distance_x(point_a.x), self.meters_to_pixels_distance_y(point_a.y));
            self.ctx.line_to(self.meters_to_pixels_distance_x(point_b.x), self.meters_to_pixels_distance_y(point_b.y));
            self.ctx.stroke();
        }
    }

    fn draw_grid(&self){
        self.ctx.set_stroke_style(&JsValue::from_str("lightgray"));
        for (quadrant, cells)  in self.grid.map.iter(){
            self.draw_quadrant_cells(cells);
        }
        self.ctx.set_stroke_style(&JsValue::from_str("Black"));
    }

    fn draw_quadrant_cells(&self, quadrant_cells: &Vec<Vec<Cell>>){
        for column in quadrant_cells.iter(){
            for cell in column.iter(){
                self.draw_cell(cell, self.grid.cell_side_length_meters as i32);
            }
        }
    }

    fn draw_cell(&self, cell: &Cell, cell_side_length: i32){
        let cell_side_length_pixels = self.canvas_pixels_to_meters_ratio * cell_side_length as f64;
        
        self.ctx.begin_path();
        self.ctx.stroke_rect(
            self.meters_to_pixels_distance_x(cell.strokerect_x as f64), 
            self.meters_to_pixels_distance_y(cell.strokerect_y as f64), 
            cell_side_length_pixels, 
            cell_side_length_pixels);
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

        let tick_spacing = 10.0;
        let y_axis_number_of_ticks = canvas_height/(self.canvas_pixels_to_meters_ratio* tick_spacing) ;
        let x_axis_number_ticks = (canvas_width/2.0)/tick_spacing;
        let x_axis_center = canvas_width/2.0;
        
        for i in 0..y_axis_number_of_ticks as i32 {
            let tick_margin = i as f64*(tick_spacing*self.canvas_pixels_to_meters_ratio);
            let tick_x = x_axis_center;
            let tick_y = canvas_height - tick_margin;
            self.ctx.move_to(tick_x-10.0, tick_y);
            self.ctx.line_to(tick_x+10.0, tick_y);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), x_axis_center+15.0, tick_y+3.0); //is there a shorthand for this err handling?
        }

        for i in 0..x_axis_number_ticks as i32 {
            let tick_margin = i as f64*(tick_spacing*self.canvas_pixels_to_meters_ratio);
            let tick_x = x_axis_center + tick_margin;
            let tick_y = canvas_height;
            self.ctx.move_to(tick_x, tick_y);
            self.ctx.line_to(tick_x, tick_y-10.0);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (x_axis_center + tick_margin) -3.0, canvas_height-15 as f64);
        }

        for i in (0..x_axis_number_ticks as i32){
            let tick_margin = i as f64*(tick_spacing*self.canvas_pixels_to_meters_ratio);
            let tick_x = x_axis_center - tick_margin;
            let tick_y = canvas_height;
            self.ctx.move_to(tick_x, tick_y);
            self.ctx.line_to(tick_x, tick_y-10.0);
            self.ctx.stroke();
            self.ctx.fill_text(&i.to_string(), (x_axis_center - tick_margin) -3.0, canvas_height-15 as f64);
        }
    }

    fn meters_to_pixels_distance_y(&self, height: f64) -> f64{
        let distance_in_pixels: f64;
        if height > 0.0 {
            distance_in_pixels = (self.canvas_height as f64/2.0) - (self.canvas_pixels_to_meters_ratio as f64 * height);
        }else if height < 0.0{
            distance_in_pixels = (self.canvas_height as f64/2.0) - (self.canvas_pixels_to_meters_ratio as f64 * height);
        }else{
            distance_in_pixels = self.canvas_height as f64/2.0
        }

        return distance_in_pixels;
    }

    fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
        let distance_in_pixels: f64;
        if distance > 0.0 {
            distance_in_pixels = (self.canvas_width/2) as f64 + self.canvas_pixels_to_meters_ratio as f64 * distance;
        }else if distance < 0.0 {
            distance_in_pixels = (self.canvas_width/2) as f64 + self.canvas_pixels_to_meters_ratio as f64 * distance;
        }else{
            distance_in_pixels = (self.canvas_width/2) as f64;
        }
        return distance_in_pixels; 
    }

}
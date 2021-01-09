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
    // pub canvas_width: u32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    // pub canvas_height: u32,
    pub ctx: CanvasRenderingContext2d,
    pub y_axis_length_meters: u32,
}

impl Renderer {
    // #[wasm_bindgen(constructor)] //Why does compiler panic when self param is added here?
    pub fn new(window: Window, canvas_id: String, grid: Grid, engine: Engine,
         ctx: CanvasRenderingContext2d) -> Renderer{
        Renderer{
            window,
            canvas_id,
            grid,
            engine,
            ctx,
            y_axis_length_meters: 100,
        }
    }

    pub fn run(&mut self){
        self.engine.run();
        self.ctx.clear_rect(0.0, 0.0, self.grid.canvas_width as f64, self.grid.canvas_height as f64);
        self.draw_grid();
        self.draw_axis();
        for body in &self.engine.bodies{
            self.draw_shape(&body);
        }
        // self.draw_cell( &Cell { id: (Quadrant::Quadrant1, 0, 0), position_x: 0, position_y: 0, strokerect_x: 0, strokerect_y: 5 }, 10)
    }

    fn draw_shape(&self, body: &Body){
        self.ctx.set_stroke_style(&JsValue::from_str("Black"));
        let points = &body.transformed_points;
        for (i, point_a) in points.iter().enumerate(){
            let point_b = &points[(i + 1) % points.len()];
            self.ctx.begin_path();
            self.ctx.move_to(self.grid.meters_to_pixels_distance_x(point_a.x), self.grid.meters_to_pixels_distance_y(point_a.y));
            self.ctx.line_to(self.grid.meters_to_pixels_distance_x(point_b.x), self.grid.meters_to_pixels_distance_y(point_b.y));
            self.ctx.stroke();
            self.ctx.close_path();
        }
    }

    fn draw_grid(&self){
        self.ctx.set_stroke_style(&JsValue::from_str("lightgray"));
        for (quadrant, cells)  in self.grid.map.iter(){
            self.draw_quadrant_cells(cells);
        }
    }

    fn draw_quadrant_cells(&self, quadrant_cells: &Vec<Vec<Cell>>){
        for column in quadrant_cells.iter(){
            for cell in column.iter(){
                self.draw_cell(cell, self.grid.cell_side_length_meters as i32);
            }
        }
    }

    fn draw_cell(&self, cell: &Cell, cell_side_length: i32){
        let cell_side_length_pixels = self.grid.canvas_pixels_to_meters_ratio * cell_side_length as f64;
        
        self.ctx.begin_path();
        self.ctx.stroke_rect(
            cell.strokerect_x as f64, 
            cell.strokerect_y as f64, 
            cell_side_length_pixels, 
            cell_side_length_pixels
        );
        self.ctx.close_path();
    }

    fn draw_axis(&mut self){ // needs optimization. slowing down rendering
        let canvas_width = self.grid.canvas_width as f64;
        let canvas_height = self.grid.canvas_height as f64;

        let x_axis_center = canvas_width/2.0;
        let y_axis_center = canvas_height/2.0;
        
        self.ctx.set_stroke_style(&JsValue::from_str("Black"));

        //draw y axis
        self.ctx.begin_path();
        self.ctx.move_to(canvas_width/2.0, 0.0);
        self.ctx.line_to(canvas_width/2.0, canvas_height);
        self.ctx.stroke();
        self.ctx.close_path();

        //draw x axis
        self.ctx.begin_path();
        self.ctx.move_to(0.0, canvas_height/2.0);
        self.ctx.line_to(canvas_width, canvas_height/2.0);
        self.ctx.stroke();
        self.ctx.close_path();

        // self.ctx.set_stroke_style(&JsValue::from_str("Red"));
        self.ctx.set_font("12px Arial");
        for (i, (positive_tick, negative_tick)) in self.grid.x_axis_ticks.iter().enumerate(){
            if i == 0 { continue; }
            self.ctx.begin_path();

            self.ctx.move_to(*positive_tick, y_axis_center+10.0);
            self.ctx.line_to(*positive_tick, y_axis_center-10.0);
            self.ctx.fill_text(&i.to_string(), *positive_tick-4.0, y_axis_center+22.0);
            self.ctx.stroke();
            
            self.ctx.move_to(*negative_tick, y_axis_center+10.0);
            self.ctx.line_to(*negative_tick, y_axis_center-10.0);
            self.ctx.fill_text(&i.to_string(), *negative_tick-4.0, y_axis_center+22.0);
            self.ctx.stroke();
            
            self.ctx.close_path();

        }
        for (i, (positive_tick, negative_tick))  in self.grid.y_axis_ticks.iter().enumerate(){
            if i == 0 { continue; }
            self.ctx.begin_path();

            self.ctx.move_to(x_axis_center+10.0, *positive_tick);
            self.ctx.line_to(x_axis_center-10.0, *positive_tick);
            self.ctx.fill_text(&i.to_string(), x_axis_center+12.0, *positive_tick+4.0);
            self.ctx.stroke();
            
            self.ctx.move_to(x_axis_center+10.0, *negative_tick);
            self.ctx.line_to(x_axis_center-10.0, *negative_tick);
            self.ctx.fill_text(&i.to_string(), x_axis_center+12.0, *negative_tick+4.0);
            self.ctx.stroke();
            
            self.ctx.close_path();
        }
    }

}
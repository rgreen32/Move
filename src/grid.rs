use crate::{body::Body, log};
use std::collections::HashMap;

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: f32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: f32,
    pub canvas_pixels_to_meters_ratio: f64,
    pub map: Vec<Vec<Cell>>,
    pub x_axis_ticks: Vec<(f64 ,f64)>,
    pub y_axis_ticks: Vec<(f64 ,f64)>
}

impl Grid{
    pub fn new(cell_side_length_meters: u32, canvas_width: f32, canvas_height: f32) -> Grid{
        Grid{
            cell_side_length_meters,
            canvas_width,
            canvas_height,
            canvas_pixels_to_meters_ratio: canvas_height as f64/100.0,
            map: Vec::new(),
            //for drawing axis
            x_axis_ticks: Vec::new(),
            y_axis_ticks: Vec::new()
        }
    }

    
    pub fn generate_spatial_mask(body: &Body){
        
    }
    
    pub fn initialize_grid(&mut self){
        log(&format!("ratio: {:?}", self.canvas_pixels_to_meters_ratio));
        let number_of_cells_y_axis = ((self.canvas_height)/(self.canvas_pixels_to_meters_ratio as f32*self.cell_side_length_meters as f32)) as usize;
        let number_of_cells_x_axis = (((self.canvas_width)/(self.canvas_pixels_to_meters_ratio as f32*self.cell_side_length_meters as f32)) + 1.0) as usize; //appending extra cell in case there is space left between last column and edge of screen 
        
        
        let x_counter = number_of_cells_x_axis/2;
        let y_counter = number_of_cells_y_axis/2;
        //iterate rows
        // let mut map: Vec<Vec<Cell>> = Vec::new();
        for y in 0..(number_of_cells_y_axis){
            self.map.push(Vec::new());
        }

        for y in (0..(number_of_cells_y_axis/2)).rev(){
            // self.map.push(Vec::new());
            log(&format!("number_of_cells_x_axis: {:?}", number_of_cells_x_axis));
            log(&format!("number_of_cells_x_axis/2: {:?}", number_of_cells_x_axis/2));

            for x in (1..(number_of_cells_x_axis/2) + 1).rev(){
                let negative_x = -(x as i32);
                let cell: Cell = Cell{
                    id: format!("{}{}", negative_x.to_string(), y.to_string()),
                    quadrant: Quadrant::Quadrant2,
                    position_x: -(x as i32*self.cell_side_length_meters as i32),
                    position_y: y as i32*self.cell_side_length_meters as i32,
                    strokerect_x: self.meters_to_pixels_position_x(-(x as i32*self.cell_side_length_meters as i32) as f64),
                    strokerect_y:  self.meters_to_pixels_position_y((y as i32*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64)
                };

                self.map[y].push(cell);
            }

            for x in 0..number_of_cells_x_axis/2{

                let cell: Cell = Cell{
                    id: format!("{}{}", x.to_string(), y.to_string()),
                    quadrant: Quadrant::Quadrant2,
                    position_x: (x as i32)*self.cell_side_length_meters as i32,
                    position_y: y as i32*self.cell_side_length_meters as i32,
                    strokerect_x: self.meters_to_pixels_position_x((x as i32*self.cell_side_length_meters as i32) as f64),
                    strokerect_y:  self.meters_to_pixels_position_y((y as i32*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64)
                };
                self.map[y].push(cell);
            }
        }

        for y in (1..(number_of_cells_y_axis/2) + 1){
            let negative_y = -(y as i32);
            for x in (1..(number_of_cells_x_axis/2) + 1).rev(){
                let negative_x = -(x as i32);
                let cell: Cell = Cell{
                    id: format!("{}{}", negative_x.to_string(), negative_y.to_string()),
                    quadrant: Quadrant::Quadrant2,
                    position_x: (negative_x*self.cell_side_length_meters as i32),
                    position_y: negative_y*self.cell_side_length_meters as i32,
                    strokerect_x: self.meters_to_pixels_position_x((negative_x*self.cell_side_length_meters as i32) as f64),
                    strokerect_y:  self.meters_to_pixels_position_y((negative_y*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64)
                };

                self.map[y].push(cell);
            }

            for x in 0..number_of_cells_x_axis/2{

                let cell: Cell = Cell{
                    id: format!("{}{}", x.to_string(), negative_y.to_string()),
                    quadrant: Quadrant::Quadrant2,
                    position_x: (x as i32)*self.cell_side_length_meters as i32,
                    position_y: negative_y*self.cell_side_length_meters as i32,
                    strokerect_x: self.meters_to_pixels_position_x((x as i32*self.cell_side_length_meters as i32) as f64),
                    strokerect_y:  self.meters_to_pixels_position_y((negative_y as i32*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64)
                };

                self.map[y].push(cell);
            }
        }

    }

    pub fn meters_to_pixels_position_x(&self, distance: f64) -> f64{
        let distance_in_pixels: f64;
        if distance > 0.0 {
            distance_in_pixels = (self.canvas_width/2.0) as f64 + self.canvas_pixels_to_meters_ratio as f64 * distance;
        }else if distance < 0.0 {
            distance_in_pixels = (self.canvas_width/2.0) as f64 + self.canvas_pixels_to_meters_ratio as f64 * distance;
        }else{
            distance_in_pixels = (self.canvas_width/2.0) as f64;
        }
        return distance_in_pixels; 
    }

    pub fn meters_to_pixels_position_y(&self, height: f64) -> f64{
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
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Quadrant{
    Quadrant1,
    Quadrant2,
    Quadrant3,
    Quadrant4
}

#[derive(Debug)]
pub struct Cell{
    pub id: String, //what data type should this be?
    pub quadrant: Quadrant,
    pub position_x: i32,
    pub position_y: i32,
    pub strokerect_x: f64,
    pub strokerect_y: f64
}
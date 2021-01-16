use crate::{body::Body, log};
use std::collections::HashMap;

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: f32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: f32,
    pub canvas_pixels_to_meters_ratio: f64,
    pub map: HashMap<Quadrant, Vec<Vec<Cell>>>,
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
            map: HashMap::new(),
            //for drawing axis
            x_axis_ticks: Vec::new(),
            y_axis_ticks: Vec::new()
        }
    }

    
    pub fn generate_spatial_mask(body: &Body){
        
    }
    
    pub fn initialize_grid(&mut self){
        let number_of_cells_y_axis = (self.canvas_height/2.0)/(self.canvas_pixels_to_meters_ratio as f32*self.cell_side_length_meters as f32);
        let number_of_cells_x_axis = ((self.canvas_width/2.0)/(self.canvas_pixels_to_meters_ratio as f32*self.cell_side_length_meters as f32)) + 1.0; //appending extra cell in case there is space left between last column and edge of screen 
        
        for x in 0..number_of_cells_x_axis as usize{
            let positive_x_tick = x as i32*self.cell_side_length_meters as i32;
            let negative_x_tick = -(x as i32)*self.cell_side_length_meters as i32;
            let positive_x_tick_pixel_position = self.meters_to_pixels_distance_x(positive_x_tick as f64);
            let negative_x_tick_pixel_position = self.meters_to_pixels_distance_x(negative_x_tick as f64);
            
            self.x_axis_ticks.push((positive_x_tick_pixel_position, negative_x_tick_pixel_position))
        }

        for y in 0..number_of_cells_y_axis as usize{
            let positive_y_tick = y as i32*self.cell_side_length_meters as i32;
            let negative_y_tick = -(y as i32)*self.cell_side_length_meters as i32;
            let positive_y_tick_pixel_position = self.meters_to_pixels_distance_y(positive_y_tick as f64);
            let negative_y_tick_pixel_position = self.meters_to_pixels_distance_y(negative_y_tick as f64);
            
            self.y_axis_ticks.push((positive_y_tick_pixel_position, negative_y_tick_pixel_position))

        }

        //Generate cells for quadrant1
        let mut quadrant1: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant1.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*self.cell_side_length_meters as i32;
                let position_y = y as i32*self.cell_side_length_meters as i32;
                let strokerect_x = self.meters_to_pixels_distance_x((x as i32*self.cell_side_length_meters as i32) as f64);
                let strokerect_y = self.meters_to_pixels_distance_y((y as i32*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64);
                
                quadrant1[x].push(Cell{id: (Quadrant::Quadrant1, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }

        //Generate cells for quadrant2
        let mut quadrant2: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant2.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = -(x as i32)*self.cell_side_length_meters as i32;
                let position_y = y as i32*self.cell_side_length_meters as i32;
                let strokerect_x = self.meters_to_pixels_distance_x((-(x as i32)*self.cell_side_length_meters as i32 - self.cell_side_length_meters as i32) as f64);
                let strokerect_y = self.meters_to_pixels_distance_y((y as i32*self.cell_side_length_meters as i32 + self.cell_side_length_meters as i32) as f64);

                quadrant2[x].push(Cell{id: (Quadrant::Quadrant2, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }

        //Generate cells for quadrant3
        let mut quadrant3: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant3.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = -(x as i32)*self.cell_side_length_meters as i32;
                let position_y = -(y as i32)*self.cell_side_length_meters as i32;
                let strokerect_x = self.meters_to_pixels_distance_x((-(x as i32)*self.cell_side_length_meters as i32 - self.cell_side_length_meters as i32) as f64);
                let strokerect_y = self.meters_to_pixels_distance_y((-(y as i32)*self.cell_side_length_meters as i32) as f64);

                quadrant3[x].push(Cell{id: (Quadrant::Quadrant3, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }

        //Generate cells for quadrant4
        let mut quadrant4: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant4.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*self.cell_side_length_meters as i32;
                let position_y = -(y as i32)*self.cell_side_length_meters as i32;
                let strokerect_x = self.meters_to_pixels_distance_x((x as i32*self.cell_side_length_meters as i32) as f64);
                let strokerect_y = self.meters_to_pixels_distance_y((-(y as i32)*self.cell_side_length_meters as i32) as f64);

                quadrant4[x].push(Cell{id: (Quadrant::Quadrant4, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }
            
        self.map.insert(Quadrant::Quadrant1, quadrant1);
        self.map.insert(Quadrant::Quadrant2, quadrant2);
        self.map.insert(Quadrant::Quadrant3, quadrant3);
        self.map.insert(Quadrant::Quadrant4, quadrant4);
    }

    pub fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
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

    pub fn meters_to_pixels_distance_y(&self, height: f64) -> f64{
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
    pub id: (Quadrant, i32, i32), //what data type should this be?
    pub position_x: i32,
    pub position_y: i32,
    pub strokerect_x: f64,
    pub strokerect_y: f64
}
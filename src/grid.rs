use crate::{body::Body, log};
use std::collections::HashMap;

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: f32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: f32,
    canvas_pixels_to_meters_ratio: f32,
    // meter_to_pixel_ratio: u32,
    // pub y_axis_distance: u32,
    // pub x_axis_distance: u32,
    pub map: HashMap<Quadrant, Vec<Vec<Cell>>>
}

impl Grid{
    pub fn new(cell_side_length_meters: u32, canvas_width: f32, canvas_height: f32) -> Grid{
        Grid{
            cell_side_length_meters,
            canvas_width,
            canvas_height,
            canvas_pixels_to_meters_ratio: canvas_height/100 as f32,

            // will have to assign empty 2d vector and generate map later if i want to move meter_to_pixel functions to Grid
            map: Grid::generate_map(canvas_width, canvas_height, cell_side_length_meters)
        }
    }
    fn generate_map(canvas_width: f32, canvas_height: f32, cell_side_length_meters: u32) -> HashMap<Quadrant, Vec<Vec<Cell>>>  {
        let canvas_pixels_to_meters_ratio = canvas_height/100 as f32;
        let number_of_cells_y_axis = (canvas_height/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32);
        let number_of_cells_x_axis = ((canvas_width/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32)) + 1.0; //appending extra cell in case there is space left between last column and edge of screen 
        let cell_margin = (cell_side_length_meters/2) as i32;
        // log(&format!("Number of cells x_axis: {:?}", number_of_cells_x_axis));
        let mut quadrant1: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            // log("quad 1 push");
            quadrant1.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*cell_side_length_meters as i32;
                let position_y = y as i32*cell_side_length_meters as i32;

                //pre-calculate coords for the top-left corner of cell to make drawing more efficient. 
                let strokerect_x = x as i32*cell_side_length_meters as i32;
                let strokerect_y = y as i32*cell_side_length_meters as i32 + cell_side_length_meters as i32;
                
                quadrant1[x].push(Cell{id: (Quadrant::Quadrant1, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }


        let mut quadrant2: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            // log("quad 2 push");
            quadrant2.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = -(x as i32)*cell_side_length_meters as i32;
                let position_y = y as i32*cell_side_length_meters as i32;

                let strokerect_x = -(x as i32)*cell_side_length_meters as i32 - cell_side_length_meters as i32;
                let strokerect_y = y as i32*cell_side_length_meters as i32 + cell_side_length_meters as i32;

                quadrant2[x].push(Cell{id: (Quadrant::Quadrant2, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }


        let mut quadrant3: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            // log("quad 2 push");
            quadrant3.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = -(x as i32)*cell_side_length_meters as i32;
                let position_y = -(y as i32)*cell_side_length_meters as i32;

                let strokerect_x = -(x as i32)*cell_side_length_meters as i32 - cell_side_length_meters as i32;
                let strokerect_y = -(y as i32)*cell_side_length_meters as i32;

                quadrant3[x].push(Cell{id: (Quadrant::Quadrant3, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }


        let mut quadrant4: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            // log("quad 2 push");
            quadrant4.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*cell_side_length_meters as i32;
                let position_y = -(y as i32)*cell_side_length_meters as i32;

                let strokerect_x = x as i32*cell_side_length_meters as i32;
                let strokerect_y = -(y as i32)*cell_side_length_meters as i32;

                quadrant4[x].push(Cell{id: (Quadrant::Quadrant4, position_x, position_y), position_x, position_y, strokerect_x, strokerect_y});
            }
        }
            
        let mut map: HashMap<Quadrant, Vec<Vec<Cell>>> = HashMap::new(); // add quadrants to map
        map.insert(Quadrant::Quadrant1, quadrant1);
        map.insert(Quadrant::Quadrant2, quadrant2);
        map.insert(Quadrant::Quadrant3, quadrant3);
        map.insert(Quadrant::Quadrant4, quadrant4);
        return map;
    }

    pub fn generate_spatial_mask(body: &Body){
        
    }

    fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
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
    pub strokerect_x: i32,
    pub strokerect_y: i32
}
use crate::{body::Body, log};
use std::collections::HashMap;

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: f32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: f32,
    // meter_to_pixel_ratio: u32,
    // pub y_axis_distance: u32,
    // pub x_axis_distance: u32,
    pub map: Vec<Vec<Cell>>
}

impl Grid{
    pub fn new(cell_side_length_meters: u32, canvas_width: f32, canvas_height: f32) -> Grid{
        Grid{
            cell_side_length_meters,
            canvas_width,
            canvas_height,
            // meter_to_pixel_ratio: (canvas_width*canvas_height) /(cell_side_length_meters*cell_side_length_meters),
            // y_axis_distance,
            // x_axis_distance,
            map: Grid::generate_map(canvas_width, canvas_height, cell_side_length_meters)
        }
    }
    fn generate_map(canvas_width: f32, canvas_height: f32, cell_side_length_meters: u32) -> Vec<Vec<Cell>> {
        let canvas_pixels_to_meters_ratio = canvas_height/100 as f32;
        let number_of_cells_y_axis = (canvas_height/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32);
        let number_of_cells_x_axis = ((canvas_width/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32)) + 1.0; //appending extra cell because there is space left between last column and edge of screen 
        let cell_margin = (cell_side_length_meters/2) as i32;
        
        let mut quadrant1: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant1.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*cell_side_length_meters as i32;
                let position_y = y as i32*cell_side_length_meters as i32;
                let center_x = x as i32*cell_side_length_meters as i32 + cell_margin;
                let center_y = y as i32*cell_side_length_meters as i32 + cell_margin;
                quadrant1[x].push(Cell{id: (Quadrant::Quadrant1, position_x, position_y), position_x, position_y, center_x, center_y});
            }
        }

        let mut quadrant2: Vec<Vec<Cell>> = Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        for x in 0..number_of_cells_x_axis as usize{
            quadrant2.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = -(x as i32)*cell_side_length_meters as i32;
                let position_y = y as i32*cell_side_length_meters as i32;
                let center_x = -(x as i32)*cell_side_length_meters as i32 + cell_margin;
                let center_y = y as i32*cell_side_length_meters as i32 + cell_margin;
                quadrant2[x].push(Cell{id: (Quadrant::Quadrant2, position_x, position_y), position_x, position_y, center_x, center_y});;
            }
        }
        // log(&format!("quadrant1: {:?}", quadrant1));
        let mut map: HashMap<Quadrant, Vec<Vec<Cell>>> = HashMap::new(); // add quadrants to map
        return quadrant1;
    }

    pub fn generate_spatial_mask(body: &Body){
        
    }
}
#[derive(Debug)]
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
    pub center_x: i32,
    pub center_y: i32
}
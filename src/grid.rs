use crate::body::Body;

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: u32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: u32,
    pub y_axis_distance: u32,
    pub x_axis_distance: u32,
    map: Vec<Vec<Cell>>
}

impl Grid{
    // pub fn new(&self) -> Grid{
    //     Grid{
    //         cell_side_length_meters,
    //         canvas_width,
    //         canvas_height,
    //         y_axis_distance,
    //         x_axis_distance,
    //         map: self.generate_map()
    //     }
    // }
    // fn generate_map() -> Vec<Vec<Cell>>{

    // }

    pub fn generate_spatial_mask(body: &Body){
        
    }
}

struct Cell{
    id: u16,
    center_x: f32,
    center_y: f32
}
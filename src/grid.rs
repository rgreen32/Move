use crate::{body::Body, log};

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
        // let number_of_boxes = (canvas_width*canvas_height) /((cell_side_length_meters*ratio)*(cell_side_length_meters*ratio));
        // log(&format!("canvas width: {:?}", canvas_width));
        // log(&format!("canvas height: {:?}", canvas_height));
        // log(&format!("cell_side_length_meters: {:?}", cell_side_length_meters));
        log(&format!("{:?}", canvas_pixels_to_meters_ratio));
        let number_of_cells = canvas_height/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32);
        let number_of_cells_y_axis = (canvas_height/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32);
        let number_of_cells_x_axis = (canvas_width/2.0)/(canvas_pixels_to_meters_ratio*cell_side_length_meters as f32);
        let mut quadrant1: Vec<Vec<Cell>>= Vec::with_capacity((number_of_cells_x_axis*number_of_cells_y_axis) as usize);
        let cell_margin = (cell_side_length_meters/2) as i32;
        for x in 0..number_of_cells_x_axis as usize{
            quadrant1.push(Vec::new());
            for y in 0..number_of_cells_y_axis as usize{
                let position_x = x as i32*cell_side_length_meters as i32;
                let position_y = y as i32*cell_side_length_meters as i32;
                let center_x = x as i32*cell_side_length_meters as i32 + cell_margin;
                let center_y = y as i32*cell_side_length_meters as i32 + cell_margin;
                quadrant1[x].push(Cell{position_x, position_y, center_x, center_y});
            }
        }
        log(&format!("quadrant1: {:?}", quadrant1));
        // let mut x = vec![vec![0.0f64; N]; M];
        let mut right_pillar: Vec<Cell> = Vec::new(); //change to vec
        // log(&format!("number of cells: {:?}", number_of_cells));

        // for i in 0..number_of_cells as u32{
        //     let position_x = 0;
        //     let position_y = i as u32 *cell_side_length_meters;
        //     let center_x = position_x + cell_margin;
        //     let center_y = position_y + cell_margin;
        //     right_pillar.push(Cell{position_x, position_y, center_x, center_y});
        // }
        // log(&format!("array: {:?}", right_pillar));
        // let right_pillar = [Cell; 10];
        // log(&format!("number of boxes: {:?}", number_of_boxes));
        return quadrant1;
    }

    pub fn generate_spatial_mask(body: &Body){
        
    }
}

#[derive(Debug)]
pub struct Cell{
    // id: u16, //what data type should this be?
    pub position_x: i32,
    pub position_y: i32,
    pub center_x: i32,
    pub center_y: i32
}
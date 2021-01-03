use crate::{body::Body, log};

pub struct Grid{
    pub cell_side_length_meters: u32,
    pub canvas_width: f32, //added width and height fields because HTMLCanvas cant be saved to a field atm.
    pub canvas_height: f32,
    // meter_to_pixel_ratio: u32,
    // pub y_axis_distance: u32,
    // pub x_axis_distance: u32,
    pub map: Vec<Cell>
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
    fn generate_map(canvas_width: f32, canvas_height: f32, cell_side_length_meters: u32) -> Vec<Cell> {
        let pixel_to_meter_ratio = canvas_height/100 as f32;
        // let number_of_boxes = (canvas_width*canvas_height) /((cell_side_length_meters*ratio)*(cell_side_length_meters*ratio));
        // log(&format!("canvas width: {:?}", canvas_width));
        // log(&format!("canvas height: {:?}", canvas_height));
        // log(&format!("cell_side_length_meters: {:?}", cell_side_length_meters));
        log(&format!("{:?}", pixel_to_meter_ratio));
        let number_of_cells = canvas_height/(pixel_to_meter_ratio*cell_side_length_meters as f32);
        let mut right_pillar: Vec<Cell> = Vec::new(); //change to vec
        // log(&format!("number of cells: {:?}", number_of_cells));
        let cell_margin = cell_side_length_meters/2;
        for i in 0..number_of_cells as u32{
            let position_x = 0;
            let position_y = i as u32 *cell_side_length_meters;
            let center_x = position_x + cell_margin;
            let center_y = position_y + cell_margin;
            right_pillar.push(Cell{position_x, position_y, center_x, center_y});
        }
        // log(&format!("array: {:?}", right_pillar));
        // let right_pillar = [Cell; 10];
        // log(&format!("number of boxes: {:?}", number_of_boxes));
        return right_pillar;
    }

    pub fn generate_spatial_mask(body: &Body){
        
    }

    // fn meters_to_pixels_distance_y(&self, height: f64) -> f64{
    //     let distance_in_pixels = self.canvas_height as f64 - (self.height_ratio as f64 * height);
    //     return distance_in_pixels;
    // }

    // fn meters_to_pixels_distance_x(&self, distance: f64) -> f64{
    //     let distance_in_pixels: f64;
    //     if distance > 0.0 {
    //         distance_in_pixels = (self.canvas_width/2) as f64 + self.height_ratio as f64 * distance;
    //     }else if distance < 0.0 {
    //         let distance_from_origin_pixels = -(self.height_ratio as f64) * distance;
    //         distance_in_pixels = (self.canvas_width/2) as f64 - distance_from_origin_pixels;
    //     }else{
    //         distance_in_pixels = (self.canvas_width/2) as f64;
    //     }
    //     return distance_in_pixels; 
    // }
}

#[derive(Debug)]
pub struct Cell{
    // id: u16, //what data type should this be?
    pub position_x: u32,
    pub position_y: u32,
    pub center_x: u32,
    pub center_y: u32
}
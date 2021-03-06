import * as Move from "./pkg"
export default Move
export class Body{ // put this class here only as a type definition because wasm_bindgen cannot export a struct that has fields using Vectors.
    position_x: number 
    position_y: number
    old_position_x: number
    old_position_y: number
    mass: number = 10
    height: number
    width: number
    // velocity_magnitude: number
    // velocity_angle: number = 0
    is_static: boolean
    points: any = [] //Array<Point>
    orientation_angle: number 
    sides: number = 4
    transformed_points: any = [] //Array<Point>
    transformed_edges: any = []//Array<Edge>
    constructor(position_x: number, position_y: number,height: number, width: number, velocity: number, orientation_angle: number, isStatic: boolean){
        this.position_x = position_x
        this.position_y = position_y
        this.old_position_x = position_x - velocity
        this.old_position_y = position_y - velocity
        this.height = height
        this.width = width
        // this.velocity_magnitude = velocity
        // this.velocity_angle = velocity_angle
        this.orientation_angle = orientation_angle
        this.is_static = isStatic
    }
}

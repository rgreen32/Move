import * as Move from "./pkg"
export default Move
export class Body{ // put this class here only as a type definition because wasm_bindgen cannot export a struct that has fields using Vectors.
    distance_x: number 
    distance_y: number
    mass: number = 10
    height: number
    width: number
    velocity_magnitude: number
    velocity_angle: number = 0
    is_static: boolean
    points: any = [] //Array<Point>
    orientation_angle: number 
    sides: number = 4
    transformed_points: any = [] //Array<Point>
    transformed_edges: any = []//Array<Edge>
    constructor(distanceX: number, distanceY: number,height: number, width: number, velocity_magnitude: number, velocity_angle: number, orientation_angle: number, isStatic: boolean){
        this.distance_x = distanceX
        this.distance_y = distanceY
        this.height = height
        this.width = width
        this.velocity_magnitude = velocity_magnitude
        this.velocity_angle = velocity_angle
        this.orientation_angle = orientation_angle
        this.is_static = isStatic
    }
}

import * as Move from "./pkg"
export default Move
export class Body{ // put this class here only as a type definition because wasm_bindgen cannot export a struct that has fields using Vectors.
    distance_x: number 
    distance_y: number
    mass: number
    height: number
    width: number
    initial_velocity: number
    is_static: boolean
    points: any = [] //Array<Point>
    angle: number = 0
    sides: number = 4
    transformed_points: any = [] //Array<Point>
    transformed_edges: any = []//Array<Edge>
    constructor(distanceX: number, distanceY: number, 
        mass: number, height: number, width: number, initialVelocity: number, angle: number, isStatic: boolean){
        this.distance_x = distanceX
        this.distance_y = distanceY
        this.mass = mass
        this.height = height
        this.width = width
        this.initial_velocity = initialVelocity
        this.angle = angle
        this.is_static = isStatic
    }
}

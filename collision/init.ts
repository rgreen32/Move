import * as Move from "./pkg"
export default Move
export class Body{ // put this class here only as a type definition because wasm_bindgen cannot export a struct that has fields using Vectors.
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    points: any = [] //Array<Point>
    angle: number = 0
    sides: number = 4
    transformedPoints: any = [] //Array<Point>
    transformedEdges: any = []//Array<Edge>
    constructor(distanceX: number, distanceY: number, 
        mass: number, height: number, width: number, initialVelocity: number, angle: number, isStatic: boolean){
        this.distanceX = distanceX
        this.distanceY = distanceY
        this.mass = mass
        this.height = height
        this.width = width
        this.initialVelocity = initialVelocity
        this.angle = angle
        this.isStatic = isStatic
    }
}

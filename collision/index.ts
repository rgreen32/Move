// import Body from "./core/body"
import wasm from "./init"
class Body{
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    points: any //Array<Point>
    angle: number = 0
    sides: number = 4
    transformedPoints: any //Array<Point>
    transformedEdges: any //Array<Edge>
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
        this.points = []
        this.transformedPoints = []
        this.transformedEdges = []
    }
}


var block = new Body(10, 45, 10, 8, 8, 0, 0, true)
var block2 = new Body(10, 80, 10, 8, 8, 0, 45, false)
// var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block, block2]

let runner = new wasm.SimulationRunner("canvas", bodies)
runner.start()



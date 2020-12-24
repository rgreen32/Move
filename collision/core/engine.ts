import CollisionDetector from "./collision"
import Body from "./body"
import wasm from "../init"

// let engine = new wasm.Engine()



export default class Engine{
    bodies: Array<Body>
    start: number
    collisionDetector: CollisionDetector
    constructor(bodies: Array<any>){
        this.bodies = bodies
        this.start = Date.now()
        this.collisionDetector = new CollisionDetector()
    }

    calculateDisplacement = (body: Body) => {
        var timedelta = Date.now() - this.start;
        var displacement = (body.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
        body.distanceY += displacement
        body.update()
    }

    run = () => {


        this.collisionDetector.run(this.bodies)
        for (let i=0; i < this.bodies.length; i++){
            let body = this.bodies[i]
            if(body.isStatic == true){
                continue
            }

            let displacement = this.calculateDisplacement(body)
            // body.distanceY += displacement
            // body.update()
        }
    }
}
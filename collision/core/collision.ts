import Body from "./body"
import {Point, Edge, Bounds} from "./utils"
import wasm from"../init"


export default class CollisionDetector{

    run = (bodies: Array<Body>) =>{
        let body1 = bodies[0]
        let body2 = bodies[1]

        // let collision = wasm.detect_collision_SAT(body1, body2)

        if(collision){
            body2.isStatic = true
        }
        
    }
}
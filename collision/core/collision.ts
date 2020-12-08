import Body from "./body"
import {Point, Edge, Bounds} from "./utils"
import wasm from".././init"


export default class CollisionDetector{

    createAxisFromEdge = (edge: Edge) => {
        let axisProj = new Point(-(edge.b.y - edge.a.y), (edge.b.x - edge.a.x))
        return axisProj
    }

    boundsOverlap = (body1Bounds: Bounds, body2Bounds: Bounds) => {
        if((body1Bounds.min < body2Bounds.max && body1Bounds.min > body2Bounds.min)
         || (body1Bounds.max > body2Bounds.min && body1Bounds.max < body2Bounds.max)
         || (body2Bounds.min < body1Bounds.max && body2Bounds.min > body1Bounds.min)
         || (body2Bounds.max > body1Bounds.min && body2Bounds.max < body1Bounds.max)){
            return true
        }else{
            return false
        }
    }

    detectCollision_SAT = (body1: Body, body2: Body) => {
        let collision = true
        for(let i = 0; i < body1.transformedEdges.length; i++){
            let axixProj = this.createAxisFromEdge(body1.transformedEdges[i])
            let body1_min = Infinity
            let body1_max = -Infinity
            let body2_min = Infinity
            let body2_max = -Infinity
            for(let x = 0; x < body1.transformedPoints.length; x++){
                let scalar = axixProj.dot(body1.transformedPoints[x])
                if(scalar < body1_min){
                    body1_min = scalar
                }
                if(scalar > body1_max){
                    body1_max = scalar
                }
            }
            for(let x = 0; x < body2.transformedPoints.length; x++){
                let scalar = axixProj.dot(body2.transformedPoints[x])
                if(scalar < body2_min){
                    body2_min = scalar
                }
                if(scalar > body2_max){
                    body2_max = scalar
                }
            }
            let body1Bounds = new Bounds(body1_min, body1_max)
            let body2Bounds = new Bounds(body2_min, body2_max)
            let boundsOverlap = this.boundsOverlap(body1Bounds, body2Bounds)

            if(!boundsOverlap){
                collision = false
                break
            }
        }

        for(let i = 0; i < body2.transformedEdges.length; i++){
            let axixProj = this.createAxisFromEdge(body2.transformedEdges[i])
            let body1_min = Infinity
            let body1_max = -Infinity
            let body2_min = Infinity
            let body2_max = -Infinity
            for(let x = 0; x < body1.transformedPoints.length; x++){
                let scalar = axixProj.dot(body1.transformedPoints[x])
                if(scalar < body1_min){
                    body1_min = scalar
                }
                if(scalar > body1_max){
                    body1_max = scalar
                }
            }
            for(let x = 0; x < body2.transformedPoints.length; x++){
                let scalar = axixProj.dot(body2.transformedPoints[x])
                if(scalar < body2_min){
                    body2_min = scalar
                }
                if(scalar > body2_max){
                    body2_max = scalar
                }
            }
            let body1Bounds = new Bounds(body1_min, body1_max)
            let body2Bounds = new Bounds(body2_min, body2_max)
            let boundsOverlap = this.boundsOverlap(body1Bounds, body2Bounds)
            if(!boundsOverlap){
                collision = false
                break
            }
        }
        return collision
    }

    run = (bodies: Array<Body>) =>{
        let body1 = bodies[0]
        let body2 = bodies[1]

        let collision = wasm.detect_collision_SAT(body1, body2)

        if(collision){
            body2.isStatic = true
        }
        
    }
}
import {evaluate} from 'mathjs'
import {Point, Edge} from "./utils"

export default class Body{
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    points: Array<Point>
    angle: number = 0
    sides: number = 4
    transformedPoints: Array<Point>
    transformedEdges: Array<Edge>
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
        this.calculateShapeVectors()
        this.calculateTransformedShapeVectors()
        this.calculateTransformedEdges()
    }

    update = () =>{
        this.calculateTransformedShapeVectors()
        this.calculateTransformedEdges()
    }

    calculateShapeVectors = () => {
        let theta = 360/this.sides
        let r = this.width/2
        let points = []
        for(let i = 0; i < this.sides; i++){
            let resultx = r * evaluate(`cos(${((theta*i)) + this.angle} deg)`) //Not adding y component?
            let resulty = r * evaluate(`sin(${((theta*i)) + this.angle} deg)`)
            points.push(new Point(resultx, resulty))
        }
        this.points = points
    }

    calculateTransformedShapeVectors = () =>{
        let points = []
        let origin = new Point(this.distanceX, this.distanceY)
        for(let i = 0; i < this.points.length; i++){
            let point = this.points[i]
            let transformedPoint = new Point(origin.x + point.x, origin.y + point.y)
            points.push(transformedPoint)
        }
        this.transformedPoints = points
    }

    calculateTransformedEdges = () =>{
        let edges = []
        for(let i = 0; i < this.transformedPoints.length; i++){
            let pointA = this.transformedPoints[i]
            let pointB = this.transformedPoints[(i+1)% this.transformedPoints.length]
            let edge = new Edge(pointA, pointB)
            edges.push(edge)
        }
        this.transformedEdges = edges
    }

}
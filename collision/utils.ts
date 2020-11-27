import {evaluate} from 'mathjs'

class Point{
    x: number
    y: number
    constructor(x: number, y: number){
        this.x = x
        this.y = y
    }
}

class Edge{
    a: Point
    b: Point
    constructor(a: Point, b: Point){
        this.a = a
        this.b = b
    }
}

export class Body{
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    points: Array<Point>
    transformedPoints: Array<Point>
    constructor(distanceX: number, distanceY: number, 
        mass: number, height: number, width: number, initialVelocity: number, isStatic: boolean){
        this.distanceX = distanceX
        this.distanceY = distanceY
        this.mass = mass
        this.height = height
        this.width = width
        this.initialVelocity = initialVelocity
        this.isStatic = isStatic
        this.calculateShapeVectors(4)
        this.calculateTransformedShapeVectors()
    }

    calculateShapeVectors = (sides: number) => {
        let theta = 360/sides
        let r = this.width/2
        let points = []
        for(let i = 0; i < sides; i++){
            let resultx = r * Math.round(evaluate(`cos(${theta*i} deg)`))
            let resulty = r * Math.round(evaluate(`sin(${theta*i} deg)`))
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
}

export class CollisionDetector{

    createAxisFromEdge = (edge: Edge) => {
        let axisProj = new Point(-(edge.b.y - edge.a.y), (edge.b.x - edge.a.x))
        return axisProj

    }

    detectOverlap_SAT = (body1: Body, body2: Body) => {

        for(let i = 0; i < body1.points.length; i++){
            let pointA = body1.points[i]
            let pointB = body1.points[(i+1)% body1.points.length]
            console.log("Point A", pointA)
            console.log("Point B", pointB)
            let edge = new Edge(pointA, pointB)
            let axixProj = this.createAxisFromEdge(edge)
            console.log(axixProj)
        }
        console.log("===================")
        
        for(let i = 0; i < body1.transformedPoints.length; i++){
            let pointA = body1.transformedPoints[i]
            let pointB = body1.transformedPoints[(i+1)% body1.transformedPoints.length]
            console.log("Point A", pointA)
            console.log("Point B", pointB)
            let edge = new Edge(pointA, pointB)
            let axixProj = this.createAxisFromEdge(edge)
            console.log(axixProj)
        }
    }

    run = (bodies: Array<Body>) =>{
        let edges = []
        let body1 = bodies[0]
        let body2 = bodies[1]
        this.detectOverlap_SAT(body1, body2)
        // for(let a = 0; a < body.points.length; a++){
        //     let point1 = body.points[a]
        //     let point2 = body.points[(a+1) % body.points.length]
        //     let edge = {x:(point2.x - point1.x), y:(point2.y, point1.y)}
        // }

        
    }
}

export class Engine{
    bodies: Array<Body>
    start: number
    collisionDetector: CollisionDetector
    constructor(bodies: Array<any>){
        this.bodies = bodies
        this.start = Date.now()
        this.collisionDetector = new CollisionDetector()
        this.collisionDetector.run(this.bodies)

    }

    calculateDisplacement = (body: Body) => {
        var timedelta = Date.now() - this.start;
        var displacement = (body.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
        body.distanceY += displacement
        body.calculateTransformedShapeVectors()
    }

    run = () => {

        for (let i=0; i < this.bodies.length; i++){
            let body = this.bodies[i]
            if(body.isStatic == true){
                continue
            }
            this.calculateDisplacement(body)
        }
    }
}



export class Renderer{
    canvas: HTMLCanvasElement;
    engine: Engine
    ctx: CanvasRenderingContext2D
    windowRatio: number
    Y_AxisDistance: number
    X_AxisDistance: number
    heightRatio: number
    widthRatio: number

    constructor(canvas: HTMLCanvasElement, engine: Engine){
        this.canvas = canvas
        this.engine = engine
        this.ctx = canvas.getContext('2d');
        this.windowRatio = canvas.width/canvas.height
        this.Y_AxisDistance = 100
        this.X_AxisDistance = 100 * this.windowRatio
        this.heightRatio = canvas.height/this.Y_AxisDistance
        this.widthRatio = canvas.width/2/this.X_AxisDistance
    }
    
    metersToPixelsDistanceY = (height: number) => {
        let distanceInPixels = this.canvas.height - (this.heightRatio * height)
        return distanceInPixels
    }

    metersToPixelsDistanceX = (distance: number) =>{
        let distanceInPixels = null
        if(distance > 0){
            distanceInPixels = (this.canvas.width/2) + this.heightRatio * distance
        }else if(distance < 0){
            let distanceFromOriginPixels = -this.heightRatio * distance
            distanceInPixels = (this.canvas.width/2) - distanceFromOriginPixels
        } else{
            distanceInPixels = (this.canvas.width/2) 
        }
        return distanceInPixels
    }
    

    run = () => {
        this.engine.run()
        let bodies = this.engine.bodies
        
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        this.drawAxis()
        for (let i=0; i < bodies.length; i++){
            let body = bodies[i]
            this.drawShape(body)
        }
    
        window.requestAnimationFrame(this.run)
    }

    drawShape = (body: Body) => {
        let points = body.transformedPoints
        for(let i = 0; i < points.length; i++){
            let pointA = points[i]
            let pointB = points[(i + 1) % points.length]
            this.ctx.beginPath();
            this.ctx.moveTo(this.metersToPixelsDistanceX(pointA.x), this.metersToPixelsDistanceY(pointA.y))
            this.ctx.lineTo(this.metersToPixelsDistanceX(pointB.x), this.metersToPixelsDistanceY(pointB.y))
            this.ctx.stroke()
        }
    }

    drawAxis = () =>{
        this.ctx.beginPath()
        this.ctx.moveTo(this.canvas.width/2, 0)
        this.ctx.lineTo(this.canvas.width/2, this.canvas.height)
        this.ctx.stroke()

        this.ctx.moveTo(0, this.canvas.height)
        this.ctx.lineTo(this.canvas.width, this.canvas.height)
        this.ctx.stroke()

        let ratio = (this.canvas.height)/this.Y_AxisDistance

        let count = this.canvas.height
        let i = 0
        while(count > 0){
            this.ctx.moveTo((this.canvas.width/2)-10, count)
            this.ctx.lineTo((this.canvas.width/2)+10, count)
            this.ctx.stroke()
            this.ctx.fillText(i.toString(), (this.canvas.width/2)+15, count+3); 
            count-= ratio*10
            i++
        }
        
        count = 0
        i = 0
        while(count < this.canvas.width/2){
            this.ctx.moveTo(((this.canvas.width/2)+count), this.canvas.height)
            this.ctx.lineTo(((this.canvas.width/2)+count), this.canvas.height-10)
            this.ctx.stroke()
            this.ctx.fillText(i.toString(), ((this.canvas.width/2)+count)-3, this.canvas.height-15); 
            count+= ratio*10
            i++
        }
        count = 0
        i = 1
        while(count > -this.canvas.width/2){
            this.ctx.moveTo(((this.canvas.width/2)+count), this.canvas.height)
            this.ctx.lineTo(((this.canvas.width/2)+count), this.canvas.height-10)
            this.ctx.stroke()
            count-= ratio*10
            this.ctx.fillText(i.toString(), ((this.canvas.width/2)+count)-3, this.canvas.height-15); 
            i++
        }
    }

    // highlightBounds = (body: Body) =>{
    //     this.ctx.fillStyle = "#FF0000";
        
    //     let params = fillrectTranslator(this.metersToPixelsDistanceX(body.distanceX), this.metersToPixelsDistanceY(body.distanceY), 4, 4)
    //     this.ctx.fillRect(params.x, params.y, params.w, params.h)

    //     let boundx = this.metersToPixelsDistanceX(body.bounds.x.min)
    //     let boundy = this.metersToPixelsDistanceY(body.distanceY)
    //     params = fillrectTranslator(boundx, boundy, 4, 4)
    //     this.ctx.fillRect(params.x, params.y, params.w, params.h)

    //     boundx = this.metersToPixelsDistanceX(body.distanceX)
    //     boundy = this.metersToPixelsDistanceY(body.bounds.y.min)
    //     params = fillrectTranslator(boundx, boundy, 4, 4)
    //     this.ctx.fillRect(params.x, params.y, params.w, params.h)

    //     boundx = this.metersToPixelsDistanceX(body.bounds.x.max)
    //     boundy = this.metersToPixelsDistanceY(body.distanceY)
    //     params = fillrectTranslator(boundx, boundy, 4, 4)
    //     this.ctx.fillRect(params.x, params.y, params.w, params.h)

    //     boundx = this.metersToPixelsDistanceX(body.distanceX)
    //     boundy = this.metersToPixelsDistanceY(body.bounds.y.max)
    //     params = fillrectTranslator(boundx, boundy, 4, 4)
    //     this.ctx.fillRect(params.x, params.y, params.w, params.h)

    // }
    

}

function fillrectTranslator(x: number, y: number, w: number, h: number){
    let X = x - (w/2)
    let Y = y - (h/2)
    return {x: X, y: Y, w: w, h: h}
}
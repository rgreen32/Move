import {evaluate, or} from 'mathjs'

interface BoundValue{
    min: number
    max: number
}

interface Bounds{
    x: BoundValue
    y: BoundValue
}

class Vector{
    length: number
    angle: number
    constructor(length: number, angle: number){
        this.length = length
        this.angle = angle
    }
}


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
    angle: number
    constructor(a: Point, b: Point, angle: number){
        this.a = a
        this.b = b
        this.angle = angle
    }
}

export class Body{
    angle: number = 90
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    bounds: Bounds
    edges: Array<Edge>
    points: Array<Point>
    edgePoints: Array<Point>
    constructor(distanceX: number, distanceY: number, 
        mass: number, height: number, width: number, initialVelocity: number, isStatic: boolean){
        this.distanceX = distanceX
        this.distanceY = distanceY
        this.mass = mass
        this.height = height
        this.width = width
        this.initialVelocity = initialVelocity
        this.isStatic = isStatic
        this.calculateBounds()
        this.points = this.calculateShapeVectors(4)
        
        let topRightCorner = new Point(this.bounds.x.max, this.bounds.y.max) 
        let bottomRightCorner = new Point(this.bounds.x.max, this.bounds.y.min)
        let topLeftCorner = new Point(this.bounds.x.min, this.bounds.y.max)
        let bottomLeftCorner = new Point(this.bounds.x.min, this.bounds.y.min)
        this.edgePoints = [topRightCorner, bottomRightCorner, topLeftCorner, bottomLeftCorner]
        this.edges = [new Edge(topRightCorner, bottomRightCorner, 90), new Edge(bottomRightCorner, bottomLeftCorner, 0),
             new Edge(bottomLeftCorner, topLeftCorner, 0), new Edge(topLeftCorner, topRightCorner, 90)]
    }

    calculateShapeVectors = (sides: number) => {
        let theta = 360/sides
        let r = this.width/2
        let points = []
        for(let i = 0; i < sides; i++){
            let resultx = r * Math.round(evaluate(`cos(${theta*i} deg)`))
            let resulty = r * Math.round(evaluate(`sin(${theta*i} deg)`))
            console.log("i", i)
            console.log("theta", `cos(${theta*i} deg)`)
            console.log("cosine", evaluate(`cos(${theta*i} deg)`))
            console.log("x", resultx)
            if (i == 3){
                console.log((resultx *3) == -0)
            }

            points.push(new Point(resultx, resulty))
        }
        // console.log(points)
        return points

    }

    calculateBounds = () =>{
        this.bounds = {
            x: {min: (this.distanceX - (this.width/2)), max: (this.distanceX + (this.width/2))},
            y: {min: (this.distanceY - (this.height/2)), max: (this.distanceY + (this.height/2))}
        }
    }

}

export class CollisionDetector{

    createAxisFromEdge = (edge: Edge) => {
        // console.log("edge", edge)
        let axisProj = new Point(-(edge.a.y - edge.b.y), (edge.a.x - edge.b.x))
        // console.log("projection", axisProj)
        let axis = new Vector(10, edge.angle - 90)
        // let projectedlength = edgeLength*Math.floor(evaluate(`cos(${axis.angle} deg)`))
        return axis

    }

    run = (bodies: Array<Body>) =>{

        // for(let i = 0; i < bodies.length; i++){
        //     let body = bodies[i]
        //     this.createAxisFromEdge(body.edges[0])
        //     }
        let edges = []
        let body = bodies[0]
        for(let a = 0; a < body.points.length; a++){
            let point1 = body.points[a]
            let point2 = body.points[(a+1) % body.points.length]
            let edge = {x:(point2.x - point1.x), y:(point2.y, point1.y)}
            edges.push(edge)
        }
        // console.log(edges)

        
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
    }

    run = () => {

        for (let i=0; i < this.bodies.length; i++){
            let body = this.bodies[i]
            if(body.isStatic == true){
                continue
            }
            this.calculateDisplacement(body)
            body.calculateBounds()
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
        this.drawShape(engine.bodies[0])

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

        this.ctx.moveTo(this.metersToPixelsDistanceX(14), this.metersToPixelsDistanceY(90))
        this.ctx.lineTo(this.metersToPixelsDistanceX(10), this.metersToPixelsDistanceY(94))
        this.ctx.stroke()
        this.ctx.moveTo(this.metersToPixelsDistanceX(10), this.metersToPixelsDistanceY(94))
        this.ctx.lineTo(this.metersToPixelsDistanceX(6), this.metersToPixelsDistanceY(90))
        this.ctx.stroke()
        this.ctx.moveTo(this.metersToPixelsDistanceX(6), this.metersToPixelsDistanceY(90))
        this.ctx.lineTo(this.metersToPixelsDistanceX(10), this.metersToPixelsDistanceY(86)) // x would be 6
        this.ctx.stroke()
        this.ctx.moveTo(this.metersToPixelsDistanceX(10), this.metersToPixelsDistanceY(86)) // x would be 6
        this.ctx.lineTo(this.metersToPixelsDistanceX(14), this.metersToPixelsDistanceY(90))
        this.ctx.stroke()
        
        this.drawAxis()
        for (let i=0; i < bodies.length; i++){
            let body = bodies[i]
            let positionX = this.metersToPixelsDistanceX(body.distanceX)
            let positionY = this.metersToPixelsDistanceY(body.distanceY)
            let widthPixels = (body.width * this.heightRatio)
            let heightPixels = (body.height * this.heightRatio)


            let params = fillrectTranslator(positionX, positionY, widthPixels, heightPixels)
        

            // this.ctx.fillRect(params.x ,params.y, params.w, params.h)
            // body.calculateBounds()
            // this.highlightBounds(body)
            
            this.ctx.fillStyle = "#000000";
        }
    
        window.requestAnimationFrame(this.run)
    }

    drawShape = (body: Body) => {
        let points = body.points
        let origin = new Point(body.distanceX, body.distanceY)
        console.log("Origin", origin)
        for(let i = 0; i < points.length; i++){
            let pointA = points[i]
            let pointB = points[(i + 1) % points.length]
            // console.log("origin x", origin.x)
            // console.log("pointA x", pointA.x)
            // console.log("pointB x", pointB.x)
            // let pointA2 = new Point(origin.x + pointA.x, origin.y + pointA.y)
            // let pointB2 = new Point(origin.x + pointB.x, origin.y + pointB.y)
            // this.ctx.beginPath();
            // console.log("PointA2 X", pointA2.x)
            // // console.log("PointA2 Y", pointA2.y)
            // console.log("PointB2 X", pointB2.x)
            // // console.log("PointB2 Y", pointB2.y)
            // console.log("=========")
            // // this.ctx.stroke()

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

    highlightBounds = (body: Body) =>{
        this.ctx.fillStyle = "#FF0000";
        
        let params = fillrectTranslator(this.metersToPixelsDistanceX(body.distanceX), this.metersToPixelsDistanceY(body.distanceY), 4, 4)
        this.ctx.fillRect(params.x, params.y, params.w, params.h)

        let boundx = this.metersToPixelsDistanceX(body.bounds.x.min)
        let boundy = this.metersToPixelsDistanceY(body.distanceY)
        params = fillrectTranslator(boundx, boundy, 4, 4)
        this.ctx.fillRect(params.x, params.y, params.w, params.h)

        boundx = this.metersToPixelsDistanceX(body.distanceX)
        boundy = this.metersToPixelsDistanceY(body.bounds.y.min)
        params = fillrectTranslator(boundx, boundy, 4, 4)
        this.ctx.fillRect(params.x, params.y, params.w, params.h)

        boundx = this.metersToPixelsDistanceX(body.bounds.x.max)
        boundy = this.metersToPixelsDistanceY(body.distanceY)
        params = fillrectTranslator(boundx, boundy, 4, 4)
        this.ctx.fillRect(params.x, params.y, params.w, params.h)

        boundx = this.metersToPixelsDistanceX(body.distanceX)
        boundy = this.metersToPixelsDistanceY(body.bounds.y.max)
        params = fillrectTranslator(boundx, boundy, 4, 4)
        this.ctx.fillRect(params.x, params.y, params.w, params.h)

    }
    

}

function fillrectTranslator(x: number, y: number, w: number, h: number){
    let X = x - (w/2)
    let Y = y - (h/2)
    return {x: X, y: Y, w: w, h: h}
}
import {evaluate} from 'mathjs'

class Point{
    x: number
    y: number
    constructor(x: number, y: number){
        this.x = x
        this.y = y
    }
    dot = (point: Point) => {
        let scalar = (this.x * point.x) + (this.y * point.y)
        return scalar
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

class Bounds{
    min: number
    max: number
    constructor(min: number, max: number){
        this.min = min
        this.max = max
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

export class CollisionDetector{

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
            // console.log("-")
        }
        return collision
    }

    run = (bodies: Array<Body>) =>{
        let body1 = bodies[0]
        let body2 = bodies[1]
        let collision = this.detectCollision_SAT(body1, body2)
        console.log("collision", collision)
        if(collision){
            body2.isStatic = true
        }
        
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


    }

    calculateDisplacement = (body: Body) => {
        var timedelta = Date.now() - this.start;
        var displacement = (body.initialVelocity * (timedelta/1000)) + (.5*-0.8*Math.pow((timedelta/1000), 2))
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
}
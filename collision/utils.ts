interface BoundValue{
    min: number
    max: number
}

interface Bounds{
    x: BoundValue
    y: BoundValue
}

export class Body{
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    bounds: Bounds
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
    }

    calculateBounds = () =>{
        this.bounds = {
            x: {min: (this.distanceX - (this.width/2)), max: (this.distanceX + (this.width/2))},
            y: {min: (this.distanceY - (this.height/2)), max: (this.distanceY + (this.height/2))}
        }
    }

}

export class CollisionDetector{
    run = (bodies: Array<Body>) =>{
        for(let i = 0; i < bodies.length; i++){
            let body = bodies[i]
            for(let x = 0; x < bodies.length; x++){
                if(i == x){
                    continue
                }
                let body2 = bodies[x]
                if(body.bounds.x.min > body2.bounds.x.min && body.bounds.x.min < body2.bounds.x.max){
                    
                    if(body.bounds.y.min > body2.bounds.y.min && body.bounds.y.min < body2.bounds.y.max){
                        console.log("X Overlap")
                        console.log("Y Overlap")
                        prompt("collsion")
                    }
                }

            }

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
        var displacement = (body.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
        body.distanceY += displacement
    }

    run = () => {
        this.collisionDetector.run(this.bodies)
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
            let positionX = this.metersToPixelsDistanceX(body.distanceX)
            let positionY = this.metersToPixelsDistanceY(body.distanceY)
            let widthPixels = (body.width * this.heightRatio)
            let heightPixels = (body.height * this.heightRatio)


            let params = fillrectTranslator(positionX, positionY, widthPixels, heightPixels)
        

            this.ctx.fillRect(params.x ,params.y, params.w, params.h)
            // body.calculateBounds()
            this.highlightBounds(body)
            
            this.ctx.fillStyle = "#000000";
        }
    
        window.requestAnimationFrame(this.run)
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

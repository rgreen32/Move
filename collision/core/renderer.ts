import Engine from "./engine"
import Body from "./body"

export default class Renderer{
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
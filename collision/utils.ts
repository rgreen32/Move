export class Body{
    distanceX: number 
    distanceY: number
    mass: number
    height: number
    width: number
    initialVelocity: number
    isStatic: boolean
    constructor(distanceX: number, distanceY: number, 
        mass: number, height: number, width: number, initialVelocity: number, isStatic: boolean){
        this.distanceX = distanceX
        this.distanceY = distanceY
        this.mass = mass
        this.height = height
        this.width = width
        this.initialVelocity = initialVelocity
        this.isStatic = isStatic
    }

}

export class Engine{
    bodies: Array<Body>
    start: number
    constructor(bodies: Array<any>){
        this.bodies = bodies
        this.start = Date.now()
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
        }
    }
}

export class Renderer{
    canvas: HTMLCanvasElement;
    engine: any
    ctx: CanvasRenderingContext2D
    windowRatio: number
    Y_AxisDistance: number
    X_AxisDistance: number
    heightRatio: number
    widthRatio: number

    constructor(canvas: HTMLCanvasElement, engine: any){
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
            distanceInPixels = (this.canvas.width/2) + this.widthRatio * distance
        }else if(distance < 0){
            let distanceFromOriginPixels = -this.widthRatio * distance
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
        for (let i=0; i < bodies.length; i++){
            this.ctx.fillRect(this.metersToPixelsDistanceX(bodies[i].distanceX), this.metersToPixelsDistanceY(bodies[i].distanceY), (bodies[i].width * this.heightRatio), (bodies[i].height * this.heightRatio))
        }
    
        window.requestAnimationFrame(this.run)
    }
    

}
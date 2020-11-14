var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement
canvas.height = window.innerHeight
canvas.width = window.innerWidth
const ctx = canvas.getContext('2d');



var windowRatio = canvas.width/canvas.height

var Y_AxisDistance = 100
var X_AxisDistance = 100 * windowRatio

var heightRatio = canvas.height/Y_AxisDistance
var widthRatio = canvas.width/2/X_AxisDistance

var block = {distanceX : 0, distanceY: 80, mass: 10, height: 1, width: 1, initialVelocity: 0, static: true}

var floor = {distanceX : 0, distanceY: 30, mass: 1000, height: 2, width: 75, initialVelocity: 0, static: true} 

var bodies = [block, floor]

class Engine{
    canvas: HTMLCanvasElement;
    bodies: any
    start: number
    constructor(canvas, bodies){
        this.canvas = canvas
        this.bodies = bodies
        this.start = Date.now()
    }

    calculateDisplacement(body){
        var timedelta = Date.now() - this.start;
        var displacement = (body.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
        body.distanceY += displacement
    }

    run(){
        for (let i=0; i < this.bodies.length; i++){
            let body = this.bodies[i]
            if(body.static == true){
                continue
            }
            this.calculateDisplacement(body)
        }
    }
}

class Renderer{
    canvas: HTMLCanvasElement;
    engine: any
    constructor(canvas: HTMLCanvasElement, engine: any){
        this.canvas = canvas
        this.engine = engine
    }
    
    metersToPixelsDistanceY(height: number){
        let distanceInPixels = canvas.height - (heightRatio * height)
        return distanceInPixels
    }

    metersToPixelsDistanceX(distance: number){
        let distanceInPixels = null
        if(distance > 0){
            distanceInPixels = (canvas.width/2) + widthRatio * distance
        }else if(distance < 0){
            let distanceFromOriginPixels = -widthRatio * distance
            distanceInPixels = (canvas.width/2) - distanceFromOriginPixels
        } else{
            distanceInPixels = (canvas.width/2) 
        }
        return distanceInPixels
    }

    run(){
        this.engine.run()
        let bodies = this.engine.bodies
        for (let i=0; i < bodies.length; i++){
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.fillRect(metersToPixelsDistanceX(bodies[i].distanceX), metersToPixelsDistanceY(bodies[i].distanceY), (bodies[i].width * heightRatio), (bodies[i].height * heightRatio))
        }
    
        window.requestAnimationFrame(this.run)
    }
    

}

// function fall(){

//     let previousHeight = block.distanceY.valueOf()
//     ctx.clearRect(0, 0, canvas.width, canvas.height);
//     block.distanceY += calculateDisplacement()
//     ctx.fillRect(metersToPixelsDistanceX(block.distanceX), metersToPixelsDistanceY(block.distanceY), (block.width * heightRatio), (block.height * heightRatio))


//     window.requestAnimationFrame(fall)
// }


window.requestAnimationFrame(fall)

var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement
canvas.height = window.innerHeight
canvas.width = window.innerWidth
const ctx = canvas.getContext('2d');



var windowRatio = canvas.width/canvas.height

var Y_AxisDistance = 100
var X_AxisDistance = 100 * windowRatio

var heightRatio = canvas.height/Y_AxisDistance
var widthRatio = canvas.width/2/X_AxisDistance

var block = {distanceX : -20, distanceY: 80, mass: 10, height: 1, width: 1, initialVelocity: 0}

var floor = {distanceX : 0, distanceY: 0, mass: 1000, initialVelocity: null, static: true} 


ctx.fillRect(metersToPixelsDistanceX(block.distanceX), metersToPixelsDistanceY(block.distanceY), (block.width * heightRatio), (block.height * heightRatio))
var start = Date.now();
function fall(){

    let previousHeight = block.distanceY.valueOf()
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    block.distanceY += calculateDisplacement()
    ctx.fillRect(metersToPixelsDistanceX(block.distanceX), metersToPixelsDistanceY(block.distanceY), (block.width * heightRatio), (block.height * heightRatio))


    window.requestAnimationFrame(fall)
}

function calculateDisplacement(){
    var timedelta = Date.now() - start;
    var displacement = (block.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
    return displacement
}

function metersToPixelsDistanceY(height: number){
    let distanceInPixels = canvas.height - (heightRatio * height)
    return distanceInPixels
}

function metersToPixelsDistanceX(distance: number){
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

window.requestAnimationFrame(fall)

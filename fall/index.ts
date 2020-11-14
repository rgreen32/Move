var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement
canvas.height = window.innerHeight
canvas.width = window.innerWidth
const ctx = canvas.getContext('2d');

var block = {x : 100, height: 80, mass: 10, initialVelocity: 0}

var ratio = canvas.height/100

ctx.fillRect(block.x, metersToPixels(block.height), 10, 10)
var start = Date.now();
function fall(){

    let previousHeight = block.height.valueOf()
    ctx.clearRect(block.x, metersToPixels(previousHeight), 10, 10)
    block.height += calculateDisplacement()
    ctx.fillRect(block.x, metersToPixels(block.height), 10, 10)


    window.requestAnimationFrame(fall)
}

function calculateDisplacement(){
    var timedelta = Date.now() - start;
    var displacement = (block.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
    return displacement
}

function metersToPixels(height: number){
    let heightInPixels = ratio * height

    return canvas.height - heightInPixels
}

window.requestAnimationFrame(fall)

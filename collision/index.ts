import {Body, Engine, Renderer} from "./utils"

var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement
canvas.height = window.innerHeight
canvas.width = window.innerWidth

var block = new Body(0, 80, 10, 1, 1, 0, false)
var floor = new Body(0, 30, 1000, 2, 75,0, true)

var bodies = [block, floor]


var engine = new Engine(bodies)
var renderer = new Renderer(canvas, engine)
window.requestAnimationFrame(renderer.run)

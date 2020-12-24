import Body from "./core/body"
import Engine from "./core/engine"
import Renderer from "./core/renderer"
import wasm from "./init"


var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement


let test = [new Body(10, 45, 10, 8, 8, 0, 0, true)]
let engine2 = new wasm.Engine(test)

let renderer2 = new wasm.Renderer("canvas", engine2)
// var block = new Body(10, 90, 10, 8, 8, 0, 0, true)
// var block2 = new Body(5, 30, 10, 8, 8, 0, 45, true)
var block = new Body(10, 45, 10, 8, 8, 0, 0, true)
var block2 = new Body(10, 80, 10, 8, 8, 0, 45, false)
var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block, block2]


var engine = new Engine(bodies)
var renderer = new Renderer(canvas, engine)

window.requestAnimationFrame(renderer.run)


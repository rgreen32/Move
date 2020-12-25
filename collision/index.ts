import Body from "./core/body"
import wasm from "./init"


var block = new Body(10, 45, 10, 8, 8, 0, 0, true)
var block2 = new Body(10, 80, 10, 8, 8, 0, 45, false)
var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block, block2]

// let engine2 = new wasm.Engine(bodies)

// let renderer2 = new wasm.Renderer("canvas", engine2)

let runner = new wasm.SimulationRunner("canvas", bodies)
// renderer2.run()
// var engine = new Engine(bodies)
// var renderer = new Renderer(canvas, engine)

// window.requestAnimationFrame(renderer.run)


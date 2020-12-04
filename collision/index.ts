import Body from "./core/body"
import Engine from "./core/engine"
import Renderer from "./core/renderer"

// this import loads and compiles wasm module
import("../pkg").then(wasm => {
    wasm.test_function("Testies!!")

    var canvas : HTMLCanvasElement = document.getElementById("canvas") as HTMLCanvasElement
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    
    // var block = new Body(10, 90, 10, 8, 8, 0, 0, true)
    // var block2 = new Body(5, 30, 10, 8, 8, 0, 45, true)
    var block = new Body(10, 45, 10, 8, 8, 0, 0, true)
    var block2 = new Body(10, 80, 10, 8, 8, 0, 45, false)
    var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)
    
    var bodies = [floor, block2]
    
    
    var engine = new Engine(bodies)
    var renderer = new Renderer(canvas, engine)
    
    window.requestAnimationFrame(renderer.run)
    
}).catch(console.error);
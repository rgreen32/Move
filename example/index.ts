import Move, {Body} from "./init"


var block = new Body(20, 35, 8, 8, 25, 0, 0, false)
var block2 = new Body(14, -20, 8, 8, 0, 0, 0, true)

// var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block2, block]

let runner = new Move.SimulationRunner("canvas", bodies)
runner.start()



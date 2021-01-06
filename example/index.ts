import Move, {Body} from "./init"


var block = new Body(0, 0, 10, 8, 8, 0, 0, true)
var block2 = new Body(-70, 0, 10, 8, 8, 0, 45, false)
// var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block, block2]

let runner = new Move.SimulationRunner("canvas", bodies)
runner.start()



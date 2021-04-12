import Move, {Body} from "./init"


var block = new Body(5, 0, 8, 8, 3, 18, 45, true)
// var block2 = new Body(11, -11, 8, 8, 0, 0, 45, true)

// var floor = new Body(0, 30, 1000, 2, 30,0, 0, true)

var bodies = [block]

let runner = new Move.SimulationRunner("canvas", bodies)
runner.start()



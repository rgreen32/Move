
export class Point{
    x: number
    y: number
    constructor(x: number, y: number){
        this.x = x
        this.y = y
    }
    dot = (point: Point) => {
        let scalar = (this.x * point.x) + (this.y * point.y)
        return scalar
    }
}

export class Edge{
    a: Point
    b: Point
    constructor(a: Point, b: Point){
        this.a = a
        this.b = b
    }
}

export class Bounds{
    min: number
    max: number
    constructor(min: number, max: number){
        this.min = min
        this.max = max
    }
}










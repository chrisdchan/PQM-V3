
export type Curve = {
    line: Line[],
}

export type Line = {
    start: Point,
    end: Point
}

export type Point = {
    x: number,
    y: number,
}
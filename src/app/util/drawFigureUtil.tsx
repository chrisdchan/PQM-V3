import * as PIXI from 'pixi.js'
import { GraphDisplay, GraphDisplayStyle, Line, Margin, Point } from '../types/api'
import { drawCanvasLine, drawLine, drawRectangle } from './drawUtil'
import { WindowDimensions } from '../lib/graph'
import { drawGraph } from './drawGraphUtil'
import { CanvasLine, CanvasPoint } from '../types/graphics'


export const drawFigure = (fg: PIXI.Graphics, bg: PIXI.Graphics, dims: WindowDimensions, graphDisplay: GraphDisplay)  => {
    fg.zIndex = 2
    bg.zIndex = 1

    fg.clear()

    drawXAxis(fg, dims, graphDisplay.graphDisplayStyle)
    drawYAxis(fg, dims, graphDisplay.graphDisplayStyle)
    drawGraph(fg, dims, graphDisplay)
}

const drawXAxis = (g: PIXI.Graphics, windowDimensions: WindowDimensions, graphDisplayStyle: GraphDisplayStyle) => {
    const margin: Margin = graphDisplayStyle.margin
    const y =  (1 - margin.bottom) * windowDimensions.height
    const x1 = margin.left * windowDimensions.width
    const x2 = (1 - margin.right) * windowDimensions.width
    const p1 : CanvasPoint = { x: x1, y: y }
    const p2 : CanvasPoint = { x: x2, y: y }
    const line : CanvasLine = { start: p1, end: p2 }
    drawCanvasLine(g, line, graphDisplayStyle.xAxisStyle.lineStyle)
}

const drawYAxis = (g: PIXI.Graphics, dims: WindowDimensions, graphDisplayStyle: GraphDisplayStyle) => {
    const margin: Margin = graphDisplayStyle.margin
    const x = margin.left * dims.width
    const y1 = (1 - margin.bottom) * dims.height
    const y2 = margin.top * dims.height

    const p1 : CanvasPoint = { x: x, y: y1 }
    const p2 : CanvasPoint = { x: x, y: y2 }
    const line : CanvasLine = { start: p1, end: p2 }
    drawCanvasLine(g, line, graphDisplayStyle.xAxisStyle.lineStyle)
}

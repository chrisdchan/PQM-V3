import * as PIXI from 'pixi.js'
import { GraphDisplay, GraphDisplayStyle, Line, LineStyle, Point } from '../types/api';
import { CanvasLine, CanvasPoint } from '../types/graphics';
import { WindowDimensions } from '../lib/graph';

export const drawCanvasLine = (g: PIXI.Graphics, line: CanvasLine, style: LineStyle) => {
    g.lineStyle(style.width, style.color, 1);
    g.moveTo(line.start.x, line.start.y);
    g.lineTo(line.end.x, line.end.y);
}

export const drawLine = (g: PIXI.Graphics, line: Line, style: LineStyle, dims: WindowDimensions, graphDisplay: GraphDisplay) => {
    const canvasLine : CanvasLine = lineToCanvasLine(line, dims, graphDisplay)
    drawCanvasLine(g, canvasLine, style)
}

export const drawRectangle = (g: PIXI.Graphics, p1: CanvasPoint, p2: CanvasPoint, color: String) => {
    g.beginFill(color)
    g.drawRect(p1.x, p1.y, p2.x, p2.y)
}

export const pointToCanvasPoint = (p: Point, dims: WindowDimensions, graphDisplay: GraphDisplay) : CanvasPoint => {
    const canvasWidth = dims.width * (1 - graphDisplay.graphDisplayStyle.margin.left - graphDisplay.graphDisplayStyle.margin.right)
    const canvasHeight = dims.height * (1 - graphDisplay.graphDisplayStyle.margin.top - graphDisplay.graphDisplayStyle.margin.bottom)

    const xAxisDisplayProperties = graphDisplay.graphDisplayProperties.xAxisDisplayProperties
    const yAxisDisplayProperties = graphDisplay.graphDisplayProperties.yAxisDisplayProperties
    const xAxisWidth = xAxisDisplayProperties.end - xAxisDisplayProperties.start
    const yAxisHeight = yAxisDisplayProperties.end - yAxisDisplayProperties.start

    const canvasPoint : CanvasPoint = {
        x: (p.x * canvasWidth) / xAxisWidth + graphDisplay.graphDisplayStyle.margin.left * dims.width,
        y: (1 - graphDisplay.graphDisplayStyle.margin.bottom) * dims.height - (p.y * canvasHeight) / yAxisHeight
    }

    return canvasPoint
}

export const canvasToPoint = (canvasPoint: CanvasPoint, dims: WindowDimensions, graphDisplay: GraphDisplay) : Point => {
    const canvasWidth = dims.width * (1 - graphDisplay.graphDisplayStyle.margin.left - graphDisplay.graphDisplayStyle.margin.right)
    const canvasHeight = dims.height * (1 - graphDisplay.graphDisplayStyle.margin.top - graphDisplay.graphDisplayStyle.margin.bottom)

    const xAxisDisplayProperties = graphDisplay.graphDisplayProperties.xAxisDisplayProperties
    const yAxisDisplayProperties = graphDisplay.graphDisplayProperties.yAxisDisplayProperties
    const xAxisWidth = xAxisDisplayProperties.end - xAxisDisplayProperties.start
    const yAxisHeight = yAxisDisplayProperties.end - yAxisDisplayProperties.start

    const point : Point = {
        x: (canvasPoint.x * xAxisWidth) / canvasWidth,
        y: (canvasPoint.y * yAxisHeight) / canvasHeight
    }

    return point
}

export const lineToCanvasLine = (line: Line, dims: WindowDimensions, graphDisplay: GraphDisplay) : CanvasLine => {
    const canvasLine: CanvasLine = {
        start: pointToCanvasPoint(line.start, dims, graphDisplay),
        end: pointToCanvasPoint(line.end, dims, graphDisplay)
    }
    return canvasLine
}
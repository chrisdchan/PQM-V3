import type { GraphContext } from "../types/graphContext"
import * as PIXI from 'pixi.js'

export const toCanvasPoint = (ctx: GraphContext, x: number, y: number): PIXI.Point => {
    const xAxisProp = ctx.properties.xAxisProperties;
    const yAxisProp = ctx.properties.yAxisProperties;
    const margin = ctx.style.margin;

    const canvasX = (margin.right - margin.left) * x / (xAxisProp.end - xAxisProp.start);
    const canvasY = (margin.top - margin.bottom) * y / (yAxisProp.end - yAxisProp.start);

    return new PIXI.Point(canvasX, canvasY);
}
import * as PIXI from 'pixi.js'
import type { AxisStyle, GraphStyle, LineStyle } from '../types/graphStyle';
import type { GraphContext } from '../types/graphContext';
import { drawLine } from './drawUtils';
import { GraphDisplayProperties } from '../types/graphDisplayProperties';

export const drawXAxis = (g: PIXI.Graphics, props: GraphDisplayProperties) => {

    let start = new PIXI.Point(graphStyle.margin.left, graphStyle.margin.bottom);
    let end = new PIXI.Point(app.view.width - graphStyle.margin.right, graphStyle.margin.bottom);
    drawLine(app, axisStyle.line, start, end);

    for (let i = graphStyle.margin.left; i < graphStyle.margin.right; i += axisProp.tickGap) {
        start = new PIXI.Point(i, )
        drawLine(app, axisStyle.tickLineStyles.line, start, end);

    }
}


export const drawYAxis = (ctx: GraphContext) => {
    const graphStyle = ctx.style
    const axisStyle = graphStyle.yAxis;
    const app = ctx.app

    const start: PIXI.Point = new PIXI.Point(graphStyle.margin.left, graphStyle.margin.bottom);
    const end = new PIXI.Point(graphStyle.margin.left, app.view.height - graphStyle.margin.top);
    drawLine(app, axisStyle.line, start, end)
}


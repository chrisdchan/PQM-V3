import * as PIXI from 'pixi.js'
import { Curve, GraphDisplay, Line, LineStyle, Point } from '../types/api'
import { Graphics } from '@inlet/react-pixi';

export const drawGraph = (g: PIXI.Graphics, graphDisplay: GraphDisplay | null) => {
    if(graphDisplay) {
        g.clear();
        testDraw(g);
    }
}

const drawLine = (g: PIXI.Graphics, line: Line, style: LineStyle) => {
    g.lineStyle(style.width, style.color, 1);
    g.moveTo(line.start.x, line.start.y);
    g.lineTo(line.end.x, line.end.y);
}

const testDraw = (g: PIXI.Graphics) => {
    g.beginFill(0xff0000);
    g.drawRect(0, 0, 100, 100);
    g.endFill();
}

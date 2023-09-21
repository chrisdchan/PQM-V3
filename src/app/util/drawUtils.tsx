import * as PIXI from 'pixi.js'
import type { LineStyle } from '../types/graphStyle';

/**
 * Draws line on canvas assumming origin is bottom left corner
 * @param app 
 * @param lineStyle 
 * @param start 
 * @param end 
 */
export const drawLine = (g: PIXI.Graphics, lineStyle: LineStyle, start: PIXI.Point, end: PIXI.Point) => {
    const line = new PIXI.Graphics();
    line.lineStyle(lineStyle.width, lineStyle.color, 1)
    line.moveTo(start.x, app.view.height - start.y)
    line.lineTo(end.x, app.view.height - end.y)
    line.closePath()
    app.stage.addChild(line);
};
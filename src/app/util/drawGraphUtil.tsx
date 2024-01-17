import * as PIXI from 'pixi.js'
import {Curve, GraphDisplay, LineStyle, StructureDisplay, StructureDisplayStyle} from '../types/api'
import {drawLine} from './drawUtil';
import {WindowDimensions} from '../lib/graph';

export const drawGraph = (g: PIXI.Graphics, dims: WindowDimensions, graphDisplay: GraphDisplay) => {
    graphDisplay.structures.forEach((structureDisplay) => drawStructure(g, structureDisplay, dims, graphDisplay))
}

const drawStructure = (g: PIXI.Graphics, structureDisplay: StructureDisplay, dims: WindowDimensions, graphDisplay: GraphDisplay) => {
    console.log(structureDisplay);
    let structureDisplayStyle = structureDisplay.displayProperties.style;
    drawCurve(g, structureDisplay.curve, structureDisplayStyle, dims, graphDisplay)
}

const drawCurve = (g: PIXI.Graphics, curve: Curve, style: StructureDisplayStyle, dims: WindowDimensions, graphDisplay: GraphDisplay) => {
    let lineStyle: LineStyle = {color: style.color, width: 1}
    for (let i = 0; i < curve.lines.length; i++) {
        drawLine(g, curve.lines[i], lineStyle, dims, graphDisplay);
    }
}


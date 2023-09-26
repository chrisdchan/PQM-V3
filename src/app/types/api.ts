export type GraphDisplay = {
    id: string,
    structures: StructureDisplay[],
    graphDisplayProperties: GraphDisplayProperties,
    graphDisplayStyle: GraphDisplayStyle,
}

export type StructureDisplay = {
    id: string,
    curve: Curve,
    structureDisplayProperties: StructureDisplayProperties,
    structureDisplayStyle: StructureDisplayStyle,
}

export type GraphDisplayProperties = {
    titleName: string,
    xAxisDisplayProperties: AxisDisplayProperties,
    yAxisDisplayProperties: AxisDisplayProperties
}

export type StructureDisplayProperties = {
    lineType: LineType
    resolution: number
}

export enum LineType {
    SOLID,
    DOTTED,
    DASHED
}

export type AxisDisplayProperties = {
    name: string
    start: number,
    end: number,
    numTicks: number,
    tickGap: number,
    percision: number,
}

// Attributes that are aesthetic only and require no backend logic
export type GraphDisplayStyle = {
    outerColor: string,
    innerColor: string,
    margin: Margin,
    titleStyle: LabelStyle,
    xAxisStyle: AxisStyle,
    yAxisStyle: AxisStyle
}

// Attributes that are aesthetic only and require no backend logic
export type StructureDisplayStyle = {
    color: string
}

export type Margin = {
    left: number,
    right: number,
    top: number,
    bottom: number
}

export type LabelStyle = {
    textColor: string,
    fontSize: number,
    xOffset: number,
    yOffset: number
}

export type AxisStyle = {
    lineStyle: LineStyle,
    titleStyle: LabelStyle,
    tickLineStyle: TickLineStyle,
    tickLabelStyle: LabelStyle
}

export type LineStyle = {
    width: number,
    color: string,
}

export type TickLineStyle = {
    lineStyle: LineStyle,
    length: number,
}
export type Curve = {
    lines: Line[],
}

export type Line = {
    start: Point,
    end: Point
}

export type Point = {
    x: number,
    y: number,
}
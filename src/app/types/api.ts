export type ModelDisplay = {
    id: string,
}

export type GraphDisplay = {
    id: string,
    graphType: GraphType,
    structures: StructureDisplay[],
    displayProperties: GraphDisplayProperties,
}

export enum GraphType {
    CURRENT_DENSITY,
    E_FIELD,
    SAR
}

export type StructureDisplay = {
    id: string,
    curve: Curve,
    displayProperties: StructureDisplayProperties,
}

export type GraphDisplayProperties = {
    titleName: string,
    xAxisDisplayProperties: AxisDisplayProperties,
    yAxisDisplayProperties: AxisDisplayProperties,
    style: GraphDisplayStyle
}

export type StructureDisplayProperties = {
    lineType: LineType
    resolution: number
    style: StructureDisplayStyle
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

export type StructureTableDisplay = {
    structureName: string,
    volume: number,
    areaMap: Record<string, number>,
    cc: number
}

export type GraphTableDisplay = {
    structureTableDisplays: StructureTableDisplay[]
}
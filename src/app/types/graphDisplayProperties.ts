import { AxisStyle } from "./graphStyle"

export type GraphDisplayProperties = {
    titleName: string,
    xAxisDisplayProperties: AxisDisplayProperties,
    yAxisDisplayProperties: AxisDisplayProperties
}

export type AxisDisplayProperties = {
    name: string
    start: number,
    end: number,
    numTicks: number,
    tickGap: number,
    percision: number,
    style: AxisStyle,
}


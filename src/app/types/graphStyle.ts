export type GraphStyle = {
    outerColor: string,
    innerColor: string,
    margin: Margin,
    titleStyle: LabelStyle,
    xAxis: AxisStyle,
    yAxis: AxisStyle
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
    line: LineStyle,
    title: LabelStyle,
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








export type StructureDisplayProperties = {
    name: string,
    lineType: LineType
    start: number
    end: number
    resolution: number
}

export enum LineType {
    solid,
    dotted,
    dashed
}
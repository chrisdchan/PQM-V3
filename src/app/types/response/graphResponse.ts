import { GraphDisplayProperties } from "../graphDisplayProperties"
import { StructureRespone } from "./structureResponse"

export type GraphResponse = {
    structures: StructureRespone[]
    graphDisplayProperties: GraphDisplayProperties
}
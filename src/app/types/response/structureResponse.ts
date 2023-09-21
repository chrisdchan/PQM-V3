import { StructureDisplayProperties } from "../structureDisplayProperties"
import { Curve } from "./curve"

export type StructureRespone = {
    name: String,
    curve: Curve
    structureProperties: StructureDisplayProperties
}
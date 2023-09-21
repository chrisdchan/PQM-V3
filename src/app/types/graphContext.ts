import type * as PIXI from 'pixi.js'
import type { GraphStyle } from './graphStyle'
import type { GraphProperties } from './graphDisplayProperties'

export type GraphContext = {
    app: PIXI.Application,
    style: GraphStyle,
    properties: GraphProperties
}
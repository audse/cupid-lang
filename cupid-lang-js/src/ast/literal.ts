import { Base } from './ast'
import { Kind } from './kind'


export interface Literal extends Base<Kind.Literal> {
    value: LiteralValue
}

export type LiteralValue = number | string | null | [number, number] | boolean
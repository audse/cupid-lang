import { Base } from './ast'
import { Kind } from './kind'

export interface Ident extends Base<Kind.Ident> {
    name: string
}
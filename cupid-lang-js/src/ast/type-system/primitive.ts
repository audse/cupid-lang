import { Base } from '../ast'
import { Kind } from '../kind'
import { TypeKind } from './typekind'

export interface Primitive extends Base<Kind.Type, TypeKind.Primitive> {
    name: string
}
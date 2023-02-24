import { Base } from '../ast'
import { Kind } from '../kind'
import { TypeKind } from './typekind'

export interface Variable extends Base<Kind.Type, TypeKind.Variable> {
    name: string
}
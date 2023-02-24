import { Scope } from '@/scope'
import { TypeKind } from './index'
import { Kind } from './kind'


export type Base<K extends Kind, T extends TypeKind | null = null> = TypeBase<T> & {
    kind: K
    source: number
}

type TypeBase<T extends TypeKind | null = null> = T extends TypeKind ? {
    typeKind: T
} : {}

export type Scoped = {
    scope: Scope
}

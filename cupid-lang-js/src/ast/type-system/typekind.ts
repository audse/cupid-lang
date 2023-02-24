import { isKind } from '../kind'

export enum TypeKind {
    Fun = 'fun',
    Instance = 'instance',
    Map = 'map',
    Primitive = 'primitive',
    // Resolved = 'resolved',
    Struct = 'struct',
    Sum = 'sum',
    Unknown = 'unknown',
    // Unresolved = 'unresolved',
    Variable = 'variable',
}

export type HasTypeKind = { typeKind: string }

export function isTypeKind<E extends V, V extends HasTypeKind = HasTypeKind> (kind: string, value: V): value is E {
    return value.typeKind === kind
}

export function isTypeInstance<E extends V, V extends HasTypeKind = HasTypeKind> (value: V): value is E {
    return isTypeKind<E>(TypeKind.Instance, value)
}
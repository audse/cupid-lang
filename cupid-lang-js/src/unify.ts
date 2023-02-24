import { Type, Expr, Field } from '@/passes/@types/5-pre-check-types'
import { Kind, TypeKind } from '@/ast'
import { err, ErrorCode } from './error'

const Ty = TypeKind
type Primitive = Type<TypeKind.Primitive>
type Fun = Type<TypeKind.Fun>
type Map = Type<TypeKind.Map>
type Struct = Type<TypeKind.Struct>
type Sum = Type<TypeKind.Sum>


function cannotUnify (a: Type, b: Type, message: string = '') {
    console.trace()
    return err(ErrorCode.TypeMismatch, message, a)
}

export function unify (a: Type | null, b: Type | null): Type {
    if (a === null && b === null) return null as unknown as Type
    if (a === null && b) return b
    if (a && b === null) return a
    if (a === null || b === null) throw err(ErrorCode.Unreachable, '', {} as any)

    const variable = unifyVariable(a, b)
    if (variable) return variable

    if (a.typeKind === Ty.Primitive && b.typeKind === Ty.Primitive) return unifyPrimitive(a, b)
    if (a.typeKind === Ty.Fun && b.typeKind === Ty.Fun) return unifyFun(a, b)
    if (a.typeKind === Ty.Struct && b.typeKind === Ty.Struct) return unifyStructSum(a, b)
    if (a.typeKind === Ty.Sum && b.typeKind === Ty.Sum) return unifyStructSum(a, b)
    if (a.typeKind === Ty.Map && b.typeKind === Ty.Map) return unifyMap(a, b)
    if (a.typeKind === Ty.Unknown || b.typeKind === Ty.Unknown) return unifyUnknown(a, b)

    throw cannotUnify(a, b)
}

function unifyVariable (a: Type, b: Type): Type | null {
    const aIsVar = a.typeKind === Ty.Variable
    const bIsVar = b.typeKind === Ty.Variable
    if (!aIsVar && bIsVar) return a
    if (aIsVar && !bIsVar) return b
    if (aIsVar && bIsVar) return a
    return null
}

function unifyFun (a: Fun, b: Fun): Fun {
    const params: Field[] = []
    for (let i = 0; i < a.params.length; i++) {
        const paramA = a.params[i], paramB = b.params[i]
        params.push({ ident: paramA.ident, type: unify(paramA.type, paramB.type) })
    }
    const returns = unify(a.returns, b.returns)
    return { ...a, params, returns }
}

function unifyMap (a: Map, b: Map): Map {
    return {
        ...a,
        keys: unify(a.keys, b.keys),
        values: unify(a.values, b.values)
    }
}

function unifyPrimitive (a: Primitive, b: Primitive): Primitive {
    if (a.name === b.name) return a
    throw cannotUnify(a, b)
}


function unifyStructSum<T extends Struct | Sum> (a: T, b: T): T {
    if (a.fields.length !== b.fields.length) throw cannotUnify(a, b, 'incorrect number of fields')
    const fields: Field[] = []
    for (let i = 0; i < a.fields.length; i++) {
        const fieldA = a.fields[i]
        const fieldB = b.fields.find(field => field.ident.name === fieldA.ident.name)
        if (fieldA && fieldB) fields.push({ ...fieldA, type: unify(fieldA.type, fieldB.type) })
    }
    return { ...a, fields }
}

function unifyUnknown (a: Type, b: Type): Type {
    if (a.typeKind === TypeKind.Unknown) return b
    return a
}
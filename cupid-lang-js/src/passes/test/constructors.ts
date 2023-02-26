import { Expr, Type, AnyTypeKind } from '../@types/1-pre-create-scope'
import { Kind, TypeKind } from '@/ast'


type Make<K extends Kind, Keys extends Partial<keyof Expr<K>>> = (
    (args: { [Key in Keys]: Expr<K>[Key] }, source?: number) => Expr<K>
)

type MakeType<T extends AnyTypeKind, Keys extends Partial<keyof Type<T>>> = (
    (args: { [Key in Keys]: Type<T>[Key] }, source?: number) => Type<T>
)

export const assign: Make<Kind.Assign, 'ident' | 'value'> = ({ ident, value }, source = -1) => ({
    kind: Kind.Assign,
    ident,
    value,
    source
})
export const binop: Make<Kind.BinOp, 'left' | 'right' | 'op'> = ({ left, right, op }, source = -1) => ({
    kind: Kind.BinOp,
    source,
    left,
    right,
    op
})
export const block: Make<Kind.Block, 'exprs'> = ({ exprs }, source = -1) => ({ kind: Kind.Block, source, exprs })
export const call: Make<Kind.Call, 'fun' | 'args'> = ({ fun, args }, source = -1) => ({ kind: Kind.Call, fun, args, source })
export const decl: Make<Kind.Decl, 'ident' | 'type' | 'value'> = ({ ident, type, value }, source = -1) => ({
    kind: Kind.Decl,
    ident,
    value,
    type,
    source,
})
export const fun: Make<Kind.Fun, 'params' | 'body' | 'returns'> = ({ params, body, returns }, source = -1) => ({
    kind: Kind.Fun,
    params,
    returns,
    body,
    source
})
export const ident: Make<Kind.Ident, 'name'> = ({ name }, source = -1) => ({ kind: Kind.Ident, name, source })
export const ifstmt: Make<Kind.IfStmt, 'condition' | 'body' | 'elseBody'> = ({ condition, body, elseBody }, source = -1) => ({
    kind: Kind.IfStmt,
    condition,
    body,
    elseBody,
    source
})
export const literal: Make<Kind.Literal, 'value'> = ({ value }, source = -1) => ({ kind: Kind.Literal, value, source })
export const map: Make<Kind.Map, 'entries'> = ({ entries }, source = -1) => ({ kind: Kind.Map, entries, source })
export const property: Make<Kind.Property, 'parent' | 'property'> = ({ parent, property }, source = -1) => ({
    kind: Kind.Property,
    parent,
    property,
    source
})
export const typeConstructor: Make<Kind.TypeConstructor, 'ident' | 'params' | 'value'> = ({ ident, params, value }, source = -1) => ({
    kind: Kind.TypeConstructor,
    ident,
    params,
    value,
    source
})
export const unop: Make<Kind.UnOp, 'expr' | 'op'> = ({ expr, op }, source = -1) => ({
    kind: Kind.UnOp,
    source,
    expr,
    op
})

export const funType: MakeType<TypeKind.Fun, 'params' | 'returns'> = ({ params, returns }, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Fun,
    params,
    returns,
    source
})
export const typeInstance: MakeType<TypeKind.Instance, 'ident' | 'args'> = ({ ident, args }, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Instance,
    ident,
    args,
    source
})
export const primitive: MakeType<TypeKind.Primitive, 'name'> = ({ name }, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Primitive,
    name,
    source
})
export const unknown: MakeType<TypeKind.Unknown, never> = (_args = {}, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Unknown,
    source
})
export const struct: MakeType<TypeKind.Struct, 'fields'> = ({ fields }, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Struct,
    fields,
    source
})
export const sum: MakeType<TypeKind.Sum, 'fields'> = ({ fields }, source = -1) => ({
    kind: Kind.Type,
    typeKind: TypeKind.Sum,
    fields,
    source
})
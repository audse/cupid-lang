import { ErrorCode, err } from '@/error'
import * as input from './@types/3-pre-resolve-types'
import * as output from './@types/4-pre-infer-types'
import { isExpr, isTypeConstructor, isTypeInstance, Kind, TypeKind, isKind } from '@/ast'
import { notFound } from '@/error/constructors'

type ResolveTypes<Input extends Kind> = (
    (expr: input.Expr<Input>) => output.Expr<Input>
)

type Methods = {
    [K in Kind]: ResolveTypes<K>
}

function notType (expr: output.Expr) {
    console.trace()
    return err(ErrorCode.NotAType, 'expected a type', expr)
}

function isType (expr: output.Expr): expr is output.Type {
    return isKind<output.Type>(Kind.Type, expr)
}

function resolveFieldTypes (field: input.Field): output.Field {
    const ident = resolveTypes<Kind.Ident>(field.ident)
    const type = resolveTypes<Kind.Type>(field.type)
    const annotated = ident.scope.annotate(ident, { type })
    if (!annotated) throw notFound(ident)
    return { ident, type }
}

function resolveTypeIdent (ident: output.Expr<Kind.Ident>): output.Type {
    const value = ident.scope.lookup(ident)
    if (!value) throw notFound(ident)
    const type = value.value
    if (!type || !isType(type)) throw notType(ident)
    return type
}

function resolveTypeConstructor (constructor: input.Expr<Kind.TypeConstructor>, args: output.Type[]): output.Type {
    for (let i = 0; i < constructor.params.length; i++) {
        const param = constructor.params[i], arg = args[i]
        if (!param || !arg) throw err(ErrorCode.IncorrectNumberOfArgs, '', constructor)
        param.scope.annotate(param, { type: arg, value: arg }) // TODO { type } or { value } 
    }
    return resolveTypes<Kind.Type>(constructor.value)
}

const map: Methods = {

    [Kind.Assign]: assign => ({
        ...assign,
        ident: resolveTypes<Kind.Ident>(assign.ident),
        value: resolveTypes(assign.value),
    }),

    [Kind.BinOp]: binop => ({
        ...binop,
        left: resolveTypes(binop.left),
        right: resolveTypes(binop.right),
        scope: binop.scope
    }),

    [Kind.Block]: block => ({
        ...block,
        scope: block.scope,
        exprs: block.exprs.map(resolveTypes),
    }),

    [Kind.Decl]: decl => {
        const ident = resolveTypes<Kind.Ident>(decl.ident)
        const value = resolveTypes(decl.value)
        const type = resolveTypes<Kind.Type>(decl.type)
        ident.scope.annotate(ident, { value })
        return { ...decl, ident, value, type, scope: decl.scope }
    },

    [Kind.Call]: call => {
        return {
            ...call,
            fun: resolveTypes(call.fun),
            args: call.args.map(resolveTypes),
        }
    },

    [Kind.Fun]: fun => ({
        ...fun,
        params: fun.params.map(resolveFieldTypes),
        returns: resolveTypes<Kind.Type>(fun.returns),
        body: resolveTypes(fun.body),
    }),

    [Kind.Ident]: ident => {
        if (!ident.scope.lookup(ident)) throw notFound(ident)
        return ident
    },

    [Kind.IfStmt]: ifstmt => ({
        ...ifstmt,
        condition: resolveTypes(ifstmt.condition),
        body: resolveTypes(ifstmt.body),
        elseBody: ifstmt.elseBody ? resolveTypes(ifstmt.elseBody) : undefined,
    }),

    [Kind.Literal]: literal => ({
        ...literal,
        scope: literal.scope
    }),

    [Kind.Map]: map => {
        const entries: [output.Expr<Kind.Literal>, output.Expr][] = map.entries.map(([key, val]) => {
            const value = resolveTypes(val)
            // if (key.kind === Kind.Ident) key.scope.annotate(key as output.Expr<Kind.Ident>, { value })
            return [resolveTypes<Kind.Literal>(key), value]
        })
        return { ...map, entries }
    },

    [Kind.Property]: property => ({
        ...property,
        parent: resolveTypes(property.parent),
        property: resolveTypes(property.property),
        scope: property.scope
    }),

    [Kind.Type]: (type): output.Expr<Kind.Type> => {
        switch (type.typeKind) {
            case TypeKind.Fun: return {
                ...type,
                params: type.params.map(resolveFieldTypes),
                returns: resolveTypes<Kind.Type>(type.returns),
            }
            case TypeKind.Instance: {
                const ident = resolveTypes<Kind.Ident>(type.ident)
                const args = type.args.map(resolveTypes<Kind.Type>)
                const constructor = type.scope.lookup(ident)
                if (!constructor) throw notType(type)
                if (constructor.value) {
                    if (isExpr<input.Expr<Kind.TypeConstructor>>(constructor.value) && isTypeConstructor(constructor.value)) {
                        return resolveTypeConstructor(constructor.value, args)
                    }
                    if (isTypeInstance(constructor.value)) return resolveTypes<Kind.Type>(constructor.value)
                    if (isType(constructor.value)) return constructor.value
                }
                console.log(ident, constructor)
                throw notType(ident)
            }
            case TypeKind.Map: return {
                ...type,
                keys: resolveTypes<Kind.Type>(type.keys),
                values: resolveTypes<Kind.Type>(type.values)
            }
            case TypeKind.Primitive: return type
            case TypeKind.Struct:
            case TypeKind.Sum: return {
                ...type,
                fields: type.fields.map(resolveFieldTypes),
            }
            case TypeKind.Unknown: return type
            case TypeKind.Variable: {
                const typeIdent = { ...type, kind: Kind.Ident, name: type.name } as output.Expr<Kind.Ident>
                return resolveTypeIdent(typeIdent)
            }
        }
    },

    [Kind.TypeConstructor]: constructor => {
        if (constructor.params.length) return constructor
        const result = resolveTypes<Kind.Type>(constructor.value)
        constructor.scope.annotate(constructor.ident, { value: result })
        const { scope, source, kind } = constructor
        return { scope, source, kind }
    },

    [Kind.UnOp]: unop => ({
        ...unop,
        expr: resolveTypes(unop.expr),
        scope: unop.scope
    })

}

export function resolveTypes<
    Input extends Kind = Kind
> (expr: input.Expr<Input>): output.Expr<Input> {
    if (!expr) console.trace()
    const kind = expr.kind as Input
    if (!(kind in map)) {
        console.log(expr)
        console.trace()
    }
    return map[kind](expr)
}
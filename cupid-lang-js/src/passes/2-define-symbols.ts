import { isCall, isIdent, isLiteral, Kind, TypeKind } from '@/ast'
import { isMap } from 'util/types'
import * as input from './@types/2-pre-define-symbols'
import * as output from './@types/3-pre-resolve-types'


type DefineSymbols<Input extends Kind> = (
    (expr: input.Expr<Input>) => output.Expr<Input>
)

type Methods = {
    [K in Kind]: DefineSymbols<K>
}

function defineFieldSymbol (field: input.Field): output.Field {
    const ident = defineSymbols<Kind.Ident>(field.ident)
    const type = defineSymbols<Kind.Type>(field.type)
    ident.scope.define(ident, { type })
    return { ident, type }
}

const map: Methods = {

    [Kind.Assign]: assign => ({
        ...assign,
        ident: defineSymbols<Kind.Ident>(assign.ident),
        value: defineSymbols(assign.value),
    }),

    [Kind.BinOp]: binop => ({
        ...binop,
        left: defineSymbols(binop.left),
        right: defineSymbols(binop.right),
    }),

    [Kind.Block]: block => ({
        ...block,
        exprs: block.exprs.map(defineSymbols),
    }),

    [Kind.Decl]: decl => {
        const ident = defineSymbols<Kind.Ident>(decl.ident)
        const value = defineSymbols(decl.value)
        ident.scope.define(ident, { value })
        return {
            ...decl,
            ident,
            value,
            type: defineSymbols<Kind.Type>(decl.type),
        }
    },

    [Kind.Call]: call => ({
        ...call,
        fun: defineSymbols(call.fun),
        args: call.args.map(defineSymbols),
    }),

    [Kind.Fun]: fun => ({
        ...fun,
        params: fun.params.map(defineFieldSymbol),
        returns: defineSymbols<Kind.Type>(fun.returns),
        body: defineSymbols(fun.body),
    }),

    [Kind.Ident]: ident => ident,

    [Kind.IfStmt]: ifstmt => ({
        ...ifstmt,
        condition: defineSymbols(ifstmt.condition),
        body: defineSymbols(ifstmt.body),
        ...ifstmt.elseBody && { elseBody: defineSymbols(ifstmt.elseBody) },
    }),

    [Kind.Literal]: literal => literal,

    [Kind.Map]: map => {
        const entries: [output.Expr<Kind.Literal>, output.Expr][] = map.entries.map(([key, value]) => {
            // const val = defineSymbols(value)
            // if (key.kind === Kind.Ident) key.scope.define(key as output.Expr<Kind.Ident>, { value: val })
            return [defineSymbols<Kind.Literal>(key), defineSymbols(value)]
        })
        return { ...map, entries }
    },

    [Kind.Property]: prop => {
        const parent = defineSymbols(prop.parent)
        const property = defineSymbols(prop.property)

        // if (isIdent<output.Expr<Kind.Ident>>(parent)) {
        //     const parentValue = prop.scope.lookup(parent)
        //     if (parentValue) scope = parentValue.value.scope
        //     console.log(parentValue)
        // }

        // function updateScope (expr: any) {
        //     if (typeof expr === 'object' && expr && 'scope' in expr) {
        //         for (const value of Object.values(expr)) updateScope(value)
        //         expr.scope = scope
        //     }
        //     return expr
        // }
        // updateScope(property)
        // console.log(scope)

        return { ...prop, parent, property }
    },

    [Kind.Type]: type => {
        switch (type.typeKind) {
            case TypeKind.Fun: return {
                ...type,
                params: type.params.map(param => defineFieldSymbol(param)),
                returns: defineSymbols<Kind.Type>(type.returns),
            }
            case TypeKind.Instance: return {
                ...type,
                ident: defineSymbols<Kind.Ident>(type.ident),
                args: type.args.map(defineSymbols<Kind.Type>)
            }
            case TypeKind.Map: return {
                ...type,
                keys: map.type(type.keys),
                values: map.type(type.values)
            }
            case TypeKind.Struct:
            case TypeKind.Sum:
                return {
                    ...type,
                    fields: type.fields.map(defineFieldSymbol),
                }
            case TypeKind.Primitive:
            case TypeKind.Variable:
            case TypeKind.Unknown: return type
        }
    },

    [Kind.TypeConstructor]: constructor => {
        const value = {
            ...constructor,
            params: constructor.params.map(param => {
                const ident = defineSymbols<Kind.Ident>(param)
                const type: output.Type<TypeKind.Variable> = {
                    kind: Kind.Type,
                    typeKind: TypeKind.Variable,
                    source: param.source,
                    name: param.name,
                    scope: param.scope
                }
                ident.scope.define(ident, { type })
                return ident
            }),
            value: defineSymbols<Kind.Type>(constructor.value)
        }
        if (value.params.length) constructor.scope.parent?.define(value.ident, { value })
        else constructor.scope.parent?.define(value.ident, { value: value.value })
        return value
    },

    [Kind.UnOp]: unop => ({
        ...unop,
        expr: defineSymbols(unop.expr),
    })

}

export function defineSymbols<
    Input extends Kind = Kind
> (expr: input.Expr<Input>): output.Expr<Input> {
    const kind = expr.kind as Input
    return map[kind](expr)
}
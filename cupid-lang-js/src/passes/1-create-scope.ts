import { Scope } from '@/scope'
import { Kind, TypeKind } from '@/ast'
import * as input from './@types/1-pre-create-scope'
import * as output from './@types/2-pre-define-symbols'

type CreateScope<Input extends Kind> = (
    (expr: input.Expr<Input>, scope: Scope) => output.Expr<Input>
)

type Methods = {
    [K in Kind]: CreateScope<K>
}

function createFieldScope (field: input.Field, scope: Scope): output.Field {
    return {
        ident: createScope<Kind.Ident>(field.ident, scope),
        type: createScope<Kind.Type>(field.type, scope),
    }
}

const map: Methods = {

    [Kind.Assign]: (assign, scope) => ({
        ...assign,
        ident: createScope<Kind.Ident>(assign.ident, scope),
        value: createScope(assign.value, scope),
        scope
    }),

    [Kind.BinOp]: (binop, scope) => ({
        ...binop,
        left: createScope(binop.left, scope),
        right: createScope(binop.right, scope),
        scope
    }),

    [Kind.Block]: (block, scope) => {
        const subscope = scope.subscope()
        return {
            ...block,
            exprs: block.exprs.map(expr => createScope(expr, subscope)),
            scope: subscope
        }
    },

    [Kind.Decl]: (decl, scope) => ({
        ...decl,
        ident: createScope<Kind.Ident>(decl.ident, scope),
        value: createScope(decl.value, scope),
        type: createScope<Kind.Type>(decl.type, scope),
        scope,
    }),

    [Kind.Call]: (call, scope) => {
        const subscope = scope.subscope()
        return {
            ...call,
            fun: createScope(call.fun, subscope),
            args: call.args.map(arg => createScope(arg, subscope)),
            scope: subscope,
        }
    },

    [Kind.Fun]: (fun, scope) => {
        const subscope = scope.subscope()
        return {
            ...fun,
            params: fun.params.map(param => createFieldScope(param, subscope)),
            body: createScope(fun.body, subscope),
            returns: createScope<Kind.Type>(fun.returns, subscope),
            scope: subscope
        }
    },

    [Kind.Ident]: (ident, scope) => ({
        ...ident,
        scope,
    }),

    [Kind.IfStmt]: (ifStmt, scope) => ({
        ...ifStmt,
        scope,
        condition: createScope(ifStmt.condition, scope),
        body: createScope(ifStmt.body, scope),
        elseBody: ifStmt.elseBody ? createScope(ifStmt.elseBody, scope) : undefined,
    }),

    [Kind.Literal]: (literal, scope) => ({
        ...literal,
        scope
    }),

    [Kind.Map]: (map, scope) => {
        const subscope = scope.subscope()
        return {
            ...map,
            entries: map.entries.map(([key, value]) => [createScope<Kind.Literal>(key, subscope), createScope(value, subscope)]),
            scope: subscope
        }
    },

    [Kind.Property]: (property, scope) => ({
        ...property,
        parent: createScope(property.parent, scope),
        property: createScope(property.property, scope),
        scope
    }),

    [Kind.Type]: (type, scope) => {
        const subscope = scope.subscope()
        switch (type.typeKind) {
            case TypeKind.Fun: return {
                ...type,
                params: type.params.map(param => createFieldScope(param, subscope)),
                returns: createScope<Kind.Type>(type.returns, subscope),
                scope: subscope
            }
            case TypeKind.Instance: return {
                ...type,
                ident: createScope<Kind.Ident>(type.ident, subscope),
                args: type.args.map(arg => createScope<Kind.Type>(arg, subscope)),
                scope: subscope,
            }
            case TypeKind.Map: return {
                ...type,
                keys: createScope<Kind.Type>(type.keys, subscope),
                values: createScope<Kind.Type>(type.values, subscope),
                scope: subscope
            }
            case TypeKind.Primitive: return { ...type, scope: subscope }
            case TypeKind.Struct:
            case TypeKind.Sum: return {
                ...type,
                scope: subscope,
                fields: type.fields.map(field => createFieldScope(field, subscope)),
            }
            case TypeKind.Unknown: return { ...type, scope: subscope }
        }
    },

    [Kind.TypeConstructor]: (constructor, scope) => {
        const subscope = scope.subscope()
        return {
            ...constructor,
            ident: createScope<Kind.Ident>(constructor.ident, subscope),
            params: constructor.params.map(param => createScope<Kind.Ident>(param, subscope)),
            value: createScope<Kind.Type>(constructor.value, subscope),
            scope: subscope
        }
    },

    [Kind.UnOp]: (unop, scope) => ({
        ...unop,
        expr: createScope(unop.expr, scope),
        scope
    }),

}

export function createScope<
    Input extends Kind = Kind
> (expr: input.Expr<Input>, scope: Scope): output.Expr<Input> {
    const kind = expr.kind as Input
    return map[kind](expr, scope)
}
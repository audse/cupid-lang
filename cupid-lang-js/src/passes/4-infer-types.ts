import { ErrorCode, err } from '@/error'
import { Option } from '@/types'
import * as input from './@types/4-pre-infer-types'
import * as output from './@types/5-pre-check-types'
import { primitive } from './test/constructors'
import { isIdent, isLiteral, isMap, Kind, TypeKind } from '@/ast'
import { unify } from '@/unify'


type InferTypes<Input extends Kind> = (
    (expr: input.Expr<Input>) => output.Expr<Input>
)

type Methods = {
    [K in Kind]: InferTypes<K>
}

function inferFieldTypes (field: input.Field): output.Field {
    const ident = inferTypes<Kind.Ident>(field.ident)
    const type = inferTypes<Kind.Type>(field.type)
    ident.scope.annotate(ident, { type })
    return { ident: { ...ident, inferredType: type }, type }
}

function makeType<T extends output.AnyTypeKind> (parent: Omit<output.Expr, 'inferredType'>, kind: T, args: Omit<output.Type<T>, 'scope' | 'source' | 'kind' | 'typeKind' | 'inferredType'>): output.Type<T> {
    return {
        typeKind: kind,
        kind: Kind.Type,
        scope: parent.scope,
        source: -1,
        inferredType: null,
        ...args
    } as output.Type<T>
}

function makePrimitive (parent: Omit<output.Expr, 'inferredType'>, name: string) {
    return makeType(parent, TypeKind.Primitive, { name })
}

function typeKind (parent: Omit<output.Expr, 'inferredType'>): output.Type {
    const type = primitive({ name: 'type' })
    return { ...type, scope: parent.scope, inferredType: null }
}

const compareOp = new Set(['not', '<', '<=', '>', '>=', 'is', '==', '!='])

const map: Methods = {

    [Kind.Assign]: assign => ({
        ...assign,
        ident: inferTypes<Kind.Ident>(assign.ident),
        value: inferTypes(assign.value),
        inferredType: makePrimitive(assign, 'none')
    }),

    [Kind.BinOp]: binop => {
        const left = inferTypes(binop.left)
        const right = inferTypes(binop.right)

        const inferredType = (() => {
            const type = unify(left.inferredType, right.inferredType)
            if (compareOp.has(binop.op)) return makePrimitive(binop, 'bool')
            return type
        })()

        return { ...binop, left, right, inferredType }
    },

    [Kind.Block]: block => {
        const exprs = block.exprs.map(expr => inferTypes(expr))
        const inferredType = (
            exprs.length ? exprs[exprs.length - 1].inferredType
                : makePrimitive(block, 'none')
        ) as output.Type
        if (!inferredType) console.log(exprs)
        return { ...block, exprs, inferredType }
    },

    [Kind.Decl]: decl => {
        const ident = inferTypes<Kind.Ident>(decl.ident)
        const value = inferTypes(decl.value)
        const inferredType = value.inferredType || typeKind(value)
        const type = (() => {
            try {
                const result = inferTypes<Kind.Type>(decl.type)
                return result
            } catch (error) {
                return inferredType
            }
        })()
        ident.scope.annotate(ident, { type: inferredType })
        return {
            ...decl,
            ident,
            value,
            type,
            inferredType: makePrimitive(value, 'none')
        }
    },

    [Kind.Call]: call => {
        const fun = inferTypes(call.fun)
        const inferredType = (fun.inferredType?.typeKind === TypeKind.Fun ? fun.inferredType.returns : null) as output.Type
        return {
            ...call,
            fun,
            args: call.args.map(inferTypes),
            inferredType
        }
    },

    [Kind.Fun]: fun => {
        const params = fun.params.map(inferFieldTypes)
        const body = inferTypes(fun.body)
        const returns = inferTypes<Kind.Type>(fun.returns)
        return {
            ...fun,
            params,
            body,
            returns,
            inferredType: makeType(body, TypeKind.Fun, {
                params: params,
                returns: body.inferredType || returns,
            }),
        }
    },

    [Kind.Ident]: ident => {
        const symbol = ident.scope.lookup<input.Expr<Kind.Ident>, output.Type>(ident)
        if (!symbol) throw err(ErrorCode.NotFound, '', ident)
        const inferredType = symbol.type
        return { ...ident, inferredType }
    },

    [Kind.IfStmt]: ifstmt => {
        const condition = inferTypes(ifstmt.condition)
        const body = inferTypes(ifstmt.body)
        const elseBody = ifstmt.elseBody ? inferTypes(ifstmt.elseBody) : undefined
        console.log(body.inferredType, elseBody?.inferredType)
        const inferredType = unify(body.inferredType, elseBody?.inferredType || null)
        return { ...ifstmt, condition, body, elseBody, inferredType }
    },

    [Kind.Literal]: literal => {
        const inferredType = (() => {
            switch (typeof literal.value) {
                case 'number': case 'bigint': return makePrimitive(literal, 'int')
                case 'boolean': return makePrimitive(literal, 'bool')
                case 'string': return makePrimitive(literal, 'str')
                case 'object': {
                    if (literal.value === null) return makePrimitive(literal, 'none')
                    if (Array.isArray(literal.value)) return makePrimitive(literal, 'decimal')
                }
                default: return makeType(literal, TypeKind.Unknown, {})
            }
        })()
        return { ...literal, inferredType }
    },

    [Kind.Map]: map => {
        const entries: [output.Expr<Kind.Literal>, output.Expr][] = map.entries.map(([key, val]) => {
            const value = inferTypes(val)
            // if (key.kind === Kind.Ident) key.scope.annotate(key as output.Expr<Kind.Ident>, { type: value.inferredType })
            return [inferTypes<Kind.Literal>(key), value]
        })
        const newMap: Omit<output.Expr<Kind.Map>, 'inferredType'> = { ...map, entries }
        const partialType = (
            newMap.entries.length ? {
                keys: newMap.entries[0][0].inferredType || makePrimitive(newMap, 'none'),
                values: newMap.entries[0][1].inferredType || makePrimitive(newMap, 'none'),
            } : {
                keys: makePrimitive(newMap, 'none'),
                values: makePrimitive(newMap, 'none')
            }
        )
        const inferredType = makeType(newMap, TypeKind.Map, partialType)
        return { ...newMap, inferredType }
    },

    [Kind.Property]: property => {
        const parent = inferTypes(property.parent)
        const prop = inferTypes(property.property)

        function getPropertyValue (obj: output.Expr): output.Expr | null {
            if (isMap<output.Expr<Kind.Map>>(obj) && isLiteral<output.Expr<Kind.Literal>>(prop)) {
                const value = obj.entries.find(
                    ([key, value]) => key.value === prop.value
                )
                if (value) return value[1]
                return null
            } else console.log(obj)
            if (isIdent<output.Expr<Kind.Ident>>(obj)) {
                const parentValue = prop.scope.lookup(obj)
                if (parentValue) return getPropertyValue(parentValue.value)
            }
            return null
        }
        const inferredType = getPropertyValue(parent)?.inferredType || makeType(prop, TypeKind.Unknown, {})
        console.log(inferredType)
        return {
            ...property,
            parent,
            property: prop,
            inferredType,
        }
    },

    [Kind.Type]: type => {
        const inferredType = null
        switch (type.typeKind) {
            case TypeKind.Fun: return {
                ...type, inferredType,
                params: type.params.map(inferFieldTypes),
                returns: map.type(type.returns)
            }
            case TypeKind.Map: return {
                ...type, inferredType,
                keys: map.type(type.keys),
                values: map.type(type.values)
            }
            case TypeKind.Primitive: return { ...type, inferredType }
            case TypeKind.Struct:
            case TypeKind.Sum: return {
                ...type, inferredType,
                fields: type.fields.map(field => inferFieldTypes(field)),
            }
            case TypeKind.Unknown: return { ...type, inferredType }
            case TypeKind.Variable: return { ...type, inferredType }
        }
    },

    [Kind.TypeConstructor]: constructor => ({
        ...constructor,
        inferredType: makePrimitive(constructor, 'none')
    }),

    [Kind.UnOp]: unop => {
        const expr = inferTypes(unop.expr)
        const inferredType = expr.inferredType || makeType(unop, TypeKind.Unknown, {})
        return {
            ...unop,
            expr,
            inferredType,
        }
    }
}

export function inferTypes<
    Input extends Kind = Kind
> (expr: input.Expr<Input>): output.Expr<Input> {
    if (!expr) {
        console.trace()
        console.log(expr)
    }
    const kind = expr.kind as Input
    return map[kind](expr)
}
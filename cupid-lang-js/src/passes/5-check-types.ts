import { ErrorCode, err, formatType } from '@/error'
import * as input from './@types/5-pre-check-types'
import * as output from './@types/5-pre-check-types'
import { Kind, TypeKind } from '@/ast'
import { unify } from '@/unify'
import { argTypeMismatch, notFound, typeAnnotationMismatch } from '@/error/constructors'

type Fun = output.Type<TypeKind.Fun>

type CheckTypes<Input extends Kind> = (
    (expr: input.Expr<Input>) => output.Expr<Input>
)

type Methods = {
    [K in Kind]: CheckTypes<K>
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

function checkFieldTypes (field: input.Field): output.Field {
    const ident = checkTypes<Kind.Ident>(field.ident)
    const type = checkTypes<Kind.Type>(field.type)
    ident.scope.annotate(ident, { type })
    return { ident: { ...ident, inferredType: type }, type }
}

const map: Methods = {

    [Kind.BinOp]: binop => {
        const left = checkTypes(binop.left)
        const right = checkTypes(binop.right)
        unify(left.inferredType, right.inferredType)
        return { ...binop, left, right }
    },

    [Kind.Block]: block => ({
        ...block,
        exprs: block.exprs.map(checkTypes)
    }),

    [Kind.Decl]: decl => {
        const ident = checkTypes<Kind.Ident>(decl.ident)
        const value = checkTypes(decl.value)
        const type = (() => {
            try {
                return unify(decl.type, value.inferredType)
            } catch (error) {
                throw typeAnnotationMismatch(decl, decl.type, value.inferredType)
            }
        })()
        ident.scope.annotate(ident, { type })
        return { ...decl, ident, value, type }
    },

    [Kind.Call]: call => {
        const fun = checkTypes(call.fun)
        const args = call.args.map(checkTypes)
        const funType = { ...fun.inferredType as Fun }

        funType.params = funType.params.map((param, i) => {
            try {
                const type = unify(param.type, args[i].inferredType)
                param.ident.scope.annotate(param.ident, { type })
                return { ...param, type }
            } catch (error) {
                throw argTypeMismatch(param, args[i], args[i].inferredType as output.Type)
            }
        })
        if (funType.returns && funType.returns.typeKind === TypeKind.Variable) {
            const value = funType.params[0].ident.scope.lookup({ name: funType.returns.name })
            if (!value) throw notFound({
                kind: Kind.Ident,
                source: funType.returns.source,
                name: funType.returns.name,
            })
            funType.returns = value.value as output.Type
        }
        fun.inferredType = funType
        return { ...call, fun, args }
    },

    [Kind.Fun]: fun => {
        const params = fun.params.map(checkFieldTypes)
        const body = checkTypes(fun.body)
        const returns = checkTypes<Kind.Type>(fun.returns)
        const returnType = unify(body.inferredType, returns)
        const inferredType: Fun = {
            ...fun.inferredType as Fun,
            returns: returnType
        }
        return {
            ...fun,
            params,
            body,
            inferredType
        }
    },

    [Kind.Ident]: ident => ident,

    [Kind.IfStmt]: ifstmt => {
        const condition = checkTypes(ifstmt.condition)
        const body = checkTypes(ifstmt.body)
        const elseBody = ifstmt.elseBody ? checkTypes(ifstmt.elseBody) : undefined
        const asBool = unify(condition.inferredType, makePrimitive(condition, 'bool'))
        return { ...ifstmt, condition: { ...condition, inferredType: asBool }, body, elseBody }
    },

    [Kind.Literal]: literal => literal,

    [Kind.Map]: map => {
        const entries: output.Expr<Kind.Map>['entries'] = (
            map.entries.map(([key, value]) => [checkTypes(key), checkTypes(value)])
        )
        if (map.inferredType.typeKind !== TypeKind.Map) throw err(ErrorCode.TypeMismatch, 'expected map', map.inferredType)
        let keyType = map.inferredType.keys, valueType = map.inferredType.values
        for (const [key, value] of entries) {
            keyType = unify(keyType, key.inferredType)
            valueType = unify(valueType, value.inferredType)
        }
        return {
            ...map,
            entries,
            inferredType: {
                ...map.inferredType,
                typeKind: TypeKind.Map,
                keys: keyType,
                values: valueType
            }
        }
    },

    [Kind.Property]: property => property,

    [Kind.Type]: type => type,

    [Kind.TypeConstructor]: constructor => constructor,

    [Kind.UnOp]: unop => ({
        ...unop,
        expr: checkTypes(unop.expr),
    })
}

export function checkTypes<
    Input extends Kind = Kind
> (expr: input.Expr<Input>): output.Expr<Input> {
    const kind = expr.kind as Input
    return map[kind](expr)
}
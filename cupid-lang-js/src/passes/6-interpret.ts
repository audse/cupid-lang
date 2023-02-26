import { ErrorCode, err } from '@/error'
import { Option } from '@/types'
import { Kind, TypeKind } from '@/ast'
import { Expr, Field, Type } from './@types/5-pre-check-types'

type Value = string | number | boolean | null | Type | Expr<Kind.Fun> | Record<string | number, any>

type Interpret<Input extends Kind> = (
    (expr: Expr<Input>) => Value
)

type Methods = {
    [K in Kind]: Interpret<K>
}

function toLiteral (parent: Expr, value: Value): Expr<Kind.Literal> {
    return {
        ...parent,
        kind: Kind.Literal,
        value: value as Expr<Kind.Literal>['value'],
        inferredType: parent.inferredType as Type
    }
}

const map: Methods = {

    [Kind.Assign]: assign => {
        const value = interpret(assign.value)
        assign.scope.annotate(assign.ident, { value: toLiteral(assign, value) })
        return null
    },

    [Kind.BinOp]: binop => {
        const left = interpret(binop.left)
        const right = interpret(binop.right)
        switch (binop.op) {
            case 'is': case '==': return left === right
            case 'not': case '!=': return left !== right
        }
        if (typeof left === 'number' && typeof right === 'number') {
            switch (binop.op) {
                case '+': return left + right
                case '-': return left - right
                case '*': return left * right
                case '/': return left / right
                case '^': return Math.pow(left, right)
                case '<': return left < right
                case '>': return left > right
                case '<=': return left <= right
                case '>=': return left >= right
                case '<<': return left << right
                case '>>': return left >> right
            }
        }
        if (typeof left === 'boolean' && typeof right === 'boolean') {
            switch (binop.op) {
                case 'and': case '&': return left && right
                case 'or': case '|': return left || right
            }
        }
        throw err(ErrorCode.InvalidOperation, '', binop)
    },

    [Kind.Block]: block => {
        const result = block.exprs.map(interpret).pop()
        return result === undefined ? null : result
    },

    [Kind.Decl]: decl => {
        interpret<Kind.Ident>(decl.ident)
        decl.ident.scope.annotate(decl.ident, { value: toLiteral(decl, interpret(decl.value)) })
        return null
    },

    [Kind.Call]: call => {
        const fun = interpret(call.fun)
        const args = call.args.map(interpret)
        if (fun && typeof fun === 'object' && fun.kind === Kind.Fun) {
            (fun as Expr<Kind.Fun>).params.forEach((param, i) => {
                param.ident.scope.annotate(param.ident, { value: toLiteral(param.ident, args[i]) })
            })
            return interpret((fun as Expr<Kind.Fun>).body)
        }
        throw err(ErrorCode.Unreachable, '', call)
    },

    [Kind.Fun]: fun => fun,

    [Kind.Ident]: ident => interpret(ident.scope.lookup(ident)?.value || null),

    [Kind.IfStmt]: ifstmt => {
        const condition = interpret(ifstmt.condition)
        if (typeof condition !== 'boolean') throw err(ErrorCode.InvalidOperation, 'expected boolean condition', ifstmt.condition)
        if (condition) return interpret(ifstmt.body)
        if (ifstmt.elseBody) return interpret(ifstmt.elseBody)
        return null
    },

    [Kind.Literal]: literal => {
        if (Array.isArray(literal.value)) return parseFloat(literal.value.join('.'))
        else return literal.value
    },

    [Kind.Map]: map => {
        const entries = map.entries.map(([key, value]) => [interpret(key), interpret(value)])
        return Object.fromEntries(entries)
    },

    [Kind.Property]: property => {
        const parent = interpret(property.parent) as Record<string | number, any>

        if (typeof parent === 'object' && property.property.kind === Kind.Literal) {
            return parent[property.property.value as keyof typeof parent]
        }

        return interpret(property.property)
        // if (property.property.kind === Kind.Ident) return parent[property.property.name]
        // already resolved as symbol
        // throw err(ErrorCode.InvalidOperation, '', property)
    },

    [Kind.Type]: type => type,

    [Kind.TypeConstructor]: constructor => null,

    [Kind.UnOp]: unop => {
        const expr = interpret(unop.expr)
        switch (unop.op) {
            case '-': if (typeof expr === 'number') return expr * -1
            case 'not': if (typeof expr === 'boolean') return !expr
        }
        throw err(ErrorCode.InvalidOperation, '', unop)
    }

}

export function interpret<
    Input extends Kind = Kind
> (expr: Expr<Input>): Value {
    if (!expr || !expr.kind) {
        console.trace()
        console.log(expr)
    }
    const kind = expr.kind as Input
    return map[kind](expr)
}
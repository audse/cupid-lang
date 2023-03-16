import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, Fun, Call, Environment, Lookup, Impl, UnOp, Branch, Match } from '@/ast'
import { bracket, paren } from '@/codegen'
import { RuntimeError } from '@/error/index'

type Incomplete = { incomplete: true }
type Value = number | string | boolean | null | Type | Fun | Environment | Incomplete

const incomplete = (): Incomplete => ({ incomplete: true })

export default class Interpreter extends ExprVisitor<Value> {

    visitAssign (assign: Assign): Value {
        assign.ident.accept(this)
        assign.value.accept(this)
        assign.ident.expectSymbol().value = assign.value
        return null
    }

    visitBinOp (binop: BinOp): Value {
        const left = binop.left.accept(this)
        const right = binop.right.accept(this)
        if (typeof left === 'number' && typeof right === 'number') switch (binop.op) {
            case '+': return left + right
            case '-': return left - right
            case '*': return left * right
            case '/': return left / right
            case '^': return Math.pow(left, right)
            case '<': return left < right
            case '>': return left > right
            case '<=': return left <= right
            case '>=': return left >= right
        }
        if (typeof left === 'boolean' && typeof right === 'boolean') switch (binop.op) {
            case 'and': case '&': return left && right
            case 'or': case '|': return left || right
        }
        switch (binop.op) {
            case 'is': case '==': return left === right
            case 'not': case '!=': return left !== right
            case 'istype': if (right instanceof Type) {
                const leftType = binop.left.expectType().getResolved()
                return leftType.isEqual(right)
            }
        }
        return null
    }

    visitBlock (block: Block): Value {
        const values = block.exprs.map(expr => expr.accept(this))
        return values.pop() || null
    }

    visitBranch (branch: Branch): Value {
        const condition = branch.condition.accept(this)
        if (condition === true) return branch.body.accept(this)
        if (branch.else) return branch.else.accept(this)
        else return incomplete()
    }

    visitCall (call: Call): Value {
        const fun = call.fun.accept(this)
        if (fun instanceof Fun) {
            fun.params.map((param, i) => {
                param.ident.expectSymbol().value = call.args[i]
            })
            if (fun.hasSelfParam && call.fun instanceof Lookup) {
                fun.scope.annotate_ty(
                    new Ident({ scope: call.fun.scope, name: 'self', file: fun.file, source: fun.source }),
                    { value: call.fun.environment }
                )
            }
            return fun.body.accept(this)
        }
        throw RuntimeError.unreachable(
            call.fun,
            'not a function (should have been caught earlier)'
        )
    }

    visitDecl (decl: Decl): Value {
        decl.ident.accept(this)
        decl.value.accept(this)
        decl.type.accept(this)
        return null
    }

    visitEnvironment (env: Environment): Value {
        return env
    }

    visitFun (fun: Fun): Value {
        return fun
    }

    visitIdent (ident: Ident): Value {
        const value = ident.expectSymbol()
        if (value) {
            if (value.value instanceof Expr) return value.value.accept(this)
            return value.value as Value
        }
        throw RuntimeError.unreachable(
            ident,
            'undefined ident (should have been caught earlier)'
        )
    }

    visitImpl (impl: Impl): Value {
        return null
    }

    visitLiteral (literal: Literal): Value {
        if (Array.isArray(literal.value)) return parseFloat(literal.value.join('.'))
        // Trim off quotes
        if (typeof literal.value === 'string') return literal.value.substring(1, literal.value.length - 1)
        return literal.value
    }

    visitLookup (lookup: Lookup): Value {
        return lookup.member.accept(this)
    }

    visitMatch (match: Match): Value {
        const expr = match.expr.accept(this)
        for (const branch of match.branches) {
            const condition = branch.condition.accept(this)
            if (condition instanceof Type && expr instanceof Type) return condition.isEqual(expr)
            if (condition instanceof Fun && expr instanceof Fun) return condition.isEqual(expr)
            if (condition instanceof Environment && expr instanceof Environment) return condition.isEqual(expr)

            if (condition === expr) return branch.body.accept(this)
            if (condition instanceof Type) return condition.isEqual(match.expr.expectType().getResolved())
        }
        return match.default.accept(this)
    }

    visitTypeConstructor (constructor: TypeConstructor): Value {
        return constructor.body.getResolved()
    }

    visitUnOp (unop: UnOp): Value {
        const expr = unop.expr.accept(this)
        switch (typeof expr) {
            case 'number': switch (unop.op) {
                case '-': return -1 * expr
            }
            case 'boolean': switch (unop.op) {
                case 'not': case '!': return !expr
            }
        }
        throw RuntimeError.unreachable(unop)
    }

    /* Types */

    visitFieldType (field: FieldType): Value {
        return field.getResolved()
    }

    visitFunType (fun: FunType): Value {
        return fun.getResolved()
    }

    visitInstanceType (instance: InstanceType): Value {
        return instance.getResolved()
    }

    visitPrimitiveType (primitive: PrimitiveType): Value {
        return primitive.getResolved()
    }

    visitStructType (struct: StructType): Value {
        return struct.getResolved()
    }

    visitUnknownType (unknown: UnknownType): Value {
        return unknown.getResolved()
    }
}

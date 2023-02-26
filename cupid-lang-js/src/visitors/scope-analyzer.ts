import { Assign, BinOp, Block, Call, Decl, ExprVisitor, FieldType, Fun, FunType, Ident, InstanceType, Literal, PrimitiveType, StructType, TypeConstructor, UnknownType } from '@/ast'
import { Context } from '@/env'

export default class ScopeAnalyzer extends ExprVisitor<void> {

    visitAssign (assign: Assign): void {
        assign.ident.scope = assign.scope
        assign.ident.accept(this)

        assign.value.scope = assign.scope
        assign.value.accept(this)
    }

    visitBinOp (binop: BinOp): void {
        binop.left.scope = binop.scope
        binop.left.accept(this)

        binop.right.scope = binop.scope
        binop.right.accept(this)
    }

    visitBlock (block: Block): void {
        const scope = block.scope.subscope()
        block.scope = scope
        block.exprs.map(expr => {
            expr.scope = scope
            expr.accept(this)
        })
    }

    visitCall (call: Call): void {
        const scope = call.scope.subscope(Context.Call)
        call.scope = scope

        call.fun.scope = scope
        call.fun.accept(this)

        call.args.map(arg => {
            arg.scope = scope
            arg.accept(this)
        })
    }

    visitDecl (decl: Decl): void {
        decl.ident.scope = decl.scope
        decl.ident.accept(this)

        decl.value.scope = decl.scope
        decl.value.accept(this)

        decl.type.scope = decl.scope
        decl.type.accept(this)
    }

    visitIdent (ident: Ident): void { }

    visitFun (fun: Fun): void {
        const scope = fun.scope.subscope(Context.Fun)
        fun.scope = scope
        fun.params.map(param => {
            param.scope = scope
            param.accept(this)
        })

        fun.body.scope = scope
        fun.body.accept(this)

        fun.returns.scope = scope
        fun.returns.accept(this)
    }

    visitLiteral (literal: Literal): void { }

    visitTypeConstructor (constructor: TypeConstructor): void {
        constructor.ident.scope = constructor.scope
        constructor.ident.accept(this)

        const scope = constructor.scope.subscope(Context.TypeConstructor)

        constructor.body.scope = scope
        constructor.body.accept(this)

        constructor.params.map(param => {
            param.scope = scope
            param.accept(this)
        })
    }

    /* Types */

    visitFieldType (field: FieldType): void {
        field.ident.scope = field.scope
        field.ident.accept(this)

        field.type.scope = field.scope
        field.type.accept(this)
    }

    visitFunType (fun: FunType): void {
        fun.params.map(param => {
            param.scope = fun.scope
            param.accept(this)
        })
        fun.returns.scope = fun.scope
        fun.returns.accept(this)
    }

    visitInstanceType (instance: InstanceType): void {
        const scope = instance.scope.subscope(Context.TypeConstructor)

        instance.scope = scope
        instance.ident.scope = scope
        instance.ident.accept(this)

        instance.args.map(arg => {
            arg.scope = scope
            arg.accept(this)
        })
    }

    visitPrimitiveType (primitive: PrimitiveType): void { }

    visitStructType (struct: StructType): void {
        const scope = struct.scope.subscope()
        struct.scope = scope
        struct.fields.map(field => {
            field.scope = scope
            field.accept(this)
        })
    }

    visitUnknownType (unknown: UnknownType): void { }
}
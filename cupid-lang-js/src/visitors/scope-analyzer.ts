import { Assign, BinOp, Block, Call, Decl, Environment, ExprVisitor, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, PrimitiveType, StructType, TypeConstructor, UnknownType } from '@/ast'
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

    visitEnvironment (env: Environment): void {
        const scope = env.scope.subscope(Context.Environment)
        if (env.ident) env.ident.accept(this)
        env.scope = scope
        env.content.map(expr => {
            expr.scope = scope
            expr.accept(this)
        })
    }

    visitIdent (ident: Ident): void { }

    visitImpl (impl: Impl): void {
        impl.type.scope = impl.scope
        impl.type.accept(this)

        impl.environment.scope = impl.scope
        impl.environment.accept(this)
    }

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

    visitLookup (lookup: Lookup): void {
        lookup.environment.scope = lookup.scope
        lookup.environment.accept(this)

        lookup.member.scope = lookup.environment.scope
        lookup.member.accept(this)
    }

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

        field.environment.scope = field.scope
        field.environment.accept(this)
    }

    visitFunType (fun: FunType): void {
        fun.params.map(param => {
            param.scope = fun.scope
            param.accept(this)
        })
        fun.returns.scope = fun.scope
        fun.returns.accept(this)

        fun.environment.scope = fun.scope
        fun.environment.accept(this)
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

        instance.environment.scope = scope
        instance.environment.accept(this)
    }

    visitPrimitiveType (primitive: PrimitiveType): void {
        primitive.environment.scope = primitive.scope
        primitive.environment.accept(this)
    }

    visitStructType (struct: StructType): void {
        const scope = struct.scope.subscope()
        struct.scope = scope
        struct.fields.map(field => {
            field.scope = scope
            field.accept(this)
        })

        struct.environment.scope = scope
        struct.environment.accept(this)
    }

    visitUnknownType (unknown: UnknownType): void {
        unknown.environment.scope = unknown.scope
        unknown.environment.accept(this)
    }
}
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, ExprVisitorWithContext, Fun, Call, Environment, Lookup, Impl } from '@/ast'

/**
 * Recursively performs the default action for all expressions
 */
export default class BaseExprVisitor extends ExprVisitor<void> {

    visitAssign (assign: Assign): void {
        assign.ident.accept(this)
        assign.value.accept(this)
    }

    visitBinOp (binop: BinOp): void {
        binop.left.accept(this)
        binop.right.accept(this)
    }

    visitBlock (block: Block): void {
        block.exprs.map(expr => expr.accept(this))
    }

    visitCall (call: Call): void {
        call.fun.accept(this)
        call.args.map(arg => arg.accept(this))
    }

    visitDecl (decl: Decl): void {
        decl.ident.accept(this)
        decl.type.accept(this)
        decl.value.accept(this)
    }

    visitEnvironment (env: Environment): void {
        env.ident?.accept(this)
        env.content.map(expr => expr.accept(this))
    }

    visitFun (fun: Fun): void {
        fun.params.map(param => param.accept(this))
        fun.body.accept(this)
        fun.returns.accept(this)
    }

    visitIdent (ident: Ident): void { }

    visitImpl (impl: Impl): void {
        impl.type.accept(this)
        impl.environment.accept(this)
    }

    visitLiteral (literal: Literal): void { }

    visitLookup (lookup: Lookup): void {
        lookup.environment.accept(this)
        lookup.member.accept(this)
    }

    visitTypeConstructor (constructor: TypeConstructor): void {
        constructor.ident.accept(this)
        constructor.params.map(param => param.accept(this))
        constructor.body.accept(this)
    }

    /* Types */

    visitFieldType (field: FieldType): void {
        field.ident.accept(this)
        field.type.accept(this)
        field.environment.accept(this)
    }

    visitFunType (fun: FunType): void {
        fun.params.map(param => param.accept(this))
        fun.returns.accept(this)
        fun.environment.accept(this)
    }

    visitInstanceType (instance: InstanceType): void {
        instance.ident.accept(this)
        instance.args.map(arg => arg.accept(this))
        instance.environment.accept(this)
    }

    visitPrimitiveType (primitive: PrimitiveType): void {
        primitive.environment.accept(this)
    }

    visitStructType (struct: StructType): void {
        struct.fields.map(field => field.accept(this))
        struct.environment.accept(this)
    }

    visitUnknownType (unknown: UnknownType): void {
        unknown.environment.accept(this)
    }

}

export class BaseExprVisitorWithContext<Ctx> extends ExprVisitorWithContext<void, Ctx> {

    visitAssign (assign: Assign, context: Ctx): void {
        assign.ident.acceptWithContext(this, context)
        assign.value.acceptWithContext(this, context)
    }

    visitBinOp (binop: BinOp, context: Ctx): void {
        binop.left.acceptWithContext(this, context)
        binop.right.acceptWithContext(this, context)
    }

    visitBlock (block: Block, context: Ctx): void {
        block.exprs.map(expr => expr.acceptWithContext(this, context))
    }

    visitCall (call: Call, context: Ctx): void {
        call.fun.acceptWithContext(this, context)
        call.args.map(arg => arg.acceptWithContext(this, context))
    }

    visitDecl (decl: Decl, context: Ctx): void {
        decl.ident.acceptWithContext(this, context)
        decl.type.acceptWithContext(this, context)
        decl.value.acceptWithContext(this, context)
    }

    visitEnvironment (env: Environment, context: Ctx): void {
        env.ident?.acceptWithContext(this, context)
        env.content.map(expr => expr.acceptWithContext(this, context))
    }

    visitIdent (ident: Ident, context: Ctx): void { }

    visitImpl (impl: Impl, context: Ctx): void {
        impl.type.acceptWithContext(this, context)
        impl.environment.acceptWithContext(this, context)
    }

    visitFun (fun: Fun, context: Ctx): void {
        fun.params.map(param => param.acceptWithContext(this, context))
        fun.body.acceptWithContext(this, context)
        fun.returns.acceptWithContext(this, context)
    }

    visitLiteral (literal: Literal, context: Ctx): void { }

    visitLookup (lookup: Lookup, context: Ctx): void {
        lookup.environment.acceptWithContext(this, context)
        lookup.member.acceptWithContext(this, context)
    }

    visitTypeConstructor (constructor: TypeConstructor, context: Ctx): void {
        constructor.ident.acceptWithContext(this, context)
        constructor.params.map(param => param.acceptWithContext(this, context))
        constructor.body.acceptWithContext(this, context)
    }

    /* Types */

    visitFieldType (field: FieldType, context: Ctx): void {
        field.ident.acceptWithContext(this, context)
        field.type.acceptWithContext(this, context)
        field.environment.acceptWithContext(this, context)
    }

    visitFunType (fun: FunType, context: Ctx): void {
        fun.params.map(param => param.acceptWithContext(this, context))
        fun.returns.acceptWithContext(this, context)
        fun.environment.acceptWithContext(this, context)
    }

    visitInstanceType (instance: InstanceType, context: Ctx): void {
        instance.ident.acceptWithContext(this, context)
        instance.args.map(arg => arg.acceptWithContext(this, context))
        instance.environment.acceptWithContext(this, context)
    }

    visitPrimitiveType (primitive: PrimitiveType, context: Ctx): void {
        primitive.environment.acceptWithContext(this, context)
    }

    visitStructType (struct: StructType, context: Ctx): void {
        struct.fields.map(field => field.acceptWithContext(this, context))
        struct.environment.acceptWithContext(this, context)
    }

    visitUnknownType (unknown: UnknownType, context: Ctx): void {
        unknown.environment.acceptWithContext(this, context)
    }

}
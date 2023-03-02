import { Assign, BinOp, Block, Branch, Call, Decl, Environment, ExprVisitor, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, Match, PrimitiveType, StructType, TypeConstructor, UnknownType, UnOp } from '@/ast'
import { Context } from '@/env'
import BaseExprVisitor from './base'

export default class ScopeAnalyzer extends BaseExprVisitor {

    visitAssign (assign: Assign): void {
        assign.ident.scope = assign.scope
        assign.value.scope = assign.scope
        super.visitAssign(assign)
    }

    visitBinOp (binop: BinOp): void {
        binop.left.scope = binop.scope
        binop.right.scope = binop.scope
        super.visitBinOp(binop)
    }

    visitBlock (block: Block): void {
        const scope = block.scope.subscope()
        block.scope = scope
        block.exprs.map(expr => expr.scope = scope)
        super.visitBlock(block)
    }

    visitBranch (branch: Branch): void {
        branch.condition.scope = branch.scope
        branch.body.scope = branch.scope
        if (branch.else) branch.else.scope = branch.scope
        super.visitBranch(branch)
    }

    visitCall (call: Call): void {
        const scope = call.scope.subscope(Context.Call)
        call.scope = scope
        call.fun.scope = scope
        call.args.map(arg => arg.scope = scope)
        super.visitCall(call)
    }

    visitDecl (decl: Decl): void {
        decl.ident.scope = decl.scope
        decl.value.scope = decl.scope
        decl.type.scope = decl.scope
        super.visitDecl(decl)
    }

    visitEnvironment (env: Environment): void {
        const scope = env.scope.subscope(Context.Environment)
        if (env.ident) env.ident.scope = env.scope
        env.scope = scope
        env.content.map(expr => expr.scope = scope)
        super.visitEnvironment(env)
    }

    visitIdent (ident: Ident): void {
        super.visitIdent(ident)
    }

    visitImpl (impl: Impl): void {
        impl.type.scope = impl.scope
        impl.environment.scope = impl.scope
        super.visitImpl(impl)
    }

    visitFun (fun: Fun): void {
        const scope = fun.scope.subscope(Context.Fun)
        fun.scope = scope
        fun.params.map(param => param.scope = scope)
        fun.body.scope = scope
        fun.returns.scope = scope
        super.visitFun(fun)
    }

    visitLiteral (literal: Literal): void {
        super.visitLiteral(literal)
    }

    visitLookup (lookup: Lookup): void {
        lookup.environment.scope = lookup.scope
        lookup.member.scope = lookup.environment.scope
        super.visitLookup(lookup)
    }

    visitMatch (match: Match): void {
        match.expr.scope = match.scope
        match.branches.map(branch => branch.scope = match.scope)
        match.default.scope = match.scope
        super.visitMatch(match)
    }

    visitTypeConstructor (constructor: TypeConstructor): void {
        constructor.ident.scope = constructor.scope
        const scope = constructor.scope.subscope(Context.TypeConstructor)
        constructor.body.scope = scope
        constructor.params.map(param => param.scope = scope)
        super.visitTypeConstructor(constructor)
    }

    visitUnOp (unop: UnOp): void {
        unop.expr.scope = unop.scope
        super.visitUnOp(unop)
    }

    /* Types */

    visitFieldType (field: FieldType): void {
        field.ident.scope = field.scope
        field.type.scope = field.scope
        field.environment.scope = field.scope
        super.visitFieldType(field)
    }

    visitFunType (fun: FunType): void {
        fun.params.map(param => param.scope = fun.scope)
        fun.returns.scope = fun.scope
        fun.environment.scope = fun.scope
        super.visitFunType(fun)
    }

    visitInstanceType (instance: InstanceType): void {
        const scope = instance.scope.subscope(Context.TypeConstructor)
        instance.scope = scope
        instance.ident.scope = scope
        instance.args.map(arg => arg.scope = scope)
        instance.environment.scope = scope
        if (instance.value) instance.value.scope = scope
        super.visitInstanceType(instance)
    }

    visitPrimitiveType (primitive: PrimitiveType): void {
        primitive.environment.scope = primitive.scope
        super.visitPrimitiveType(primitive)
    }

    visitStructType (struct: StructType): void {
        const scope = struct.scope.subscope()
        struct.scope = scope
        struct.fields.map(field => field.scope = scope)
        struct.environment.scope = scope
        super.visitStructType(struct)
    }

    visitUnknownType (unknown: UnknownType): void {
        unknown.environment.scope = unknown.scope
        super.visitUnknownType(unknown)
    }
}
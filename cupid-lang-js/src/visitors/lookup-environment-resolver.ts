import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Lookup, Environment, Impl, ExprVisitorWithContext, Call, Fun, UnOp, Branch, Match } from '@/ast'
import { Scope } from '@/env'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor, { BaseExprVisitorWithContext } from './base'

/**
 * Gathers all possible environments that a lookup may be referencing
 * This can be the type (ex. decimal::sin), struct fields (ex person::name), etc
 */
export default class LookupEnvironmentResolver extends BaseExprVisitorWithContext<LookupEnvironmentFinder> {

    visitAssign (assign: Assign, context: LookupEnvironmentFinder): void {
        super.visitAssign(assign, context)
        assign.lookupEnvironments = context.visit(assign)
    }

    visitBinOp (binop: BinOp, context: LookupEnvironmentFinder): void {
        super.visitBinOp(binop, context)
        binop.lookupEnvironments = context.visit(binop)
    }

    visitBlock (block: Block, context: LookupEnvironmentFinder): void {
        super.visitBlock(block, context)
        block.lookupEnvironments = context.visit(block)
    }

    visitBranch (branch: Branch, context: LookupEnvironmentFinder): void {
        super.visitBranch(branch, context)
        branch.lookupEnvironments = context.visit(branch)
    }

    visitCall (call: Call, context: LookupEnvironmentFinder): void {
        super.visitCall(call, context)
        call.lookupEnvironments = context.visit(call)
    }

    visitDecl (decl: Decl, context: LookupEnvironmentFinder): void {
        super.visitDecl(decl, context)
        decl.lookupEnvironments = context.visit(decl)
    }

    visitEnvironment (env: Environment, context: LookupEnvironmentFinder): void {
        super.visitEnvironment(env, context)
        env.lookupEnvironments = context.visit(env)
    }

    visitFun (fun: Fun, context: LookupEnvironmentFinder): void {
        super.visitFun(fun, context)
        fun.lookupEnvironments = context.visit(fun)
    }

    visitIdent (ident: Ident, context: LookupEnvironmentFinder): void {
        super.visitIdent(ident, context)
        ident.lookupEnvironments = context.visit(ident)
    }

    visitImpl (impl: Impl, context: LookupEnvironmentFinder): void {
        super.visitImpl(impl, context)
        impl.lookupEnvironments = context.visit(impl)
    }

    visitLiteral (literal: Literal, context: LookupEnvironmentFinder): void {
        super.visitLiteral(literal, context)
        literal.lookupEnvironments = context.visit(literal)
    }

    visitLookup (lookup: Lookup, context: LookupEnvironmentFinder): void {
        lookup.environment.acceptWithContext(this, context)
        lookup.lookupEnvironments = context.visit(lookup)
        lookup.member.lookupEnvironments = lookup.lookupEnvironments
    }

    visitMatch (match: Match, context: LookupEnvironmentFinder): void {
        super.visitMatch(match, context)
        match.lookupEnvironments = context.visit(match)
    }

    visitTypeConstructor (constructor: TypeConstructor, context: LookupEnvironmentFinder): void {
        super.visitTypeConstructor(constructor, context)
        constructor.lookupEnvironments = context.visit(constructor)
    }

    visitUnOp (unop: UnOp, context: LookupEnvironmentFinder): void {
        super.visitUnOp(unop, context)
        unop.lookupEnvironments = context.visit(unop)
    }

    /* Types */

    visitFieldType (field: FieldType, context: LookupEnvironmentFinder): void {
        super.visitFieldType(field, context)
        field.lookupEnvironments = context.visit(field)
    }

    visitFunType (fun: FunType, context: LookupEnvironmentFinder): void {
        super.visitFunType(fun, context)
        fun.lookupEnvironments = context.visit(fun)
    }

    visitInstanceType (instance: InstanceType, context: LookupEnvironmentFinder): void {
        super.visitInstanceType(instance, context)
        instance.lookupEnvironments = context.visit(instance)
    }

    visitPrimitiveType (primitive: PrimitiveType, context: LookupEnvironmentFinder): void {
        super.visitPrimitiveType(primitive, context)
        primitive.lookupEnvironments = context.visit(primitive)
    }

    visitStructType (struct: StructType, context: LookupEnvironmentFinder): void {
        super.visitStructType(struct, context)
        struct.lookupEnvironments = context.visit(struct)
    }

    visitUnknownType (unknown: UnknownType, context: LookupEnvironmentFinder): void {
        super.visitUnknownType(unknown, context)
        unknown.lookupEnvironments = context.visit(unknown)
    }
}

export class LookupEnvironmentFinder extends ExprVisitor<Scope[]> {

    visitAssign (assign: Assign): Scope[] {
        return []
    }

    visitBinOp (binop: BinOp): Scope[] {
        return binop.expectType().accept(this)
    }

    visitBlock (block: Block): Scope[] {
        return block.expectType().accept(this)
    }

    visitBranch (branch: Branch): Scope[] {
        return [
            ...branch.body.expectType().accept(this),
            ...branch.else ? branch.else.expectType().accept(this) : [],
        ]
    }

    visitCall (call: Call): Scope[] {
        return call.expectType().accept(this)
    }

    visitDecl (decl: Decl): Scope[] {
        return []
    }

    visitEnvironment (env: Environment): Scope[] {
        return [env.scope]
    }

    visitFun (fun: Fun): Scope[] {
        return fun.expectType().accept(this)
    }

    visitIdent (ident: Ident): Scope[] {
        if (ident.symbol) return [
            ...ident.expectType().accept(this),
            ...ident.symbol.type?.accept(this) || [],
            ...ident.symbol.value?.accept(this) || []
        ]
        return ident.expectType().accept(this)
    }

    visitImpl (impl: Impl): Scope[] {
        return [
            ...impl.type.accept(this),
            ...impl.environment.accept(this)
        ]
    }

    visitLiteral (literal: Literal): Scope[] {
        return literal.expectType().accept(this)
    }

    visitLookup (lookup: Lookup): Scope[] {
        return lookup.environment.accept(this)
    }

    visitMatch (match: Match): Scope[] {
        return match.expectType().accept(this)
    }

    visitTypeConstructor (constructor: TypeConstructor): Scope[] {
        return constructor.body.accept(this)
    }

    visitUnOp (unop: UnOp): Scope[] {
        return unop.expectType().accept(this)
    }

    /* Types */

    visitFieldType (field: FieldType): Scope[] {
        return [
            ...field.environment.accept(this),
            ...field.type.accept(this),
        ]
    }

    visitFunType (fun: FunType): Scope[] {
        return fun.environment.accept(this)
    }

    visitInstanceType (instance: InstanceType): Scope[] {
        return [
            ...instance.getResolved().environment.accept(this),
            ...instance.environment.accept(this)
        ]
    }

    visitPrimitiveType (primitive: PrimitiveType): Scope[] {
        return primitive.environment.accept(this)
    }

    visitStructType (struct: StructType): Scope[] {
        return [
            struct.scope,
            ...struct.getResolved().environment.accept(this),
        ]
    }

    visitUnknownType (unknown: UnknownType): Scope[] {
        return []
    }
}
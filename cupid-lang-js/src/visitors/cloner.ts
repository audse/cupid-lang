import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, Fun, Call, Environment, Lookup, Impl, UnOp, Branch, Match } from '@/ast'
import { ExprVisitorWithContext } from '@/ast/visitor'
import { Context, Scope } from '@/env'

export default class Cloner extends ExprVisitorWithContext<Expr, Scope> {

    visitAssign (assign: Assign, scope: Scope): Assign {
        return new Assign({
            scope,
            source: assign.source,
            ident: this.visitIdent(assign.ident, scope),
            value: assign.value.acceptWithContext(this, scope),
            inferredType: assign.inferredType,
        })
    }

    visitBinOp (binop: BinOp, scope: Scope): BinOp {
        return new BinOp({
            scope,
            source: binop.source,
            left: binop.left.acceptWithContext(this, scope),
            right: binop.right.acceptWithContext(this, scope),
            op: binop.op,
            inferredType: binop.inferredType,
        })
    }

    visitBlock (block: Block, scope: Scope): Block {
        const subscope = scope.subscope(Context.Block)
        return new Block({
            scope: subscope,
            source: block.source,
            inferredType: block.inferredType,
            exprs: block.exprs.map(expr => expr.acceptWithContext(this, subscope)),
        })
    }

    visitBranch (branch: Branch, scope: Scope): Branch {
        return new Branch({
            scope,
            source: branch.source,
            inferredType: branch.inferredType,
            condition: branch.condition.acceptWithContext(this, scope),
            body: branch.body.acceptWithContext(this, scope),
            ...branch.else && { else: branch.else.acceptWithContext(this, scope) }
        })
    }

    visitCall (call: Call, scope: Scope): Call {
        const subscope = scope.subscope(Context.Call)
        return new Call({
            scope: subscope,
            source: call.source,
            inferredType: call.inferredType,
            fun: call.fun.acceptWithContext(this, subscope),
            args: call.args.map(arg => arg.acceptWithContext(this, subscope)),
        })
    }

    visitDecl (decl: Decl, scope: Scope): Decl {
        return new Decl({
            scope,
            source: decl.source,
            mutable: decl.mutable,
            inferredType: decl.inferredType,
            ident: this.visitIdent(decl.ident, scope),
            value: decl.value.acceptWithContext(this, scope),
            type: this.visitType(decl.type, scope),
        })
    }

    visitEnvironment (env: Environment, scope: Scope): Environment {
        const subscope = scope.subscope(Context.Environment)
        return new Environment({
            scope: subscope,
            source: env.source,
            inferredType: env.inferredType,
            content: env.content.map(expr => expr.acceptWithContext(this, subscope)),
            ...env.ident && { ident: this.visitIdent(env.ident, subscope) },
        })
    }

    visitFun (fun: Fun, scope: Scope): Fun {
        const subscope = scope.subscope(Context.Fun)
        return new Fun({
            scope: subscope,
            source: fun.source,
            inferredType: fun.inferredType,
            params: fun.params.map(param => this.visitFieldType(param, subscope)),
            body: fun.body.acceptWithContext(this, subscope),
            returns: this.visitType(fun.returns, subscope)
        })
    }

    visitIdent (ident: Ident, scope: Scope): Ident {
        return new Ident({
            scope,
            source: ident.source,
            name: ident.name,
            inferredType: ident.inferredType,
        })
    }

    visitImpl (impl: Impl, scope: Scope): Impl {
        return new Impl({
            scope,
            source: impl.source,
            inferredType: impl.inferredType,
            type: this.visitType(impl.type, scope),
            environment: this.visitEnvironment(impl.environment, scope),
        })
    }

    visitLiteral (literal: Literal, scope: Scope): Literal {
        return new Literal({
            scope,
            source: literal.source,
            inferredType: literal.inferredType,
            value: Array.isArray(literal.value) ? [...literal.value] : literal.value,
        })
    }

    visitLookup (lookup: Lookup, scope: Scope): Lookup {
        return new Lookup({
            scope,
            source: lookup.source,
            inferredType: lookup.inferredType,
            environment: lookup.environment.acceptWithContext(this, scope),
            member: (() => {
                if (lookup.member instanceof Ident) return this.visitIdent(lookup.member, scope)
                else return this.visitLiteral(lookup.member, scope)
            })(),
        })
    }

    visitMatch (match: Match, scope: Scope): Match {
        return new Match({
            scope,
            expr: match.expr.acceptWithContext(this, scope),
            branches: match.branches.map(branch => this.visitBranch(branch, scope)),
            default: match.default.acceptWithContext(this, scope)
        })
    }

    visitTypeConstructor (constructor: TypeConstructor, scope: Scope): TypeConstructor {
        const subscope = scope.subscope(Context.TypeConstructor)
        return new TypeConstructor({
            scope: subscope,
            source: constructor.source,
            inferredType: constructor.inferredType,
            ident: this.visitIdent(constructor.ident, subscope),
            params: constructor.params.map(param => this.visitIdent(param, subscope)),
            body: this.visitType(constructor.body, subscope),
        })
    }

    visitUnOp (unop: UnOp, scope: Scope): Expr {
        return new UnOp({
            scope,
            source: unop.source,
            expr: unop.acceptWithContext(this, scope),
            op: unop.op
        })
    }

    /* Types */

    visitType (type: Type, scope: Scope): Type {
        return type.acceptWithContext(this, scope) as Type
    }

    visitFieldType (field: FieldType, scope: Scope): FieldType {
        return new FieldType({
            scope,
            source: field.source,
            environment: field.environment,
            inferredType: field.inferredType,
            ident: this.visitIdent(field.ident, scope),
            type: this.visitType(field.type, scope),
        })
    }

    visitFunType (fun: FunType, scope: Scope): FunType {
        return new FunType({
            scope,
            source: fun.source,
            environment: fun.environment,
            inferredType: fun.inferredType,
            params: fun.params.map(param => this.visitFieldType(param, scope)),
            returns: this.visitType(fun.returns, scope),
        })
    }

    visitInstanceType (instance: InstanceType, scope: Scope): InstanceType {
        const subscope = scope.subscope(Context.TypeConstructor)
        return new InstanceType({
            ...instance,
            scope: subscope,
            ident: this.visitIdent(instance.ident, subscope),
            args: instance.args.map(arg => this.visitType(arg, subscope)),
        })
    }

    visitPrimitiveType (primitive: PrimitiveType, scope: Scope): PrimitiveType {
        return new PrimitiveType({
            ...primitive,
            scope,
        })
    }

    visitStructType (struct: StructType, scope: Scope): StructType {
        const subscope = scope.subscope(Context.TypeConstructor)
        return new StructType({
            ...struct,
            scope: subscope,
            fields: struct.fields.map(field => this.visitFieldType(field, subscope)),
        })
    }

    visitUnknownType (unknown: UnknownType, scope: Scope): UnknownType {
        return new UnknownType({
            ...unknown,
            scope,
        })
    }

}
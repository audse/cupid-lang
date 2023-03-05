import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, Fun, Call, Environment, Lookup, Impl, UnOp, Branch, Match } from '@/ast'
import { ExprVisitorWithContext } from '@/ast/visitor'
import { Context, Scope } from '@/env'

export default class Cloner extends ExprVisitorWithContext<Expr, Scope> {

    visitAssign (assign: Assign, scope: Scope): Assign {
        const { file, source, inferredType } = assign
        return new Assign({
            scope, file, source, inferredType,
            ident: this.visitIdent(assign.ident, scope),
            value: assign.value.acceptWithContext(this, scope),
        })
    }

    visitBinOp (binop: BinOp, scope: Scope): BinOp {
        const { file, source, op, inferredType } = binop
        return new BinOp({
            scope, file, source, op, inferredType,
            left: binop.left.acceptWithContext(this, scope),
            right: binop.right.acceptWithContext(this, scope),
        })
    }

    visitBlock (block: Block, scope: Scope): Block {
        const { file, source, inferredType } = block
        const subscope = scope.subscope(Context.Block)
        return new Block({
            file, source, inferredType,
            scope: subscope,
            exprs: block.exprs.map(expr => expr.acceptWithContext(this, subscope)),
        })
    }

    visitBranch (branch: Branch, scope: Scope): Branch {
        const { file, source, inferredType } = branch
        return new Branch({
            scope, file, source, inferredType,
            condition: branch.condition.acceptWithContext(this, scope),
            body: branch.body.acceptWithContext(this, scope),
            ...branch.else && { else: branch.else.acceptWithContext(this, scope) }
        })
    }

    visitCall (call: Call, scope: Scope): Call {
        const { file, source, inferredType } = call
        const subscope = scope.subscope(Context.Call)
        return new Call({
            file, source, inferredType,
            scope: subscope,
            fun: call.fun.acceptWithContext(this, subscope),
            args: call.args.map(arg => arg.acceptWithContext(this, subscope)),
        })
    }

    visitDecl (decl: Decl, scope: Scope): Decl {
        const { file, source, inferredType, mutable } = decl
        return new Decl({
            scope, file, source, inferredType, mutable,
            ident: this.visitIdent(decl.ident, scope),
            value: decl.value.acceptWithContext(this, scope),
            type: this.visitType(decl.type, scope),
        })
    }

    visitEnvironment (env: Environment, scope: Scope): Environment {
        const { file, source, inferredType } = env
        const subscope = scope.global().subscope(Context.Environment)
        return new Environment({
            file, source, inferredType,
            scope: subscope,
            content: env.content.map(expr => expr.acceptWithContext(this, subscope)),
            ...env.ident && { ident: this.visitIdent(env.ident, subscope) },
        })
    }

    visitFun (fun: Fun, scope: Scope): Fun {
        const { file, source, inferredType } = fun
        const subscope = scope.subscope(Context.Fun)
        return new Fun({
            file, source, inferredType,
            scope: subscope,
            params: fun.params.map(param => this.visitFieldType(param, subscope)),
            body: fun.body.acceptWithContext(this, subscope),
            returns: this.visitType(fun.returns, subscope)
        })
    }

    visitIdent (ident: Ident, scope: Scope): Ident {
        const { file, source, inferredType, name } = ident
        return new Ident({ scope, file, source, inferredType, name })
    }

    visitImpl (impl: Impl, scope: Scope): Impl {
        const { file, source, inferredType } = impl
        return new Impl({
            scope, file, source, inferredType,
            type: this.visitType(impl.type, scope),
            environment: this.visitEnvironment(impl.environment, scope),
        })
    }

    visitLiteral (literal: Literal, scope: Scope): Literal {
        const { file, source, inferredType } = literal
        return new Literal({
            scope, file, source, inferredType,
            value: Array.isArray(literal.value) ? [...literal.value] : literal.value,
        })
    }

    visitLookup (lookup: Lookup, scope: Scope): Lookup {
        const { file, source, inferredType } = lookup
        return new Lookup({
            scope, file, source, inferredType,
            environment: lookup.environment.acceptWithContext(this, scope),
            member: (() => {
                if (lookup.member instanceof Ident) return this.visitIdent(lookup.member, scope)
                else return this.visitLiteral(lookup.member, scope)
            })(),
        })
    }

    visitMatch (match: Match, scope: Scope): Match {
        const { file, source, inferredType } = match
        return new Match({
            scope, file, source, inferredType,
            expr: match.expr.acceptWithContext(this, scope),
            branches: match.branches.map(branch => this.visitBranch(branch, scope)),
            default: match.default.acceptWithContext(this, scope)
        })
    }

    visitTypeConstructor (constructor: TypeConstructor, scope: Scope): TypeConstructor {
        const { file, source, inferredType } = constructor
        const subscope = scope.subscope(Context.TypeConstructor)
        return new TypeConstructor({
            file, source, inferredType,
            scope: subscope,
            ident: this.visitIdent(constructor.ident, subscope),
            params: constructor.params.map(param => this.visitIdent(param, subscope)),
            body: this.visitType(constructor.body, subscope),
        })
    }

    visitUnOp (unop: UnOp, scope: Scope): Expr {
        const { file, source, inferredType, op } = unop
        return new UnOp({
            scope, file, source, inferredType, op,
            expr: unop.acceptWithContext(this, scope),
        })
    }

    /* Types */

    visitType (type: Type, scope: Scope): Type {
        return type.acceptWithContext(this, scope) as Type
    }

    visitFieldType (field: FieldType, scope: Scope): FieldType {
        const { file, source, inferredType, environment } = field
        return new FieldType({
            scope, file, source, inferredType, environment,
            ident: this.visitIdent(field.ident, scope),
            type: this.visitType(field.type, scope),
        })
    }

    visitFunType (fun: FunType, scope: Scope): FunType {
        const { file, source, inferredType, environment } = fun
        return new FunType({
            scope, file, source, inferredType, environment,
            params: fun.params.map(param => this.visitFieldType(param, scope)),
            returns: this.visitType(fun.returns, scope),
        })
    }

    visitInstanceType (instance: InstanceType, scope: Scope): InstanceType {
        const { file, source, inferredType, environment } = instance
        const subscope = scope.subscope(Context.TypeConstructor)
        return new InstanceType({
            file, source, inferredType, environment,
            scope: subscope,
            ident: this.visitIdent(instance.ident, subscope),
            args: instance.args.map(arg => this.visitType(arg, subscope)),
        })
    }

    visitPrimitiveType (primitive: PrimitiveType, scope: Scope): PrimitiveType {
        const { file, source, inferredType, environment, name } = primitive
        return new PrimitiveType({ scope, file, source, inferredType, environment, name })
    }

    visitStructType (struct: StructType, scope: Scope): StructType {
        const { file, source, inferredType, environment } = struct
        const subscope = scope.subscope(Context.TypeConstructor)
        return new StructType({
            file, source, inferredType, environment,
            scope: subscope,
            fields: struct.fields.map(field => this.visitFieldType(field, subscope)),
        })
    }

    visitUnknownType (unknown: UnknownType, scope: Scope): UnknownType {
        const { file, source, inferredType, environment } = unknown
        return new UnknownType({ scope, file, source, inferredType, environment })
    }

}
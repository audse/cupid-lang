import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, ExprVisitorWithContext, Fun, Call, Environment, Lookup, Impl } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import { BaseExprVisitorWithContext } from './base'
import { TypeUnifier } from './type-unifier'

export default class TypeInferrer extends BaseExprVisitorWithContext<Infer> {

    visitAssign (assign: Assign, inferrer: Infer): void {
        super.visitAssign(assign, inferrer)
        inferrer.visit(assign)
    }

    visitBinOp (binop: BinOp, inferrer: Infer): void {
        super.visitBinOp(binop, inferrer)
        inferrer.visit(binop)
    }

    visitBlock (block: Block, inferrer: Infer): void {
        super.visitBlock(block, inferrer)
        inferrer.visit(block)
    }

    visitCall (call: Call, inferrer: Infer): void {
        super.visitCall(call, inferrer)
        inferrer.visit(call)
    }

    visitDecl (decl: Decl, inferrer: Infer): void {
        super.visitDecl(decl, inferrer)
        inferrer.visit(decl)
    }

    visitEnvironment (env: Environment, inferrer: Infer): void {
        super.visitEnvironment(env, inferrer)
        inferrer.visit(env)
    }

    visitFun (fun: Fun, inferrer: Infer): void {
        super.visitFun(fun, inferrer)
        inferrer.visit(fun)
    }

    visitIdent (ident: Ident, inferrer: Infer): void {
        super.visitIdent(ident, inferrer)
        inferrer.visit(ident)
    }

    visitLiteral (literal: Literal, inferrer: Infer): void {
        super.visitLiteral(literal, inferrer)
        inferrer.visit(literal)
    }

    visitLookup (lookup: Lookup, inferrer: Infer): void {
        lookup.environment.acceptWithContext(this, inferrer)
        inferrer.visit(lookup)
    }

    visitTypeConstructor (constructor: TypeConstructor, inferrer: Infer): void {
        super.visitTypeConstructor(constructor, inferrer)
        inferrer.visit(constructor)
    }

    /* Types */

    visitFieldType (field: FieldType, inferrer: Infer): void {
        super.visitFieldType(field, inferrer)
        inferrer.visit(field)
    }

    visitFunType (fun: FunType, inferrer: Infer): void {
        super.visitFunType(fun, inferrer)
        inferrer.visit(fun)
    }

    visitInstanceType (instance: InstanceType, inferrer: Infer): void {
        super.visitInstanceType(instance, inferrer)
        inferrer.visit(instance)
    }

    visitPrimitiveType (primitive: PrimitiveType, inferrer: Infer): void {
        inferrer.visit(primitive)
    }

    visitStructType (struct: StructType, inferrer: Infer): void {
        super.visitStructType(struct, inferrer)
        inferrer.visit(struct)
    }

    visitUnknownType (unknown: UnknownType, inferrer: Infer): void {
        inferrer.visit(unknown)
    }

}

function primitiveType (parent: Expr, name: string): Type {
    const ident = new Ident({ source: parent.source, scope: parent.scope, name })
    const symbol = parent.scope.lookup(ident)
    if (symbol) {
        if (symbol.value instanceof Type) return symbol.value
        if (symbol.value instanceof TypeConstructor) return symbol.value.body
    }
    throw CompilationError.notDefined(ident)
}

function noneType (parent: Expr) {
    return primitiveType(parent, 'none')
}

function type (parent: Expr) {
    return primitiveType(parent, 'type')
}

function unknownType (parent: Expr) {
    return new UnknownType({
        source: parent.source,
        scope: parent.scope,
    })
}

export class Infer extends ExprVisitor<Type> {

    visit (expr: Expr): Type {
        const type = expr.accept(this)
        expr.inferredType = type
        return type
    }

    visitAssign (assign: Assign): Type {
        return noneType(assign)
    }

    visitBinOp (binop: BinOp): Type {
        if (['+', '-', '*', '/', '^', '%'].includes(binop.op)) {
            const left = binop.left.accept(this)
            const right = binop.right.accept(this)
            const unifier = new TypeUnifier()
            return unifier.visit(left, right)
        }
        return primitiveType(binop, 'bool')
    }

    visitBlock (block: Block): Type {
        const types = block.exprs.map(expr => expr.accept(this))
        return types.pop() || noneType(block)
    }

    visitCall (call: Call): Type {
        const fun = call.fun.accept(this)
        if (fun instanceof FunType) return fun.returns
        if (fun instanceof UnknownType) return unknownType(call)
        throw CompilationError.notAFunction(call.fun)
    }

    visitDecl (decl: Decl): Type {
        return noneType(decl)
    }

    visitEnvironment (env: Environment): Type {
        return primitiveType(env, 'env')
    }

    visitFun (fun: Fun): Type {
        return new FunType({
            scope: fun.scope,
            source: fun.source,
            params: fun.params,
            returns: fun.body.accept(this),
        })
    }

    visitIdent (ident: Ident): Type {
        const symbol = ident.expectSymbol()
        if (symbol.type && !(symbol.type instanceof UnknownType)) return symbol.type
        if (symbol.value) return symbol.value.accept(this)
        else return unknownType(ident)
    }

    visitImpl (impl: Impl): Type {
        return primitiveType(impl, 'none')
    }

    visitLiteral (literal: Literal): Type {
        switch (typeof literal.value) {
            case 'string': return primitiveType(literal, 'str')
            case 'number': return primitiveType(literal, 'int')
            case 'boolean': return primitiveType(literal, 'bool')
            case 'object': {
                if (Array.isArray(literal.value)) return primitiveType(literal, 'decimal')
                if (!literal.value) return noneType(literal)
            }
        }
        return unknownType(literal)
    }

    visitLookup (lookup: Lookup): Type {
        return unknownType(lookup)
    }

    visitTypeConstructor (constructor: TypeConstructor): Type {
        return type(constructor)
    }

    /* Types */

    visitFieldType (field: FieldType): Type {
        return type(field)
    }

    visitFunType (fun: FunType): Type {
        return type(fun)
    }

    visitInstanceType (instance: InstanceType): Type {
        return type(instance)
    }

    visitPrimitiveType (primitive: PrimitiveType): Type {
        return type(primitive)
    }

    visitStructType (struct: StructType): Type {
        return type(struct)
    }

    visitUnknownType (unknown: UnknownType): Type {
        return type(unknown)
    }

}
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, ExprVisitorWithContext, Fun, Call } from '@/ast'
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

    visitFun (fun: Fun, inferrer: Infer): void {
        super.visitFun(fun, inferrer)
        inferrer.visit(fun)
    }

    visitIdent (ident: Ident, inferrer: Infer): void {
        inferrer.visit(ident)
    }

    visitLiteral (literal: Literal, inferrer: Infer): void {
        inferrer.visit(literal)
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

function primitiveType (parent: Expr, name: string) {
    return new PrimitiveType({
        source: parent.source,
        scope: parent.scope,
        name
    })
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
        throw CompilationError.notAFunction(call.fun)
    }

    visitDecl (decl: Decl): Type {
        return noneType(decl)
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
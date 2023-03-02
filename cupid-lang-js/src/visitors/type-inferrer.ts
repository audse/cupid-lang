import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, ExprVisitorWithContext, Fun, Call, Environment, Lookup, Impl, UnOp, Branch, Match } from '@/ast'
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

    visitBranch (branch: Branch, inferrer: Infer): void {
        super.visitBranch(branch, inferrer)
        inferrer.visit(branch)
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

    visitImpl (impl: Impl, inferrer: Infer): void {
        super.visitImpl(impl, inferrer)
        inferrer.visit(impl)
    }

    visitLiteral (literal: Literal, inferrer: Infer): void {
        super.visitLiteral(literal, inferrer)
        inferrer.visit(literal)
    }

    visitLookup (lookup: Lookup, inferrer: Infer): void {
        lookup.environment.acceptWithContext(this, inferrer)
        inferrer.visit(lookup)
    }

    visitMatch (match: Match, inferrer: Infer): void {
        super.visitMatch(match, inferrer)
        inferrer.visit(match)
    }

    visitTypeConstructor (constructor: TypeConstructor, inferrer: Infer): void {
        super.visitTypeConstructor(constructor, inferrer)
        inferrer.visit(constructor)
    }

    visitUnOp (unop: UnOp, inferrer: Infer): void {
        super.visitUnOp(unop, inferrer)
        inferrer.visit(unop)
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

function infer<E extends Expr> (expr: E, doInfer: () => Type): Type {
    if (expr.inferredType && !(expr.inferredType instanceof UnknownType)) return expr.inferredType
    const inferredType = doInfer()
    expr.inferredType = inferredType
    return inferredType
}

export class Infer extends ExprVisitor<Type> {

    visitAssign (assign: Assign): Type {
        return infer(assign, () => noneType(assign))
    }

    visitBinOp (binop: BinOp): Type {
        return infer(binop, () => {
            if (['+', '-', '*', '/', '^', '%'].includes(binop.op)) {
                const left = binop.left.accept(this)
                const right = binop.right.accept(this)
                const unifier = new TypeUnifier()
                return unifier.visit(left, right)
            }
            return primitiveType(binop, 'bool')
        })
    }

    visitBlock (block: Block): Type {
        return infer(block, () => {
            const types = block.exprs.map(expr => expr.accept(this))
            return types.pop() || noneType(block)
        })
    }

    visitBranch (branch: Branch): Type {
        return infer(branch, () => {
            const bodyType = branch.body.accept(this)
            if (branch.else) {
                const elseType = branch.else.accept(this)
                return new TypeUnifier().visit(bodyType, elseType)
            }
            return bodyType
        })
    }

    visitCall (call: Call): Type {
        return infer(call, () => {
            const fun = call.fun.accept(this)
            if (fun instanceof FunType) return fun.returns
            if (fun instanceof UnknownType) return unknownType(call)
            throw CompilationError.notAFunction(call.fun)
        })
    }

    visitDecl (decl: Decl): Type {
        return infer(decl, () => noneType(decl))
    }

    visitEnvironment (env: Environment): Type {
        return infer(env, () => primitiveType(env, 'env'))
    }

    visitFun (fun: Fun): Type {
        return infer(fun, () => new FunType({
            scope: fun.scope,
            source: fun.source,
            params: fun.params,
            returns: fun.body.accept(this),
        }))
    }

    visitIdent (ident: Ident): Type {
        return infer(ident, () => {
            const symbol = ident.expectSymbol()
            if (symbol.type && !(symbol.type instanceof UnknownType)) return symbol.type
            if (symbol.value) return symbol.value.accept(this)
            else return unknownType(ident)
        })
    }

    visitImpl (impl: Impl): Type {
        return infer(impl, () => primitiveType(impl, 'none'))
    }

    visitLiteral (literal: Literal): Type {
        return infer(literal, () => {
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
        })
    }

    visitLookup (lookup: Lookup): Type {
        return infer(lookup, () => unknownType(lookup))
    }

    visitMatch (match: Match): Type {
        return infer(match, () => match.default.accept(this))
    }

    visitTypeConstructor (constructor: TypeConstructor): Type {
        return infer(constructor, () => type(constructor))
    }

    visitUnOp (unop: UnOp): Type {
        return infer(unop, () => unop.expr.accept(this))
    }

    /* Types */

    visitFieldType (field: FieldType): Type {
        return infer(field, () => type(field))
    }

    visitFunType (fun: FunType): Type {
        return infer(fun, () => type(fun))
    }

    visitInstanceType (instance: InstanceType): Type {
        return infer(instance, () => type(instance))
    }

    visitPrimitiveType (primitive: PrimitiveType): Type {
        return infer(primitive, () => type(primitive))
    }

    visitStructType (struct: StructType): Type {
        return infer(struct, () => type(struct))
    }

    visitUnknownType (unknown: UnknownType): Type {
        return infer(unknown, () => type(unknown))
    }

}
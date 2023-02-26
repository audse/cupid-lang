import { Assign, BinOp, Block, Call, Decl, Environment, Expr, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, PrimitiveType, StructType, Type, TypeConstructor, UnknownType } from './index'

export abstract class TypeVisitor<T> {
    abstract visitFieldType (field: FieldType): T
    abstract visitFunType (fun: FunType): T
    abstract visitInstanceType (instance: InstanceType): T
    abstract visitPrimitiveType (primitive: PrimitiveType): T
    abstract visitStructType (struct: StructType): T
    abstract visitUnknownType (unknown: UnknownType): T
}

export abstract class TypeVisitorWithContext<T, Ctx = never> {
    visit (type: Type, context: Ctx): T {
        return type.acceptWithContext(this, context)
    }
    abstract visitFieldType (field: FieldType, context: Ctx): T
    abstract visitFunType (fun: FunType, context: Ctx): T
    abstract visitInstanceType (instance: InstanceType, context: Ctx): T
    abstract visitPrimitiveType (primitive: PrimitiveType, context: Ctx): T
    abstract visitStructType (struct: StructType, context: Ctx): T
    abstract visitUnknownType (unknown: UnknownType, context: Ctx): T
}

export abstract class ExprVisitor<T> extends TypeVisitor<T> {
    visit (expr: Expr): T {
        return expr.accept<T>(this)
    }
    abstract visitAssign (assign: Assign): T
    abstract visitBinOp (binop: BinOp): T
    abstract visitBlock (block: Block): T
    abstract visitCall (call: Call): T
    abstract visitDecl (decl: Decl): T
    abstract visitEnvironment (env: Environment): T
    abstract visitFun (fun: Fun): T
    abstract visitIdent (ident: Ident): T
    abstract visitImpl (impl: Impl): T
    abstract visitLiteral (literal: Literal): T
    abstract visitLookup (lookup: Lookup): T
    abstract visitTypeConstructor (constructor: TypeConstructor): T
}

export abstract class ExprVisitorWithContext<T, Ctx = never> extends TypeVisitorWithContext<T, Ctx> {
    visit (expr: Expr, context: Ctx): T {
        return expr.acceptWithContext<T, Ctx>(this, context)
    }
    abstract visitAssign (assign: Assign, context: Ctx): T
    abstract visitBinOp (binop: BinOp, context: Ctx): T
    abstract visitBlock (block: Block, context: Ctx): T
    abstract visitCall (call: Call, context: Ctx): T
    abstract visitDecl (decl: Decl, context: Ctx): T
    abstract visitEnvironment (env: Environment, context: Ctx): T
    abstract visitFun (fun: Fun, context: Ctx): T
    abstract visitIdent (ident: Ident, context: Ctx): T
    abstract visitImpl (impl: Impl, context: Ctx): T
    abstract visitLiteral (literal: Literal, context: Ctx): T
    abstract visitLookup (lookup: Lookup, context: Ctx): T
    abstract visitTypeConstructor (constructor: TypeConstructor, context: Ctx): T
}
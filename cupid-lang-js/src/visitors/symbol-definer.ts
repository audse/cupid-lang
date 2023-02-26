import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Fun, Environment } from '@/ast'
import BaseExprVisitor from './base'

export default class SymbolDefiner extends BaseExprVisitor {

    visitDecl (decl: Decl): void {
        super.visitDecl(decl)
        decl.scope.define({
            ident: decl.ident,
            type: decl.type,
            value: decl.value,
            mutable: decl.mutable,
        })
    }

    visitEnvironment (env: Environment): void {
        super.visitEnvironment(env)
        if (env.ident) env.ident.scope.define({
            ident: env.ident,
            value: env
        })
    }

    visitFun (fun: Fun): void {
        fun.body.accept(this)
        fun.returns.accept(this)
        fun.params.map(param => param.scope.define({
            ident: param.ident,
            type: param.type,
        }))
    }

    visitTypeConstructor (constructor: TypeConstructor): void {
        super.visitTypeConstructor(constructor)
        constructor.params.map(param => param.scope.define({
            ident: param,
            value: new UnknownType({
                scope: param.scope,
                source: param.source,
            })
        }))
        constructor.scope.define({
            ident: constructor.ident,
            value: constructor
        })
    }

    /* Types */

    visitFieldType (field: FieldType): void {
        super.visitFieldType(field)
        field.scope.define({
            ident: field.ident,
            value: field.type
        })
    }

}
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Call } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor, { BaseExprVisitorWithContext } from './base'
import { TypeUnifier } from './type-unifier'

export default class TypeChecker extends BaseExprVisitorWithContext<TypeUnifier> {

    visitBinOp (binop: BinOp, unifier: TypeUnifier): void {
        super.visitBinOp(binop, unifier)
        unifier.visit(binop.left.expectType(), binop.right.expectType())
    }

    visitCall (call: Call, unifier: TypeUnifier): void {
        super.visitCall(call, unifier)
        const callType = call.fun.expectType() as FunType

        // Check number of arguments
        if (callType.params.length !== call.args.length) throw CompilationError.incorrectNumArgs(
            call,
            callType.params.length,
            call.args.length
        )

        // Check type of arguments
        call.args.map((arg, i) => {
            const param = callType.params[i]
            unifier.visit(arg.expectType(), param.type)
        })
    }

    visitDecl (decl: Decl, unifier: TypeUnifier): void {
        super.visitDecl(decl, unifier)
        const type = unifier.visit(decl.type, decl.value.expectType())
        decl.scope.annotate(decl.ident, { type })
    }

}
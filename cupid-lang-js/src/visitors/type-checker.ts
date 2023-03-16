import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Call, Lookup, Branch, Match, Fun } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor, { BaseExprVisitorWithContext } from './base'
import { TypeUnifier } from './type-unifier'

export default class TypeChecker extends BaseExprVisitorWithContext<TypeUnifier> {

    NonUnifiableOperations = new Set(['istype'])

    visitBinOp (binop: BinOp, unifier: TypeUnifier): void {
        super.visitBinOp(binop, unifier)
        if (!this.NonUnifiableOperations.has(binop.op)) unifier.visit(binop.left.expectType(), binop.right.expectType())
    }

    visitBranch (branch: Branch, unifier: TypeUnifier): void {
        super.visitBranch(branch, unifier)
        const branchType = branch.condition.expectType()
        if (!branchType.isBoolType()) throw CompilationError.incorrectType(
            branchType,
            'bool'
        )
    }

    visitCall (call: Call, unifier: TypeUnifier): void {
        super.visitCall(call, unifier)
        const callType = call.fun.expectType()

        if (callType instanceof FunType) {
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
    }

    visitDecl (decl: Decl, unifier: TypeUnifier): void {
        super.visitDecl(decl, unifier)
        const type = unifier.visit(decl.type, decl.value.expectType())
        decl.scope.annotate_ty(decl.ident, { type })
    }

    visitFun (fun: Fun, unifier: TypeUnifier): void {
        super.visitFun(fun, unifier)
        const type = fun.inferredType?.getResolved() as FunType
        fun.returns = unifier.visit(fun.returns, type.returns)
    }

    visitMatch (match: Match, unifier: TypeUnifier): void {
        match.expr.acceptWithContext(this, unifier)
        const type = match.expectType()

        for (const branch of match.branches) {
            branch.condition.acceptWithContext(this, unifier)
            branch.body.acceptWithContext(this, unifier)
            branch.else?.acceptWithContext(this, unifier)

            unifier.visit(type, branch.expectType())
        }

        unifier.visit(type, match.default.expectType())
    }

}
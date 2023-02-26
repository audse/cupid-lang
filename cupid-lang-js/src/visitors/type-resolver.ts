
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, Fun, Call } from '@/ast'
import { CompilationError } from '@/error/index'

export default class TypeResolver extends ExprVisitor<void> {

    visitAssign (assign: Assign): void {
        assign.ident.accept(this)
        assign.value.accept(this)
    }

    visitBinOp (binop: BinOp): void {
        binop.left.accept(this)
        binop.right.accept(this)
    }

    visitBlock (block: Block): void {
        block.exprs.map(expr => expr.accept(this))
    }

    visitCall (call: Call): void {
        call.fun.accept(this)
        call.args.map(arg => arg.accept(this))
    }

    visitDecl (decl: Decl): void {
        decl.ident.accept(this)
        decl.value.accept(this)
        decl.type.accept(this)
    }

    visitFun (fun: Fun): void {
        fun.params.map(param => param.accept(this))
        fun.body.accept(this)
        fun.returns.accept(this)
    }

    visitIdent (ident: Ident): void { }

    visitLiteral (literal: Literal): void { }

    visitTypeConstructor (constructor: TypeConstructor): void {
        constructor.ident.accept(this)
        constructor.params.map(param => param.accept(this))
        constructor.body.accept(this)
    }

    visitFieldType (field: FieldType): void {
        field.ident.accept(this)
        field.type.accept(this)
    }

    visitFunType (fun: FunType): void {
        fun.params.map(param => param.accept(this))
        fun.returns.accept(this)
    }

    visitInstanceType (instance: InstanceType): void {
        instance.ident.accept(this)
        const value = instance.ident.expectSymbol()

        if (value.value instanceof Type) instance.value = value.value
        else if (value.value instanceof TypeConstructor) {
            // Handle incorrect number of arguments error
            if (value.value.params.length !== instance.args.length) {
                throw CompilationError.incorrectNumArgs(instance, value.value.params.length, instance.args.length)
            }
            const constructor = value.value.cloneIntoScope(instance.scope)
            // Annotate constructor params with matching arguments
            constructor.params.map((param, i) => instance.scope.define({
                ident: param,
                value: instance.args[i]
            }))
            instance.value = constructor.body
        }
        else throw CompilationError.notAType(instance)

        instance.value?.accept(this)
    }

    visitPrimitiveType (primitive: PrimitiveType): void { }

    visitStructType (struct: StructType): void {
        struct.fields.map(field => field.accept(this))
    }

    visitUnknownType (unknown: UnknownType): void { }
}

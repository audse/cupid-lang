
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, Fun, Call, Impl } from '@/ast'
import { CompilationError } from '@/error/index'
import BaseExprVisitor from './base'
import Cloner from './cloner'

export default class TypeResolver extends BaseExprVisitor {

    visitImpl (impl: Impl): void {
        impl.type.accept(this)
        impl.environment.accept(this)
        impl.type.acceptEnvironmentMerge(impl.environment)
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
            // Only bother cloning if there are parameters
            if (value.value.params.length) {
                const cloner = new Cloner()
                const constructor = cloner.visitTypeConstructor(value.value, instance.scope)
                // Annotate constructor params with matching arguments
                constructor.params.map((param, i) => instance.scope.define({
                    ident: param,
                    value: instance.args[i]
                }))
                instance.value = constructor.body
            } else instance.value = value.value.body
        }
        else throw CompilationError.notAType(instance)

        instance.value?.accept(this)
    }

}

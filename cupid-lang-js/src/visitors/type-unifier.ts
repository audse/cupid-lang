
import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, TypeVisitor, UnknownType, Decl, Assign, Block, InstanceType, TypeVisitorWithContext } from '@/ast'
import { CompilationError } from '@/error/index'

export class TypeUnifier extends TypeVisitorWithContext<Type, Type> {

    visitFieldType (field: FieldType, context: Type): Type {
        const current = field.getResolved()
        const other = context.getResolved()
        if (other instanceof FieldType) return new FieldType({
            ...current,
            type: field.type.acceptWithContext(this, other.type)
        })
        throw CompilationError.unableToUnify(context)
    }

    visitFunType (fun: FunType, context: Type): Type {
        const current = fun.getResolved()
        const other = context.getResolved()
        if (other instanceof FunType) {
            if (current.params.length !== other.params.length) throw CompilationError.unableToUnify(other)

            const params: FieldType[] = []
            for (const param of current.params) {
                const otherParam = param.findMatch(other.params)
                if (otherParam) params.push(
                    param.acceptWithContext(this, otherParam) as FieldType
                )
            }
            const returns = current.returns.acceptWithContext(this, other.returns)
            return new FunType({
                ...current,
                params,
                returns
            })
        }
        throw CompilationError.unableToUnify(other)
    }

    visitInstanceType (instance: InstanceType, context: Type): Type {
        const current = instance.getResolved()
        const other = context.getResolved()
        return current.acceptWithContext(this, other)
    }

    visitPrimitiveType (primitive: PrimitiveType, context: Type): Type {
        const other = context.getResolved()
        // console.log('\nunifying', primitive.report(), other.report(), '\n')
        if (other instanceof PrimitiveType && primitive.name === other.name) return primitive
        if (other instanceof UnknownType) return primitive
        throw CompilationError.unableToUnify(other)
    }

    visitStructType (struct: StructType, context: Type): Type {
        const current = struct.getResolved()
        const other = context.getResolved()
        if (other instanceof StructType) {
            if (current.fields.length !== other.fields.length) throw CompilationError.unableToUnify(other)

            const fields: FieldType[] = []
            for (const field of current.fields) {
                const otherField = field.findMatch(other.fields)
                if (otherField) fields.push(
                    field.acceptWithContext(this, otherField) as FieldType
                )
                else throw CompilationError.unableToUnify(field)
            }

            return new StructType({
                ...current,
                fields
            })
        }
        if (other instanceof UnknownType) return current
        throw CompilationError.unableToUnify(other)
    }

    visitUnknownType (unknown: UnknownType, context: Type): Type {
        const other = context.getResolved()
        if (other instanceof UnknownType) return unknown
        return other
    }
}
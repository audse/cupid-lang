import { TypeVisitor, TypeVisitorWithContext } from '../visitor'
import { Type, TypeProps } from './type'
import FieldType from './field'
import { Context, Scope } from '@/env'


interface StructProps extends TypeProps {
    fields: FieldType[]
}


export default class StructType extends Type implements StructProps {

    fields: FieldType[]

    constructor (props: StructProps) {
        super(props)
        this.fields = props.fields
    }

    report (): string {
        const fields = this.fields.map(field => field.report())
        return `struct [${ fields.join(', ') }]`
    }

    getResolved (): StructType {
        const { scope, source, file, environment, inferredType } = this
        return new StructType({
            scope, source, file, environment, inferredType,
            fields: this.fields.map(field => field.getResolved()),
        })
    }

    isEqual (other: this): boolean {
        return this.fields.every((field, i) => field.isEqual(other.fields[i]))
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitStructType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitStructType(this, context)
    }

}
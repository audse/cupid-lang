import { Option } from '@/types'
import { Type, TypeProps } from './type'
import { ExprVisitor, ExprVisitorWithContext, TypeVisitor, TypeVisitorWithContext } from '../visitor'
import Ident from '../ident'
import { Scope } from '@/env'
import { Expr } from '../expr'

interface FieldProps extends TypeProps {
    ident: Ident
    type: Type
}

export default class FieldType extends Type implements FieldProps {
    ident: Ident
    type: Type

    constructor (props: FieldProps) {
        super(props)
        this.ident = props.ident
        this.type = props.type
    }

    report (): string {
        return `${ this.ident.report() } : ${ this.type.report() }`
    }

    getResolved (): FieldType {
        const { scope, source, ident, file, environment, inferredType } = this
        return new FieldType({
            scope, source, ident, file, environment, inferredType,
            type: this.type.getResolved(),
        })
    }

    isEqual (other: this): boolean {
        return (
            this.ident.isEqual(other.ident)
            && this.type.isEqual(other.type)
        )
    }

    findMatch (other: FieldType[]): Option<FieldType> {
        return other.find(field => this.ident.isEqual(field.ident)) || null
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitFieldType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitFieldType(this, context)
    }

}
import { Scope } from '@/env'
import { TypeVisitor, TypeVisitorWithContext } from '../visitor'
import { Type, TypeProps } from './type'


interface UnknownProps extends TypeProps { }


export default class UnknownType extends Type implements UnknownProps {

    constructor (props: UnknownProps) {
        super(props)
    }

    report (): string {
        return `<unknown>`
    }

    isEqual (other: this): boolean {
        return true
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitUnknownType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitUnknownType(this, context)
    }

}
import { Scope } from '@/env'
import Ident from '../ident'
import { TypeVisitor, TypeVisitorWithContext } from '../visitor'
import { Type, TypeProps } from './type'


interface PrimitiveProps extends TypeProps {
    name: string
}


export default class PrimitiveType extends Type implements PrimitiveProps {

    name: string

    constructor (props: PrimitiveProps) {
        super(props)
        this.name = props.name
    }

    report (): string {
        return this.name
    }

    isEqual (other: this): boolean {
        return this.name === other.name
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitPrimitiveType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitPrimitiveType(this, context)
    }

}
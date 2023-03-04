import { Scope } from '@/env'
import { Expr, ExprProps } from './expr'
import Ident from './ident'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'

export type LiteralValue = number | string | null | [number, number] | boolean

interface LiteralProps extends ExprProps {
    value: LiteralValue
}

export default class Literal extends Expr implements LiteralProps {

    value: LiteralValue

    constructor (props: LiteralProps) {
        super(props)
        this.value = props.value
    }

    intoIdent (): Ident {
        const name: string | number = (
            ['string', 'number'].includes(typeof this.value) ? this.value as string | number
                : (this.value || 'none').toString()
        )
        return new Ident({
            scope: this.scope,
            source: this.source,
            file: this.file,
            name,
            inferredType: this.inferredType,
        })
    }

    report (): string {
        return `${ this.value }`
    }

    isEqual (other: this): boolean {
        if (Array.isArray(this.value) && Array.isArray(other.value)) {
            return this.value[0] === other.value[0] && this.value[1] === other.value[1]
        }
        return this.value === other.value
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitLiteral(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitLiteral(this, context)
    }

}
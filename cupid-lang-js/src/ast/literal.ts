import { Scope } from '@/env'
import { Expr, ExprProps } from './expr'
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

    report (): string {
        return `${ this.value }`
    }

    isEqual (other: this): boolean {
        if (Array.isArray(this.value) && Array.isArray(other.value)) {
            return this.value[0] === other.value[0] && this.value[1] === other.value[1]
        }
        return this.value === other.value
    }

    cloneIntoScope (scope: Scope): Literal {
        return new Literal({
            scope,
            source: this.source,
            value: Array.isArray(this.value) ? [...this.value] : this.value,
        })
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitLiteral(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitLiteral(this, context)
    }

}
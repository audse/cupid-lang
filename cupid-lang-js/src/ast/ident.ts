import { CompilationError } from '@/error/compilation-error'
import { Option } from '@/types'
import { Expr, ExprProps } from './expr'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import { Scope, Symbol } from '@/env'


interface IdentProps extends ExprProps {
    name: string | number
}


export default class Ident extends Expr implements IdentProps {

    name: string | number
    symbol: Option<Symbol> = null

    constructor (props: IdentProps) {
        super(props)
        this.name = props.name
    }

    expectSymbol (): Symbol {
        if (this.symbol) return this.symbol
        return this.scope.lookupExpect(this)
    }

    report (): string {
        return this.name.toString()
    }

    isEqual (other: this): boolean {
        return this.name === other.name
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitIdent(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitIdent(this, context)
    }

}

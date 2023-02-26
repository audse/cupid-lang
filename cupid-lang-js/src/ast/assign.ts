import { Expr, ExprProps } from './expr'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import Ident from './ident'
import { stringify } from '@/utils'
import { Scope } from '@/env'

interface AssignProps extends ExprProps {
    ident: Ident
    value: Expr
}

export default class Assign extends Expr implements AssignProps {
    ident: Ident
    value: Expr

    constructor (props: AssignProps) {
        super(props)
        this.ident = props.ident
        this.value = props.value
    }

    report (): string {
        const ident = this.ident.report()
        const value = this.value.report()
        return stringify({ assign: { ident, value } })
    }

    isEqual (other: this): boolean {
        return false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitAssign(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitAssign(this, context)
    }
}
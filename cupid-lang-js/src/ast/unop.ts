import { Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'

interface UnOpProps extends ExprProps {
    expr: Expr
    op: string
}

export default class UnOp extends Expr implements UnOpProps {
    expr: Expr
    op: string

    constructor (props: UnOpProps) {
        super(props)
        this.expr = props.expr
        this.op = props.op
    }

    report (): string {
        const expr = this.expr.report()
        return stringify({ operation: { expr, operator: this.op } })
    }

    isEqual (other: this): boolean {
        return false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitUnOp(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitUnOp(this, context)
    }
}
import { Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'

interface BinOpProps extends ExprProps {
    left: Expr
    right: Expr
    op: string
}

export default class BinOp extends Expr implements BinOpProps {
    left: Expr
    right: Expr
    op: string

    constructor (props: BinOpProps) {
        super(props)
        this.left = props.left
        this.right = props.right
        this.op = props.op
    }

    report (): string {
        const left = this.left.report()
        const right = this.right.report()
        return stringify({ operation: { left, right, operator: this.op } })
    }

    isEqual (other: this): boolean {
        return false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitBinOp(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitBinOp(this, context)
    }
}
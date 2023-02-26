import { Context, Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'


interface BlockProps extends ExprProps {
    exprs: Expr[]
}


export default class Block extends Expr implements BlockProps {

    exprs: Expr[]

    constructor (props: BlockProps) {
        super(props)
        this.exprs = props.exprs
    }

    report (): string {
        const exprs = this.exprs.map(expr => expr.report())
        return stringify({ block: exprs })
    }

    isEqual (other: this): boolean {
        return false
    }

    cloneIntoScope (scope: Scope): Block {
        const subscope = scope.subscope(Context.Block)
        return new Block({
            scope: subscope,
            source: this.source,
            exprs: this.exprs.map(expr => expr.cloneIntoScope(subscope))
        })
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitBlock(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitBlock(this, context)
    }

}

import { stringify } from '@/utils'
import { Branch, Expr, ExprProps } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'

interface MatchProps extends ExprProps {
    expr: Expr
    branches: Branch[]
    default: Expr
}

export default class Match extends Expr implements MatchProps {

    expr: Expr
    branches: Branch[]
    default: Expr

    constructor (props: MatchProps) {
        super(props)
        this.expr = props.expr
        this.branches = props.branches
        this.default = props.default
    }

    report (): string {
        return stringify({
            match: {
                expr: this.expr.report(),
                branches: this.branches.map(branch => branch.report()),
                default: this.default.report()
            }
        })
    }

    isEqual (other: this): boolean {
        return false // TODO
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitMatch(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitMatch(this, context)
    }

}
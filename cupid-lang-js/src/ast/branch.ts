import { Option } from '@/types'
import { Context, Scope, Symbol } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { Environment, FieldType, Ident, Literal, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import { CompilationError } from '@/error/compilation-error'


interface BranchProps extends ExprProps {
    condition: Expr
    body: Expr
    else?: Expr
}


export default class Branch extends Expr implements BranchProps {

    condition: Expr
    body: Expr
    else?: Expr

    constructor (props: BranchProps) {
        super(props)
        this.condition = props.condition
        this.body = props.body
        this.else = props.else
    }

    report (): string {
        return stringify({ lookup: this.scope.report() })
    }

    isEqual (other: this): boolean {
        return (
            this.condition.isEqual(other.condition)
            && this.body.isEqual(other.body)
        )
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitBranch(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitBranch(this, context)
    }

}

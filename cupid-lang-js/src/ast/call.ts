import { Context, Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { FieldType, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'


interface CallProps extends ExprProps {
    fun: Expr
    args: Expr[]
}


export default class Call extends Expr implements CallProps {

    fun: Expr
    args: Expr[]

    constructor (props: CallProps) {
        super(props)
        this.fun = props.fun
        this.args = props.args
    }

    report (): string {
        return stringify({
            call: {
                fun: this.fun.report(),
                args: this.args.map(arg => arg.report()),
            }
        })
    }

    isEqual (other: this): boolean {
        return false
    }

    cloneIntoScope (scope: Scope): Call {
        const subscope = scope.subscope(Context.Call)
        return new Call({
            scope: subscope,
            source: this.source,
            fun: this.fun.cloneIntoScope(subscope),
            args: this.args.map(arg => arg.cloneIntoScope(subscope)),
        })
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitCall(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitCall(this, context)
    }

}

import { Context, Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { FieldType, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'


interface FunProps extends ExprProps {
    params: FieldType[]
    body: Expr
    returns?: Type
}


export default class Fun extends Expr implements FunProps {

    params: FieldType[]
    body: Expr
    returns: Type

    constructor (props: FunProps) {
        super(props)
        this.params = props.params
        this.body = props.body
        this.returns = props.returns || new UnknownType({ scope: this.scope })
    }

    report (): string {
        return stringify({
            fun: {
                params: this.params.map(param => param.report()),
                body: this.body.report(),
                returns: this.returns.report()
            }
        })
    }

    isEqual (other: this): boolean {
        return false
    }

    cloneIntoScope (scope: Scope): Fun {
        const subscope = scope.subscope(Context.Fun)
        return new Fun({
            scope: subscope,
            source: this.source,
            params: this.params.map(param => param.cloneIntoScope(subscope)),
            body: this.body.cloneIntoScope(subscope),
            returns: this.returns.cloneIntoScope(subscope)
        })
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitFun(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitFun(this, context)
    }

}

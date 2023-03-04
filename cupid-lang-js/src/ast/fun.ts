import { Context, Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { FieldType, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'


interface FunProps extends ExprProps {
    hasSelfParam?: boolean
    params: FieldType[]
    body: Expr
    returns?: Type
}


export default class Fun extends Expr implements FunProps {

    hasSelfParam: boolean
    params: FieldType[]
    body: Expr
    returns: Type

    constructor (props: FunProps) {
        super(props)
        this.params = props.params
        this.body = props.body
        this.returns = props.returns || new UnknownType({ scope: this.scope, source: this.source, file: this.file })
        this.hasSelfParam = props.hasSelfParam || false
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

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitFun(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitFun(this, context)
    }

}

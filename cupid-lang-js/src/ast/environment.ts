import { Option } from '@/types'
import { Context, Scope } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { FieldType, Ident, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'


interface EnvironmentProps extends ExprProps {
    ident?: Ident
    content: Expr[]
}


export default class Environment extends Expr {

    ident: Option<Ident>
    content: Expr[] = []

    constructor (props: EnvironmentProps) {
        super(props)
        this.ident = props.ident || null
        this.content = props.content
    }

    acceptMerge (other: Environment): void {
        this.content.push(...other.content)
        this.scope.acceptMerge(other.scope)
    }

    report (): string {
        return stringify({ environment: this.scope.report() })
    }

    isEqual (other: this): boolean {
        return this.ident && other.ident ? this.ident.isEqual(other.ident) : false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitEnvironment(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitEnvironment(this, context)
    }

}

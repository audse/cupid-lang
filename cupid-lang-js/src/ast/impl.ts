import { Option } from '@/types'
import { Type, TypeProps } from './type-system/type'
import { ExprVisitor, ExprVisitorWithContext, TypeVisitor, TypeVisitorWithContext } from './visitor'
import Ident from './ident'
import { Scope } from '@/env'
import { Expr } from './expr'
import Fun from './fun'
import Environment from './environment'
import { stringify } from '@/utils'

interface ImplProps extends TypeProps {
    type: Type
    environment: Environment
}

export default class Impl extends Expr implements ImplProps {

    type: Type
    environment: Environment

    constructor (props: ImplProps) {
        super(props)
        this.type = props.type
        this.environment = props.environment
    }

    report (): string {
        return stringify({
            impl: this.type.report(),
            env: this.environment.report()
        })
    }

    isEqual (other: this): boolean {
        throw 'unimplemented'
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitImpl(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitImpl(this, context)
    }

}
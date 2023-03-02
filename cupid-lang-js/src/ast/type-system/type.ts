import { Scope } from '@/env'
import Environment from '../environment'
import { Expr, ExprProps } from '../expr'
import { TypeVisitor, TypeVisitorWithContext } from '../visitor'

export interface TypeProps extends ExprProps {
    environment?: Environment
}

export abstract class Type extends Expr {
    environment: Environment

    constructor (props: TypeProps) {
        super(props)
        this.environment = props.environment || new Environment({
            scope: this.scope,
            source: this.source,
            content: [],
        })
    }

    acceptEnvironmentMerge (env: Environment): void {
        this.environment.acceptMerge(env)
    }

    getResolved (): Type {
        return this
    }

    abstract accept<T> (visitor: TypeVisitor<T>): T
    abstract acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T

    isBoolType (): boolean {
        return false
    }

}
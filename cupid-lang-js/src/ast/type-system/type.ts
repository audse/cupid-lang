import { Scope } from '@/env'
import { Expr, ExprProps } from '../expr'
import { TypeVisitor, TypeVisitorWithContext } from '../visitor'

export interface TypeProps extends ExprProps { }

export abstract class Type extends Expr implements TypeProps {

    constructor (props: TypeProps) {
        super(props)
    }

    getResolved (): Type {
        return this
    }

    abstract cloneIntoScope (scope: Scope): Type
    abstract accept<T> (visitor: TypeVisitor<T>): T
    abstract acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T

}
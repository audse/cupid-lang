import { Expr, ExprProps } from './expr'
import { Type } from './type-system/type'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import Ident from './ident'
import { bracket } from '@/codegen'
import { Context, Scope } from '@/env'

export interface TypeConstructorProps extends ExprProps {
    ident: Ident
    params: Ident[]
    body: Type
}

export default class TypeConstructor extends Expr implements TypeConstructorProps {

    ident: Ident
    params: Ident[]
    body: Type

    constructor (props: TypeConstructorProps) {
        super(props)
        this.ident = props.ident
        this.params = props.params
        this.body = props.body
    }

    report (): string {
        const ident = this.ident.report()
        const params = this.params.map(param => param.report())
        return `${ ident }${ params.length ? bracket(params.join(', ')) : '' } -> ${ this.body.report() }`
    }

    isEqual (other: this): boolean {
        return false
    }

    cloneIntoScope (scope: Scope): TypeConstructor {
        const subscope = scope.subscope(Context.TypeConstructor)
        return new TypeConstructor({
            scope: subscope,
            source: this.source,
            ident: this.ident.cloneIntoScope(subscope),
            params: this.params.map(param => param.cloneIntoScope(subscope)),
            body: this.body.cloneIntoScope(subscope)
        })
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitTypeConstructor(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitTypeConstructor(this, context)
    }

}

import { Option } from '@/types'
import { Context, Scope, Symbol } from '@/env'
import { stringify } from '@/utils'
import { Expr, ExprProps } from './expr'
import { Environment, FieldType, Ident, Literal, Type, UnknownType } from './index'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import { CompilationError } from '@/error/compilation-error'


interface LookupProps extends ExprProps {
    environment: Expr
    member: Ident | Literal
}


export default class Lookup extends Expr implements LookupProps {

    environment: Expr
    member: Ident | Literal

    constructor (props: LookupProps) {
        super(props)
        this.environment = props.environment
        this.member = props.member
    }

    report (): string {
        return stringify({ Lookup: this.scope.report() })
    }

    isEqual (other: this): boolean {
        if (!this.environment.isEqual(other.environment)) return false
        if (this.member instanceof Ident && other.member instanceof Ident) return this.member.isEqual(other.member)
        if (this.member instanceof Literal && other.member instanceof Literal) return this.member.isEqual(other.member)
        return false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitLookup(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitLookup(this, context)
    }

}

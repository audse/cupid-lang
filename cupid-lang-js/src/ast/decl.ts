import { Expr, ExprProps } from './expr'
import { Type } from './type-system/type'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'
import Ident from './ident'
import UnknownType from './type-system/unknown'
import { stringify } from '@/utils'
import { Scope } from '@/env'

interface DeclProps extends ExprProps {
    ident: Ident
    value: Expr
    type?: Type
    mutable?: boolean
}

export default class Decl extends Expr implements DeclProps {
    ident: Ident
    value: Expr
    type: Type
    mutable: boolean = false

    constructor (props: DeclProps) {
        super(props)
        this.ident = props.ident
        this.value = props.value
        this.type = props.type || new UnknownType({ scope: this.scope })
        this.mutable = props.mutable || false
    }

    report (): string {
        const ident = this.ident.report()
        const value = this.value.report()
        const type = this.type.report()
        return stringify({ declare: { ident, value, type, mutable: this.mutable } })
    }

    isEqual (other: this): boolean {
        return false
    }

    accept<T> (visitor: ExprVisitor<T>): T {
        return visitor.visitDecl(this)
    }

    acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitDecl(this, context)
    }

}
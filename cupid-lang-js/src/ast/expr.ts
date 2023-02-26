import { Scope } from '@/env'
import { CompilationError, Reportable } from '@/error/index'
import { Option } from '@/types'
import { Type } from './type-system/type'
import { ExprVisitor, ExprVisitorWithContext } from './visitor'

export interface ExprProps {
    source?: number
    scope: Scope
}

export abstract class Expr implements ExprProps, Reportable {
    source: number
    scope: Scope
    inferredType: Option<Type> = null

    constructor (props: ExprProps) {
        this.source = props.source || -1
        this.scope = props.scope
    }

    expectType (): Type {
        if (this.inferredType) return this.inferredType
        throw CompilationError.cannotInfer(this)
    }

    log () {
        console.log(this.report())
    }

    abstract report (): string
    abstract cloneIntoScope (scope: Scope): Expr
    abstract isEqual (other: typeof this): boolean
    abstract accept<T> (visitor: ExprVisitor<T>): T
    abstract acceptWithContext<T, Ctx> (visitor: ExprVisitorWithContext<T, Ctx>, context: Ctx): T
}

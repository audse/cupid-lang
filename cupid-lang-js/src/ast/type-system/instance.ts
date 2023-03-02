import { CompilationError, CompilationErrorCode } from '../../error/compilation-error'
import { Option } from '@/types'
import Ident from '../ident'
import { TypeVisitor, TypeVisitorWithContext } from '../visitor'
import { Type, TypeProps } from './type'
import { Reportable } from '@/error/index'
import { bracket } from '@/codegen'
import { stringify } from '@/utils'
import { Context, Scope } from '@/env'
import environment from '../environment'

interface InstanceProps extends TypeProps {
    ident: Ident
    args: Type[]
}

export default class InstanceType extends Type implements InstanceProps, Reportable {

    ident: Ident
    args: Type[] = []

    value: Option<Type> = null

    constructor (props: InstanceProps) {
        super(props)
        this.ident = props.ident
        this.args = props.args
    }

    report (): string {
        const args = this.args.length ? ' ' + bracket(this.args.map(arg => arg.report()).join(', ')) : ''
        const instance = `${ this.ident.report() }${ args }`
        if (this.value === null) return `unresolved ${ instance }`
        else return stringify({ instance: { ident: instance, resolved: this.value.report() } })
    }

    isEqual (other: this): boolean {
        if (this.value) return this.value.isEqual(other)
        return (
            this.ident.isEqual(other.ident)
            && this.args.every((arg, i) => other.args.length > i && arg.isEqual(other.args[i]))
        )
    }

    acceptEnvironmentMerge (env: environment): void {
        if (this.value) {
            this.value.acceptEnvironmentMerge(env)
            this.environment = this.value.environment
        }
        else super.acceptEnvironmentMerge(env)
    }

    getResolved (): Type {
        if (this.value) return this.value.getResolved()
        throw new CompilationError(
            CompilationErrorCode.UnableToResolveType,
            this
        )
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitInstanceType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitInstanceType(this, context)
    }

    isBoolType (): boolean {
        return this.getResolved().isBoolType()
    }

}
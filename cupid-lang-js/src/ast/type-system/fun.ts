import { TypeVisitor, TypeVisitorWithContext } from '../visitor'
import { Type, TypeProps } from './type'
import FieldType from './field'
import { paren } from '@/codegen'
import { Scope } from '@/env'

interface FunProps extends TypeProps {
    params: FieldType[]
    returns: Type
}

export default class FunType extends Type implements FunProps {

    params: FieldType[]
    returns: Type

    constructor (props: FunProps) {
        super(props)
        this.params = props.params
        this.returns = props.returns
    }

    report (): string {
        const params = this.params.map(param => param.report())
        return `${ paren(params.join(', ')) } -> ${ this.returns.report() }`
    }

    getResolved (): FunType {
        const { scope, source, file, environment, inferredType } = this
        return new FunType({
            scope, source, file, environment, inferredType,
            params: this.params.map(param => param.getResolved()),
            returns: this.returns.getResolved(),
        })
    }

    isEqual (other: this): boolean {
        return (
            this.params.every((param, i) => param.isEqual(other.params[i]))
            && this.returns.isEqual(other.returns)
        )
    }

    accept<T> (visitor: TypeVisitor<T>): T {
        return visitor.visitFunType(this)
    }

    acceptWithContext<T, Ctx> (visitor: TypeVisitorWithContext<T, Ctx>, context: Ctx): T {
        return visitor.visitFunType(this, context)
    }

}
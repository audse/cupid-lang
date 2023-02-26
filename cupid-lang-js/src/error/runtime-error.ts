import { Expr } from '@/ast/expr'
import { reindent } from '@/codegen'
import { CupidError, Reportable } from './error'

export enum RuntimeErrorCode {
    // NotAFunction = 'not a function',
    Unimplemented = 'unimplemented',
    Unreachable = 'unreachable',
}

export class RuntimeError<T extends Reportable> extends CupidError<T> implements Reportable {

    code: RuntimeErrorCode = RuntimeErrorCode.Unimplemented
    message: string = ''
    context: T

    constructor (code: RuntimeErrorCode, context: T, message: string = '') {
        super(code, context, message)
        this.code = code
        this.message = message
        this.context = context
    }

    // static notAFunction<T extends Reportable> (context: T, message: string): RuntimeError<T> {
    //     return new RuntimeError(RuntimeErrorCode.NotAFunction)
    // }

    static unimplemented<T extends Reportable> (context: T, message: string = '') {
        return new RuntimeError(RuntimeErrorCode.Unimplemented, context, message)
    }

    static unreachable<T extends Reportable> (context: T, message: string = '') {
        return new RuntimeError(RuntimeErrorCode.Unreachable, context, message)
    }

}
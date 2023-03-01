import { CupidError, Reportable } from './error'

export enum RuntimeErrorCode {
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

    static unimplemented<T extends Reportable> (context: T, message: string = '') {
        return new RuntimeError(RuntimeErrorCode.Unimplemented, context, message)
    }

    static unreachable<T extends Reportable> (context: T, message: string = '') {
        return new RuntimeError(RuntimeErrorCode.Unreachable, context, message)
    }

}
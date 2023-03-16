import { pluralize } from '@/utils'
import { CupidErr, Reportable } from './error'

export enum CompilationErrorCode {
    AlreadyDefined = 'already defined',
    CannotInfer = 'cannot infer',
    Immutable = 'immutable',
    IncorrectNumberOfArgs = 'incorrect number of args',
    IncorrectType = 'incorrect type',
    NotAFunction = 'not a function',
    NotAType = 'not a type',
    NotDefined = 'not defined',
    UnableToResolveLookup = 'unable to resolve lookup',
    UnableToResolveType = 'unable to resolve type',
    UnableToUnifyType = 'unable to unify type',
    Unimplemented = 'unimplemented',
    Unreachable = 'unreachable',
}

export class CompilationError<T extends Reportable> extends CupidErr<T> {

    code: CompilationErrorCode = CompilationErrorCode.Unimplemented
    message: string = ''
    context: T

    constructor (code: CompilationErrorCode, context: T, message: string = '') {
        super(code, context, message)
        this.code = code
        this.context = context
        this.message = message
    }

    static alreadyDefined<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.AlreadyDefined, context, message)
    }

    static cannotInfer<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.CannotInfer, context, message)
    }

    static immutable<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(
            CompilationErrorCode.Immutable,
            context,
            message.length ? message : 'cannot assign to immutable variable'
        )
    }

    static incorrectNumArgs<T extends Reportable> (context: T, expected: number, found: number): CompilationError<T> {
        return new CompilationError(
            CompilationErrorCode.IncorrectNumberOfArgs,
            context,
            `expected ${ expected } ${ pluralize('argument', expected) }, but called with ${ found }`
        )
    }

    static incorrectType<T extends Reportable> (context: T, expected: string | Reportable): CompilationError<T> {
        return new CompilationError(
            CompilationErrorCode.IncorrectType,
            context,
            `expected type ${ typeof expected === 'string' ? expected : expected.report() }`
        )
    }

    static notAFunction<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.NotAFunction, context, message)
    }

    static notAType<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.NotAType, context, message)
    }

    static notDefined<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.NotDefined, context, message)
    }

    static unableToResolveLookup<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(
            CompilationErrorCode.UnableToResolveLookup,
            context,
            message.length ? message : `unable to statically resolve member lookup - member must be a constant expression`
        )
    }

    static unableToUnify<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.UnableToUnifyType, context, message)
    }

    static unimplemented<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.Unimplemented, context, message)
    }

    static unreachable<T extends Reportable> (context: T, message: string = ''): CompilationError<T> {
        return new CompilationError(CompilationErrorCode.Unreachable, context, message)
    }

}
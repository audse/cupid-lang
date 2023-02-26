import { reindent } from '@/codegen'
import { color, ConsoleFgColorBright, logColor, logFgRed } from '@/console'
import { Result } from '@/types'
import { CompilationError } from './compilation-error'
import { AnyExpr, Err, ErrorCode } from './index'
import { RuntimeError } from './runtime-error'

export function err (code: ErrorCode, context: string = '', expr: Err['expr']): Result<any, Err> {
    return {
        ok: false,
        err: {
            code,
            context,
            expr
        }
    }
}

export function ok<T> (result: T): Result<T, any> {
    return {
        ok: true,
        result
    }
}

export interface Reportable {
    report: () => string
    log: () => void
}

export class CupidError<T extends Reportable> extends Error implements Reportable {
    code: string | number = 404
    message: string = ''
    context: T

    constructor (code: string | number, context: T, message: string = '') {
        super(message)
        this.code = code
        this.message = message
        this.context = context
    }

    title () {
        return `error: ${ this.code } ${ this.message ? ['-', this.message].join(' ') : '' }`
    }

    report () {
        return reindent(`
            ${ this.title() }
            ${ this.context.report() }
        `.trim())
    }

    log () {
        console.trace()
        this.logTitle()
        if (this.message.length) this.logMessage()
        this.context.log()
        console.log()
    }

    logTitle () {
        console.log(color().underline.red(), `\nerror:`, color().red(), this.code)
    }

    logMessage () {
        console.log(color().dim(), this.message)
    }
}

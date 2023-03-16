import { reindent } from '@/codegen'
import { color } from '@/console'

export interface Reportable {
    report: () => string
    log: () => void
}

export class CupidErr<T extends Reportable> extends Error implements Reportable {
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
        console.log(`\n${ color.underline.red('error') }${ color.red(':') }`, color.red(this.code.toString()))
    }

    logMessage () {
        console.log(color.dim(this.message))
    }
}

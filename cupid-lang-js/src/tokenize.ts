import { Option, token } from '@/types'

type TokenizerPosition = token.Position & {
    index: number
}

const lowercaseLetters = 'abcdefghijklmnopqrstuvwxyz'
const uppercaseLetters = lowercaseLetters.toUpperCase()

const letters = new Set([...lowercaseLetters.split(''), ...uppercaseLetters.split('')])
const digits = new Set('0123456789'.split(''))
const identStartChars = new Set([...letters, '@', '#', '$', '_'])
const identChars = new Set([...letters, ...digits, '@', '#', '$', '_', '&', '-'])

function isDigit (char: string): boolean {
    return digits.has(char)
}

function isIdent (char: string): boolean {
    return identChars.has(char)
}

export class Tokenizer {

    tokens: token.Token[] = []

    start: TokenizerPosition = {
        line: 0,
        column: 0,
        index: 0
    }

    position: TokenizerPosition = {
        line: 0,
        column: 0,
        index: 0
    }

    file: token.File
    content: string

    constructor (file: token.File, content: string) {
        this.file = file
        this.content = content
    }

    tokenize (): token.Token[] {
        let c: string | null = this.#current()
        while (c) {
            const type = this.#charType(c)
            if (type === token.Char.Newline) {
                this.position.line += 1
                this.position.column = 0
            }
            else if (type === token.Char.Quote) this.#str()
            else if (type === token.Char.Number) this.#number()
            else if (type === token.Char.Ident) this.#ident()
            else if (type === token.Char.Symbol) {
                // line comments
                if (c === '-' && this.#peek(1) === '-') this.#lineComment()
                // block comments
                else if (c === '*' && this.#peek(1) === '*' && this.#peek(2) === '*') this.#blockComment()
                else this.#symbol()
            }
            c = this.#next()
            this.start = { ...this.position }
        }
        return this.tokens
    }

    #current (): string {
        return this.content[this.position.index]
    }

    #peek (amt: number = 1): string {
        return this.content[this.position.index + amt]
    }

    #next (): string | null {
        this.position.index += 1
        this.position.column += 1
        if (this.content.length > this.position.index) {
            return this.#current()
        }
        return null
    }

    #commit (type: token.Type) {
        const token = {
            content: this.content.substring(this.start.index, this.position.index + 1),
            start: { ...this.start },
            end: { ...this.position },
            file: this.file,
            type,
        }
        this.start = this.position
        this.tokens.push(token)
    }

    #charType (char: string): token.Char {
        switch (char) {
            case '\n': return token.Char.Newline
            case ' ': case '\t': return token.Char.Space
            case '\'': case '"': case '`': return token.Char.Quote
            default: {
                if (isDigit(char)) return token.Char.Number
                if (identStartChars.has(char)) return token.Char.Ident
                return token.Char.Symbol
            }
        }
    }

    #ident () {
        while (true) {
            const next = this.#peek(1)
            if (isIdent(next)) this.#next()
            else break
        }
        this.#commit(token.Type.Ident)
    }

    #symbol () {
        const curr = this.#current()
        const next = this.#peek(1)

        const isShift = (curr === '>' && next === '>') || (curr === '<' && next === '<')
        const isCompare = ['<', '>', '!', '='].includes(curr) && ['='].includes(next)

        if (isShift || isCompare) this.#next()

        this.#commit(token.Type.Symbol)
    }

    #str () {
        const quote = this.#current()
        while (true) {
            const next = this.#next()
            if (!next || next === quote) break
        }
        this.#commit(token.Type.Str)
    }

    #number () {
        while (true) {
            const next = this.#peek(1)
            const nextIsDigit = isDigit(next) || next === '_'
            const nextIsDecimal = next === '.' && isDigit(this.#peek(2))
            if (nextIsDigit || nextIsDecimal) this.#next()
            else break
        }
        this.#commit(token.Type.Number)
    }

    #lineComment () {
        while (true) {
            const next = this.#peek(1)
            if (!next || next === '\n') break
            else this.#next()
        }
        this.#commit(token.Type.Comment)
    }

    #blockComment () {
        let numAsterisks = 0
        while (numAsterisks < 3) {
            const next = this.#next()
            if (next === '*') numAsterisks += 1
            else numAsterisks = 0
        }
        this.#commit(token.Type.Comment)
    }
}
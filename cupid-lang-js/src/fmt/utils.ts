import { color, colorString, ConsoleColor, ConsoleColorModifier, ConsoleFgColor } from '@/console'
import { token } from '@/types'

export function tabsToSpaces (line: string): string {
    return line.replace(/\t/g, ' ')
}

export function underline (len: number, char: string = '^'): string {
    return new Array(len).fill(char).join('')
}

export function replaceSubstring (str: string, using: string, from: number, to?: number): string {
    return `${ str.substring(0, from) }${ using }${ str.substring(to || str.length) }`
}

interface FileFormatterProps {
    path: string
    file: string
    fileId: number

    lines: string[]
    totalLines: number
}

export class FileFormatter implements FileFormatterProps {

    path: string
    file: string
    fileId: number

    lines: string[]
    formatted: string[] = []
    totalLines: number

    #useConsoleColors: boolean = false

    constructor (path: string, file: string, id: number) {
        this.path = path
        this.file = file
        this.fileId = id

        this.lines = file.split('\n')
        this.formatted = this.lines
            .map(line => [line, ''.padStart(line.length, ' ')]) // insert empty line between each line
            .flat()
        this.totalLines = this.lines.length
    }

    useConsoleColors (val: boolean = true): this {
        this.#useConsoleColors = val
        return this
    }

    #color (input: string, ...colors: ConsoleColor[]): string {
        return this.#useConsoleColors ? colorString(input, ...colors) : input
    }

    #red (input: string): string {
        return this.#color(input, ConsoleFgColor.Red)
    }

    #dim (input: string): string {
        return this.#color(input, ConsoleColorModifier.Dim)
    }

    get #lineNumberSeparator (): string {
        return ' │ '
    }

    get #lineNumberPadding (): number {
        return this.totalLines.toString().length
    }

    #number (line: string, i: number): string {
        const lineNumber = this.#dim(
            `${ (i % 2 ? '' : i / 2 + 1).toString().padStart(this.#lineNumberPadding) }${ this.#lineNumberSeparator }`
        )
        if (i % 2) return `${ lineNumber }${ this.#red(line) }`
        return `${ lineNumber }${ line }`
    }

    useLineNumbers (): this {
        this.formatted = this.formatted.map(this.#number.bind(this))
        return this
    }

    underline (lineNum: number, startIndex: number, endIndex?: number) {
        const start = startIndex
        const end = endIndex !== undefined ? endIndex : this.lines[lineNum].length
        this.formatted[lineNum] = replaceSubstring(
            this.formatted[lineNum],
            underline(end - start, '^'),
            start,
            end
        )
    }

    underlineToken (tkn: token.Token): this {
        const start = (tkn.start.line * 2) + 1
        const end = (tkn.end.line * 2) + 1
        for (let line = start; line <= end; line += 2) this.underline(
            line,
            line === start ? tkn.start.column - 1 : 0,
            line === end ? tkn.end.column : undefined
        )
        return this
    }

    build (startLine = 0, endLine = this.formatted.length): string {
        const start = Math.max(startLine * 2, 0)
        const end = Math.min(endLine * 2, this.formatted.length)
        return [
            this.#color(this.path, ConsoleColorModifier.Italic, ConsoleFgColor.Gray),
            this.#dim(`┬─`.padStart(this.#lineNumberPadding + this.#lineNumberSeparator.length, '─').padEnd(this.path.length + 2, '─')),
            ...this.formatted.slice(start, end)
        ].join('\n')
    }

}
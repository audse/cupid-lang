import { mapEntries } from '@/utils'

import { AnonFuncParams, AssignParams, Config, Formatter, FuncParams, FunctionConfig, GeneratedString, IfStmtParams, Literal, NamespaceParams, ObjectLiteral, TypeParams, VariableParams } from '@/types'


export function namespace ({ name, statements, ...config }: NamespaceParams): GeneratedString {
    return kw.exported(`namespace ${ name } {
            ${ joined(statements, '\n') }
        }`, config)
}

/** Type generation */

export function type ({ name, value, ...config }: TypeParams<GeneratedString>): GeneratedString {
    const and = fmt(config.and, and => `${ (Array.isArray(and) ? and : [and]).join(' & ') } & `)
    return kw.exported(`type ${ name } = ${ and }${ value }`, config)
}

export function union ({ name, value, ...config }: TypeParams<GeneratedString[]>): GeneratedString {
    const lines = value.join('\n\t| ')
    return type({ name, value: `(\n${ lines }\n)`, ...config })
}

export function literalUnion ({ name, value, ...config }: TypeParams<Literal[]>): GeneratedString {
    return union({ name, value: value.map(literal), ...config })
}

/** Function generation */

export function func ({ name, params, statements, ...config }: FuncParams): GeneratedString {
    return kw.exported(
        kw.async([
            `function ${ name } ${ paren(params.join(', ')) }${ returnType(config) } {`,
            statements.join('\n'),
            '}'
        ].join('\n'), config
        ), config)
}

export function anonFunc ({ params, statements, ...config }: AnonFuncParams): GeneratedString {
    const body = Array.isArray(statements) ? `{\n${ statements.join('\n') }\n}` : statements
    return kw.async(
        `${ paren(params.join(', ')) }${ returnType(config) } => ${ body }`,
        config
    )
}

function returnType (config: FunctionConfig): GeneratedString {
    return fmt(config.type, t => `: ${ t }`)
}

/** Statement generation */


export function ifStmt ({ compare, thenDo, elseDo, elseIf }: IfStmtParams): GeneratedString {
    const makeBody = (stmts: string | string[]) => Array.isArray(stmts) ? brace(stmts.join('\n')) : stmts
    let stmts = [`if (${ compare }) ${ makeBody(thenDo) }`]
    if (elseIf) stmts = stmts.concat(elseIf.map(ifStmt).map(stmt => `else ${ stmt }`))
    if (elseDo) stmts.push(`else ${ makeBody(elseDo) }`)
    return stmts.join('\n')
}

/** Variable generation */

export function constant ({ name, value, type, ...config }: VariableParams): GeneratedString {
    return kw.exported(`const ${ assign({ name, type, value }) }`, config)
}

export function variable ({ name, value, type, ...config }: VariableParams): GeneratedString {
    return kw.exported(`let ${ assign({ name, type, value }) }`, config)
}

export function assign ({ name, type, value }: AssignParams): GeneratedString {
    return ident(name, type) + fmt(value, v => ` = ${ v }`)
}

function ident (name: string, type?: string): GeneratedString {
    return name + fmt(type, t => `: ${ t }`)
}

/** Value generation */

export function literal (from: Literal): GeneratedString {
    if (Array.isArray(from)) return arrayLiteral(from)
    switch (typeof from) {
        case 'string': return quote(from as string)
        case 'object' && from: return objectLiteral(from as ObjectLiteral)
        default: return `${ from }`
    }
}

export function object (...entries: GeneratedString[][]): GeneratedString {
    return entries.map(([key, val]) => {
        return `${ key }: ${ val },`
    }).join('\n')
}

export function array (...elements: GeneratedString[]): GeneratedString {
    return bracket(elements.join(', '))
}

export function arrayLiteral (elements: Literal[]): GeneratedString {
    return bracket(elements.map(literal).join(', '))
}

export function objectLiteral (entries: ObjectLiteral): GeneratedString {
    const entry = (key: keyof ObjectLiteral, value: Literal): GeneratedString => `${ key }: ${ literal(value) },`
    return brace(mapEntries(entries, entry).join('\n'))
}

/** Comment generation */

export function headerComment (content: string): GeneratedString {
    const asterisks = new Array(content.length + 6).fill('*').join('')
    return [
        `/${ asterisks }/`,
        `/** ${ content } **/`,
        `/${ asterisks }/`,
        ''
    ].join('\n')
}

export function subheaderComment (content: string | string[]): GeneratedString {
    return [
        `/**`,
        ` * ${ joined(content, '\n * ') }`,
        ` */`
    ].join('\n')
}


/** Utils */

export function quote (str: string): GeneratedString {
    return `'${ str }'`
}

export function bracket (str: string): GeneratedString {
    return `[${ str }]`
}

export function brace (str: string): GeneratedString {
    return `{\n${ str }\n}`
}

export function paren (str: string): GeneratedString {
    return `(${ str })`
}

/** Keywords */
namespace kw {

    export function exported (line: string, config: Config = {}): string {
        return config.export ? `export ${ line }` : line
    }

    export function async (func: string, config: FunctionConfig): string {
        return config.async ? `async ${ func }` : func
    }
}

function fmt<F = string> (str?: F, formatter: Formatter<F> = s => s): F | string {
    if (str) return formatter(str)
    else return ''
}

function joined (str: string | string[], sep = ', ', formatter: Formatter<string> = s => s): string {
    return formatter((Array.isArray(str) ? str : [str]).join(sep))
}

export function reindent (str: string): string {
    const completed: string[] = []
    const lines = str.split('\n')
    const bracketStack = []
    let level = 0
    for (const line of lines) {
        const lineBracketStack = []
        for (let i = 0; i < line.length; i++) {
            const char = line[i]
            if (['(', '[', '{'].includes(char)) {
                lineBracketStack.push(char)
                bracketStack.push(char)
            } else if ([')', ']', '}'].includes(char)) {
                lineBracketStack.pop()
                bracketStack.pop()
            }
        }
        const level = lineBracketStack.length ? (bracketStack.length - 1) : bracketStack.length
        completed.push(`${ new Array(level).fill('    ').join('') }${ line.trim() }`)
    }
    return completed.join('\n')
}
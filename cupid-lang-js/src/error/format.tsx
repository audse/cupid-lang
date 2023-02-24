import { DecimalNode, IdentNode, IntNode, Node, nodeIs, StringNode, token, token as tokens } from '@/types'
import { Kind, TypeKind, Variable } from '@/ast'
import { reindent } from '@/codegen'
import { Expr, Type, Field } from '@/passes/@types/1-pre-create-scope'
import { safeStringify, filterObjectRecursive } from '@/utils'
import React from 'react'
import { AnyExpr, AnyField, AnyType, Err, Step } from './index'

type Formatter<T> = {
    fmt: (files: string[], context: T extends Kind ? AnyExpr<T> : T, env: Node[]) => string
    fmtJsx: (files: string[], context: T extends Kind ? AnyExpr<T> : T, env: Node[]) => React.ReactNode
}

export function format (files: string[], error: Err, env: Node[]): string {
    const body = Array.isArray(error.expr) ? error.expr.map(step => formatStep(files, step, env)) : expr.fmt(files, error.expr as any, env)
    return reindent(`
        ${ error.code } - ${ error.context }
        ${ body }
    `)
}

function formatStep (files: string[], step: Step, env: Node[]): string {
    return reindent(`
        ${ step.context }
        ${ expr.fmt(files, step.expr as any, env) }
    `)
}

export function FormatJsx ({ files, error, env }: { files: string[], error: Err, env: Node[] }): JSX.Element {
    return <>
        <div style={ { color: 'var(--error)' } }>
            <strong>{ error.code }</strong> { error.context && `- ${ error.context }` }
        </div>
        <div>
            { Array.isArray(error.expr)
                ? error.expr.map((step, i) => <FormatStepJsx key={ i } step={ step } env={ env } files={ files } />)
                : expr.fmtJsx(files, error.expr as any, env) }
        </div>
    </>
}

export function FormatStepJsx ({ files, step, env }: { files: string[], step: Step, env: Node[] }): JSX.Element {
    return <>
        <div style={ { color: 'var(--error-400)', marginTop: '0.5rem' } }>
            { step.context }
        </div>
        <div>
            { expr.fmtJsx(files, step.expr as any, env) }
        </div>
    </>
}

export function formatType (type: AnyType): string {
    switch (type.typeKind) {
        case TypeKind.Primitive: return type.name
        case TypeKind.Instance: return type.ident.name + (type.args.length ? ` [${ type.args.map(formatType).join(', ') }]` : '')
        case TypeKind.Map: return `map [${ formatType(type.keys) }, ${ formatType(type.values) }]`
        case TypeKind.Unknown: return `unknown`
        case TypeKind.Variable: return type.name
        case TypeKind.Struct: return `struct [\n${ type.fields.map(formatField).join(',\n') }\n]`
        case TypeKind.Sum: return `struct [\n${ type.fields.map(formatField).join(',\n') }\n]`
        case TypeKind.Fun: return `(${ type.params.map(formatField).join(',') })` + type.returns ? ` -> ${ formatType(type.returns as AnyType) }` : ''
    }
}

export function formatField (field: AnyField): string {
    return `${ field.ident.name } : ${ formatType(field.type) }`
}

function stringifyFiltered (obj: any) {
    return safeStringify(filterObjectRecursive(obj, key => !['scope', 'token', 'tokens'].includes(key.toString())))
}

function tabsToSpaces (line: string): string {
    return line.replace(/\t/g, ' ')
}

function getFormatter (exp: Omit<Expr, 'scope'>): Formatter<Kind> {
    switch (exp.kind) {
        case Kind.Decl: return declare as Formatter<Kind>
        case Kind.Ident: return ident as Formatter<Kind>
        case Kind.Type: return type as Formatter<Kind>
        case Kind.Literal: return literal as Formatter<Kind>
        default: return {
            fmt: (files, exp, env) => safeStringify(filterObjectRecursive(exp, key => !['scope'].includes(key.toString()))),
            fmtJsx: (files, exp, env) => safeStringify(filterObjectRecursive(exp, key => !['scope'].includes(key.toString())))
        } as Formatter<Kind>
    }
}

const expr: Formatter<Kind> = {
    fmt (files, exp, env) {
        return getFormatter(exp).fmt(files, exp, env)
    },
    fmtJsx (files, exp, env) {
        return getFormatter(exp).fmtJsx(files, exp, env)
    }
}

const declare: Formatter<Kind.Decl> = {
    fmt (files, decl, env) {
        return ident.fmt(files, decl.ident, env)
    },
    fmtJsx (files, decl, env) {
        return ident.fmtJsx(files, decl.ident, env)
    }
}

const type: Formatter<Kind.Type> = {
    fmt (files, type, env) {
        switch (type.typeKind) {
            case TypeKind.Primitive: return type.name
            case TypeKind.Instance: return ident.fmt(files, type.ident, env)
            default: return stringifyFiltered(type)
        }
    },
    fmtJsx (files, type, env) {
        switch (type.typeKind) {
            case TypeKind.Primitive: return type.name
            case TypeKind.Instance: return ident.fmtJsx(files, type.ident, env)
            default: return stringifyFiltered(type)
        }
    },
}

function getFirstToken (node: Node): token.Token | null {
    if (!node) return null
    if (nodeIs.RuleNode(node)) return getFirstToken(node.items[0])
    if ('token' in node) return node.token
    return null
}

const ident: Formatter<Kind.Ident> = {
    fmt (files, ident, env) {
        const node = env[ident.source - 1]
        const token = getFirstToken(node)
        if (!token) return badNode(node, ident.source)
        return formatToken(files, token)
    },
    fmtJsx (files, ident, env) {
        const node = env[ident.source - 1]
        const token = getFirstToken(node)
        if (!token) return badNode(node, ident.source)
        return formatTokenJsx(files, token)
    }
}

function badNode (node: Node, source: number) {
    return `<unable to find source for node ${ source } ${ node && stringifyFiltered(node) }>`
}

const literal: Formatter<Kind.Literal> = {
    fmt (files, value, env) {
        const node = env[value.source - 1]
        if (!node || !('token' in node)) return badNode(node, value.source)
        return formatToken(files, (node as StringNode | IntNode | DecimalNode).token)
    },
    fmtJsx (files, value, env) {
        const node = env[value.source - 1]
        if (!node || !('token' in node)) return badNode(node, value.source)
        return formatTokenJsx(files, (node as StringNode | IntNode | DecimalNode).token)
    }
}

export function formatToken (files: string[], token: tokens.Token): string {
    const lines = files[token.file].split('\n')
    const underlineArgs = make.underlineTokenArgs(files, token)
    const formattedLines = underlineArgs.map(({ line, start, end, lineNumber }) => make.underlinedLine(line, start, end, lineNumber, lines.length))
    const prefixLine = (line: number) => `${ make.linePrefix(line, lines.length) }${ lines[line] }`
    return [
        ...(token.start.line > 1 ? [prefixLine(token.start.line - 2)] : []),
        ...(token.start.line > 0 ? [prefixLine(token.start.line - 1)] : []),
        ...formattedLines,
        ...(token.end.line < lines.length ? [prefixLine(token.end.line + 1)] : []),
        ...(token.end.line < lines.length - 1 ? [prefixLine(token.end.line + 2)] : []),
    ].join('\n')
}

export function formatTokenJsx (files: string[], token: tokens.Token): React.ReactNode {
    const lines = files[token.file].split('\n')
    const underlineArgs = make.underlineTokenArgs(files, token)
    return (
        <jsx.withContext lines={ lines } startLine={ token.start.line } endLine={ token.end.line }>
            { underlineArgs.map((args, i) => (
                <jsx.underlinedLine key={ i }
                    color={ 'red' }
                    start={ args.start }
                    end={ args.end }
                    line={ args.line }
                    lineNumber={ args.lineNumber }
                    totalLines={ args.totalLines } />
            )) }
        </jsx.withContext>
    )
}

interface UnderlineTokenProps {
    line: string
    start: number
    end: number
    lineNumber: number
    totalLines: number
}

namespace make {

    export function underline (len: number): string {
        return new Array(len).fill('^').join('') + '\n'
    }

    export function underlinedLine (line: string, start: number, end: number, lineNumber: number, totalLines: number): string {
        return [
            `${ linePrefix(lineNumber, totalLines) }${ line }`,
            underline(end - start).padStart(start + (end - start) + (totalLines.toString().length + 3), ' ')
        ].join('\n')
    }

    export function underlineTokenArgs (files: string[], token: tokens.Token): UnderlineTokenProps[] {
        const file = files[token.file]
        const lines = file.split(/\n/).map(tabsToSpaces)
        const lineNums = new Array((token.start.line - token.end.line) + 1)
            .fill(null)
            .map((_, i) => token.start.line + i)
        const cols: [number, number][] = [...lineNums].map((lineNum, i) => [0, lines[lineNum].length])
        const startLineIndex = lineNums.findIndex(num => num === token.start.line)
        const endLineIndex = lineNums.findIndex(num => num === token.end.line)
        cols[startLineIndex][0] = token.start.column
        cols[endLineIndex][1] = token.end.column + 1
        return lineNums.map((num, i) => ({ line: lines[num], start: cols[i][0], end: cols[i][1], lineNumber: num, totalLines: lines.length }))
    }

    export function linePrefix (lineNumber: number, totalLines: number) {
        const lineString = lineNumber.toString().padStart(totalLines.toString().length, ' ')
        return `${ lineString } | `
    }

}

namespace jsx {

    export function underlinedLine (props: UnderlineTokenProps & { color: React.CSSProperties['color'] }): JSX.Element {
        return <>
            <span style={ { display: 'block' } }>
                { linePrefix(props.lineNumber, props.totalLines) }{ props.line }
            </span>
            <span style={ { display: 'block', color: props.color, lineHeight: '0.25rem' } }>
                { new Array(props.totalLines.toString().length + 3).fill(' ').join('') }{ make.underline(props.end - props.start).padStart(props.start + (props.end - props.start), ' ') }
            </span>
        </>
    }

    export function withContext ({ lines, startLine, endLine, children }: { lines: string[], startLine: number, endLine: number, children: React.ReactNode }) {
        const style = { display: 'block', opacity: 0.5 }
        return <>
            { startLine > 1 && <span style={ style }>{ linePrefix(startLine - 2, lines.length) }{ lines[startLine - 2] }&nbsp;</span> }
            { startLine > 0 && <span style={ style }>{ linePrefix(startLine - 1, lines.length) }{ lines[startLine - 1] }&nbsp;</span> }
            { children }
            { endLine < lines.length - 1 && <span style={ style }>{ linePrefix(endLine + 1, lines.length) }{ lines[endLine + 1] }&nbsp;</span> }
            { startLine < lines.length - 2 && <span style={ style }>{ linePrefix(endLine + 2, lines.length) }{ lines[startLine + 2] }&nbsp;</span> }
        </>
    }

    export function linePrefix (lineNumber: number, totalLines: number) {
        const lineString = lineNumber.toString().padStart(totalLines.toString().length, ' ')
        return <span style={ { opacity: 0.5 } }>{ lineString } | </span>
    }

}
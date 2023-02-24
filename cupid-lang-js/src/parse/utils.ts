import { Option, Node, token } from '@/types'
import { TokenParser } from '@/parse/parse'

type NodeArray = (Node | boolean | (Node | boolean)[])[]

export function getNodeArray (nodes: NodeArray | Node | boolean): Node[] {
    if (typeof nodes === 'boolean') return []
    if (Array.isArray(nodes)) return (nodes as NodeArray)
        .flat()
        .filter(node => typeof node !== 'boolean') as Node[]
    return [nodes]
}

export namespace modifier {

    export function multiple<T> (parser: TokenParser, func: (parser: TokenParser) => Option<T>): T[] {
        const nodes: T[] = []
        while (true) {
            const node = func(parser)
            if (node !== null) nodes.push(node)
            else break
        }
        return nodes
    }

    export function atLeastOne<T> (parser: TokenParser, func: (parser: TokenParser) => Option<T>): Option<T[]> {
        const items = multiple<T>(parser, func)
        if (items.length) return items
        return null
    }

    export function optional<T> (parser: TokenParser, func: (parser: TokenParser) => Option<T>): Option<T | boolean> {
        const item = func(parser)
        if (item !== null) return item
        else return false
    }

    export function negative<T> (parser: TokenParser, func: (parser: TokenParser) => Option<T>): Option<T | boolean> {
        const item = func(parser)
        if (item === null) return false
        return null
    }

}

export namespace node {
    export function string (token: Option<token.Token>): Option<Node> {
        if (token) return { token, string: token.content }
        return null
    }

    export function int (token: Option<token.Token>): Option<Node> {
        if (token) return { token, int: parseInt(token.content) }
        return null
    }

    export function decimal (token: Option<token.Token>): Option<Node> {
        if (token) return { token, decimal: token.content.split('.').map(v => parseInt(v)) as [number, number] }
        return null
    }

    export function ident (token: Option<token.Token>): Option<Node> {
        if (token) return { token, ident: token.content }
        return null
    }
}
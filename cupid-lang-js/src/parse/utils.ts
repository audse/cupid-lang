import { Option, Node, token, CustomNode, nodeIs, RuleNode } from '@/types'
import { TokenParser } from '@/parse/parse'

type NodeArray = (Node | boolean | (Node | boolean)[])[]

export function getNodeArray (nodes: NodeArray | Node | boolean | null): Option<Node[]> {
    if (nodes === null) return null
    if (typeof nodes === 'boolean') return []
    if (Array.isArray(nodes)) return (nodes as NodeArray)
        .flat()
        .filter(node => typeof node !== 'boolean') as Node[]
    return [nodes]
}

export function makeNode (name: string, items: NodeArray | Node | boolean | null): Option<RuleNode> {
    if (items === null) return null
    return { name, items: getNodeArray(items) || [] }
}

type ParseFunc<T> = (parser: TokenParser) => T

export namespace modifier {

    export function passThrough<T> (func: ParseFunc<Option<T>>): ParseFunc<Option<boolean>> {
        return parser => func(parser) ? true : null
    }

    export function multiple<T> (func: ParseFunc<Option<T>>): ParseFunc<T[]> {
        return parser => {
            const nodes: T[] = []
            while (true) {
                const node = func(parser)
                if (node !== null) nodes.push(node)
                else break
            }
            return nodes
        }
    }

    export function atLeastOne<T> (func: ParseFunc<Option<T>>): ParseFunc<Option<T[]>> {
        return parser => {
            const items = multiple<T>(func)(parser)
            if (items.length) return items
            return null
        }
    }

    export function optional<T> (func: ParseFunc<Option<T>>): ParseFunc<Option<T | boolean>> {
        return parser => {
            const item = func(parser)
            if (item !== null) return item
            else return false
        }
    }

    export function negative<T> (func: ParseFunc<Option<T>>): ParseFunc<Option<T | boolean>> {
        return parser => {
            const item = func(parser)
            if (item === null) return false
            return null
        }
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
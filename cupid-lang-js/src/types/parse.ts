import { TokenParser } from '@/parse/parse'
import { token, Option } from '@/types'

export type Node = StringNode | IntNode | DecimalNode | IdentNode | RuleNode

export type NodeParser = (parser: TokenParser) => Option<any>

type BaseNode<SingleToken extends boolean> = (SingleToken extends true
    ? { token: token.Token }
    : { tokens: token.Token[] })

export type StringNode = BaseNode<true> & { string: string }
export type IntNode = BaseNode<true> & { int: number }
export type DecimalNode = BaseNode<true> & { decimal: [number, number] }
export type IdentNode = BaseNode<true> & { ident: string }
export type RuleNode = {
    name: string,
    items: Node[]
}

export namespace nodeIs {
    export function type (node: Node): string {
        if (RuleNode(node)) return 'RuleNode'
        if (IdentNode(node)) return 'IdentNode'
        if (StringNode(node)) return 'StringNode'
        if (DecimalNode(node)) return 'DecimalNode'
        if (IntNode(node)) return 'IntNode'
        return 'Unknown type'
    }
    export function RuleNode (node: Node): node is RuleNode {
        return node && 'name' in node
    }
    export function IdentNode (node: Node): node is IdentNode {
        return node && 'ident' in node
    }
    export function StringNode (node: Node): node is StringNode {
        return node && 'string' in node
    }
    export function DecimalNode (node: Node): node is DecimalNode {
        return node && 'decimal' in node
    }
    export function IntNode (node: Node): node is IntNode {
        return node && 'int' in node
    }
}
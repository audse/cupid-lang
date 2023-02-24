import { TokenParser } from '@/parse/parse'
import { Tokenizer } from '@/tokenize'
import { token, Node } from '@/types'
import { cupid as parser } from '@/parse/cupid.parser'
import { toTree, createScope, defineSymbols, resolveTypes, inferTypes, checkTypes, interpret } from '../'
import { Expr } from '../@types/1-pre-create-scope'
import { Scope } from '@/scope'
import { Err } from '@/error'

function compile (exprs: Expr[], scope: Scope) {
    console.time('compiling...')
    // console.log(...exprs)
    try {
        const scoped = exprs.map(expr => createScope(expr, scope))
        const defined = scoped.map(defineSymbols)
        const resolved = defined.map(resolveTypes)
        const inferred = resolved.map(inferTypes)
        const checked = inferred.map(checkTypes)
        const interpreted = checked.map(interpret)
        // console.log(...interpreted)
        console.timeEnd('compiling...')
        return interpreted
    } catch (error) {
        console.timeEnd('compiling...')
        throw error
    }
}

function tokenize (content: string) {
    console.time('tokenizing...')
    const tokenizer = new Tokenizer(0, content)
    const tokens = tokenizer.tokenize().filter(tkn => tkn.type !== token.Type.Comment)
    console.timeEnd('tokenizing...')
    return tokens
}

function parse (tokens: token.Token[]): Node[] {
    console.time('parsing...')
    const cupidParser = new TokenParser(tokens)
    const tree = []
    while (cupidParser.peek()) {
        const exp = parser.expr(cupidParser)
        if (exp) tree.push(exp)
        else break
    }
    console.timeEnd('parsing...')
    return tree.flat()
}

export function test (content: string): { results: any, tree: Expr[], env: Node[] } {
    const scope = new Scope()
    const env: Node[] = []

    const tokens = tokenize(content)
    const nodes = parse(tokens)
    const exprs = nodes.map(node => toTree(node, env))

    try {
        const results = compile(exprs, scope)
        return { results, tree: exprs, env }
    } catch (error) {
        throw { tree: exprs, env, error: error as Err }
    }
}
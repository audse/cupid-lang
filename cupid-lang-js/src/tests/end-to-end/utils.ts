import { reindent } from '@/codegen'
import { intoAst } from '@/into-ast'
import { cupid } from '@/parse/cupid.parser'
import { TokenParser } from '@/parse/parse'
import { Tokenizer } from '@/tokenize'
import { token } from '@/types/tokenize'

export function setup (content: string) {

    const fullContent = reindent(`
        type int = primitive int
        type none = primitive none
        type decimal = primitive decimal
        type bool = primitive bool
        type str = primitive str
        type env = primitive env
        type type = primitive type
        ${ content }
    `.trim())

    const { scope, source, into } = intoAst()

    const tokens = tokenize(fullContent)
    const nodes = parse(tokens)
    const exprs = nodes?.map(expr => into(expr)) || []

    return { scope, source, tokens, exprs, content: fullContent }
}

export function tokenize (content: string) {
    const tokenizer = new Tokenizer(0, content)
    return tokenizer.tokenize()
}

export function parse (tokens: token.Token[]) {
    const parser = new TokenParser(tokens)
    const exprs = []
    while (parser.peek()) {
        const expr = cupid.expr(parser)
        if (expr) exprs.push(...expr)
        else {
            console.error({
                error: `unable to parse token`,
                token: parser.current()
            })
            throw 'unable to parse token'
        }
    }
    return exprs
}
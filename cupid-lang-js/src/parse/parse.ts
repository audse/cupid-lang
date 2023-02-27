import { Option, token as tokens, grmmr } from '@/types'
import { modifier as mod } from './utils'

type Pair = typeof pairs[keyof typeof pairs]

enum Pairs {
    Paren = 'paren',
    Bracket = 'bracket',
    Brace = 'brace',
    Angle = 'angle'
}

const pairs = {
    [Pairs.Paren]: ['(', ')'],
    [Pairs.Bracket]: ['[', ']'],
    [Pairs.Brace]: ['{', '}'],
    [Pairs.Angle]: ['<', '>']
} as const

const str = (data: any) => `${ JSON.stringify(data, null, 2) }`

export class TokenIterator {

    tokens: tokens.Token[] = []
    index = -1

    debug: boolean
    traceback: string[] = []

    constructor (t: tokens.Token[], debug = false) {
        this.tokens = t
        this.debug = debug
    }

    consume = (): Option<tokens.Token> => {
        this.trace(() => `Consuming token:\n${ str(this.current()) }`)

        this.index += 1
        return this.current()
    }

    current = (): Option<tokens.Token> => this.peek(0)

    peek = (amount: number = 1): Option<tokens.Token> => this.tokens[this.index + amount] || null

    mark = (): number => this.index

    reset = (to: number) => this.index = to

    try<T> (parser: (t: this) => Option<T>): Option<T> {
        const mark = this.mark()

        const result = parser(this)
        if (result) return result

        this.reset(mark)
        return null
    }

    trace (string: () => string) {
        if (this.debug) this.traceback.push(string())
    }
}

export class TokenParser extends TokenIterator {

    token = (validate: (t: tokens.Token) => boolean): Option<tokens.Token> => this.try<tokens.Token>(tokens => {
        const tkn = tokens.consume()
        if (tkn && validate(tkn)) return tkn
        return null
    })

    match = (str: string): Option<tokens.Token> => this.token(token => token.content === str)
    matchOne = (...strs: string[]): Option<tokens.Token> => this.token(token => strs.includes(token.content))
    matchOneSet = (set: Set<string>): Option<tokens.Token> => this.token(token => set.has(token.content))
    type = (type: tokens.Type): Option<tokens.Token> => this.token(token => token.type === type)
    string = (): Option<tokens.Token> => this.type(tokens.Type.Str)
    number = (): Option<tokens.Token> => this.type(tokens.Type.Number)
    int = (): Option<tokens.Token> => this.token(token => token.type === tokens.Type.Number && !token.content.includes('.'))
    decimal = (): Option<tokens.Token> => this.token(token => token.type === tokens.Type.Number && token.content.includes('.'))
    ident = (): Option<tokens.Token> => this.type(tokens.Type.Ident)
    symbol = (symbol: string): Option<tokens.Token> => this.token(token => token.type === tokens.Type.Symbol && token.content === symbol)
    any = () => this.token(() => true)

    chain<T extends any[]> (...inner: { [N in keyof T]: (parser: this) => Option<T[N]> }): Option<T> {
        return this.try<T>(parser => {
            const result = []
            for (const item of inner) {
                const itemResult = item(parser)
                if (itemResult !== null) result.push(itemResult)
                else return null
            }
            return result as T
        })
    }

    loop<T> (inner: (parser: this) => Option<T>): T[] {
        const results: T[] = []
        while (this.peek()) {
            const result = this.try(inner)
            if (result) results.push(result)
            else break
        }
        return results
    }

    list<T> (inner: (parser: this) => Option<T>, seperator: Option<string> = ','): T[] {
        return this.loop<T>(parser => {
            const result = parser.try(inner)
            if (!result) return null
            if (seperator) parser.match(seperator)
            return result
        })
    }

    #group<T> (inner: (parser: this) => Option<T>, brackets: Pair): Option<T> {
        const [open, close] = brackets
        const results = this.chain(
            parser => parser.match(open),
            parser => parser.try(inner),
            parser => parser.match(close)
        )
        if (!results) return null
        const [_, result, __] = results
        return result
    }

    #groupList<T> (inner: (parser: this) => Option<T>, seperator: Option<string>, brackets: Pair): Option<T[]> {
        return this.#group<T[]>(parser => parser.list<T>(inner, seperator), brackets)
    }

    group = <T> (inner: (parser: this) => Option<T>): { [K in Pairs]: () => Option<T> } => ({
        paren: () => this.#group(inner, pairs.paren),
        bracket: () => this.#group(inner, pairs.bracket),
        brace: () => this.#group(inner, pairs.brace),
        angle: () => this.#group(inner, pairs.angle),
    })

    groupList = <T> (inner: (parser: this) => Option<T>, seperator: Option<string>): { [K in Pairs]: () => Option<T[]> } => ({
        paren: () => this.#groupList(inner, seperator, pairs.paren),
        bracket: () => this.#groupList(inner, seperator, pairs.bracket),
        brace: () => this.#groupList(inner, seperator, pairs.brace),
        angle: () => this.#groupList(inner, seperator, pairs.angle),
    })
}


export namespace grammar {

    export function parse (parser: TokenParser): Option<grmmr.Rule[]> {
        const rules: grmmr.Rule[] = []
        while (parser.peek()) {
            const result = rule(parser)
            if (result) rules.push(result)
            else throw `Could not parse token:\n${ JSON.stringify(parser.peek(), null, 2) }`
        }
        return rules
    }

    function rule (parser: TokenParser): Option<grmmr.Rule> {
        const results = parser.chain(
            parser => parser.ident(),
            mod.optional<string[]>(parser => parser.groupList(ruleParam, ',').bracket()),
            parser => ruleModifier(parser) || false,
            parser => parser.groupList(group, '|').brace()
        )
        if (!results) return null
        const [name, params, modifier, alts] = results
        return {
            name: name.content,
            params: Array.isArray(params) ? params : [],
            alts,
            modifier: modifier || null
        }
    }

    function ruleParam (parser: TokenParser): Option<string> {
        const result = parser.ident()
        if (result) return result.content
        return null
    }

    function ruleModifier (parser: TokenParser): Option<grmmr.RuleModifier> {
        const results = parser.matchOne(...Object.values(grmmr.RuleModifier))
        if (results) return results.content as grmmr.RuleModifier
        return null
    }

    function group (parser: TokenParser): Option<grmmr.Group> {
        const results = parser.groupList(item, null).paren()
        if (results) return {
            items: results,
            modifier: modifier(parser)
        }
        const itemsList = items(parser)
        if (itemsList) return { items: itemsList, modifier: null }
        return null
    }

    function items (parser: TokenParser): Option<grmmr.Item[]> {
        const content = parser.loop(parser => item(parser))
        if (content.length) return content
        return null
    }

    function item (parser: TokenParser): Option<grmmr.Item> {
        const token = parser.ident() || parser.string()
        const args = mod.optional(parser => parser.groupList(item, ',').bracket())(parser)
        if (token) return {
            content: token.content,
            modifier: modifier(parser),
            args: Array.isArray(args) ? args : []
        }
        return null
    }

    function modifier (parser: TokenParser): Option<grmmr.Modifier> {
        const mod = parser.matchOne(...Object.values(grmmr.Modifier))
        if (mod) return mod.content as grmmr.Modifier
        return null
    }
}
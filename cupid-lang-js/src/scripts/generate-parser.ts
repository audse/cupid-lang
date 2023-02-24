import { Tokenizer } from '@/tokenize'
import { generator } from '../parse/generator'
import { grammar, TokenParser } from '../parse/parse'
import { write, file } from 'bun'
import { resolve } from 'path'

import cupidGrammar from '../parse/cupid.grammar'

const output = resolve('./src/parse/cupid.parser.ts')

const tokenizer = new Tokenizer(0, cupidGrammar)
const parser = new TokenParser(tokenizer.tokenize(), true)

try {
    const rules = grammar.parse(parser)
    if (rules) {
        const generatedParser = generator.generate('cupid', rules)
        console.log('Writing file...', file(output))
        await write(
            output,
            generatedParser,
        )
        console.info('Successfully wrote file!')
    }
    else throw `No rules found`
} catch (err) {
    console.warn(err)
    console.log(...parser.traceback.slice(parser.traceback.length - 5))
}
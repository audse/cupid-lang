import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Ident, Literal } from '@/ast'
import { setup } from './utils'

describe('assign end-to-end', () => {

    test('int', () => {
        const { exprs } = setup(`
            let mut int x = 1
            x = 2
            x
        `)
        const ident = last(exprs)
        interpret(...exprs)
        const symbol = ident instanceof Ident ? ident.expectSymbol() : null
        expect(
            symbol
            && symbol.value instanceof Literal
            && symbol.value.value === 2
        ).toBeTruthy()
    })

    test('immutable', () => {
        const { exprs } = setup(`
            let int x = 1
            x = 2
        `)
        expectCompilationError(
            CompilationErrorCode.Immutable,
            () => interpret(...exprs)
        )
    })

})
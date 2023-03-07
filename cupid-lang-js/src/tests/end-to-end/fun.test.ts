import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Ident, Literal } from '@/ast'
import { setup } from './utils'

describe('assign end-to-end', () => {

    test('add', () => {
        const { exprs } = setup(`
            let add = int a, int b -> int => a + b
            add (1, 2)
        `)
        expect(last(interpret(...exprs))).toBe(3)
    })

    test('nested add', () => {
        const { exprs } = setup(`
            let add = int a, int b -> int => a + b
            add (1, add (2, 3))
        `)
        expect(last(interpret(...exprs))).toBe(6)
    })

    test('wrong argument type', () => {
        const { exprs } = setup(`
            let add = int a, int b -> int => a + b
            add (1.5, 2)
        `)
        expectCompilationError(
            CompilationErrorCode.UnableToUnifyType,
            () => interpret(...exprs)
        )
    })

    test('wrong number of arguments', () => {
        const { exprs } = setup(`
            let add = int a, int b -> int => a + b
            add (1, 2, 3)
        `)
        expectCompilationError(
            CompilationErrorCode.IncorrectNumberOfArgs,
            () => interpret(...exprs)
        )
    })

    test('wrong return type', () => {
        const { exprs } = setup(`
            let some-fun = () -> int => true
            some-fun ()
        `)
        expectCompilationError(
            CompilationErrorCode.UnableToUnifyType,
            () => interpret(...exprs)
        )
    })

})
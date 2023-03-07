import { describe, expect, test } from 'bun:test'
import { compile, expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Fun, Ident, Impl, InstanceType, Literal, PrimitiveType } from '@/ast'
import { setup } from './utils'

describe('impl end-to-end', () => {

    test('int', () => {
        const { exprs } = setup(`
            impl int = [
                add : int a, int b -> int => a + b
            ]
            int\\add
        `)
        const result = last(interpret(...exprs))
        expect(
            result instanceof Fun
            && result.returns.getResolved() instanceof PrimitiveType
        ).toBeTruthy()
    })

    test('int explicit call', () => {
        const { exprs } = setup(`
            impl int = [
                add : int a, int b -> int => a + b
            ]
            int\\add(1, 2)
        `)
        expect(last(interpret(...exprs))).toBe(3)
    })

    test('int explicit call wrong number of args', () => {
        const { exprs } = setup(`
            impl int = [
                add : int a, int b -> int => a + b
            ]
            int\\add(1)
        `)
        expectCompilationError(
            CompilationErrorCode.IncorrectNumberOfArgs,
            () => interpret(...exprs)
        )
    })

    test('int implicit call', () => {
        const { exprs } = setup(`
            impl int = [
                add : int a, int b -> int => a + b
            ]
            100\\add(1, 2)
        `)
        expect(last(interpret(...exprs))).toBe(3)
    })

    test('int implicit call with self argument', () => {
        const { exprs } = setup(`
            impl int = [
                add : self, int other -> int => self + other
            ]
            1\\add(2)
        `)
        expect(last(interpret(...exprs))).toBe(3)
    })

})
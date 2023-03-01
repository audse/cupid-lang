import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Ident, Literal, PrimitiveType, StructType, Type } from '@/ast'
import { setup } from './utils'

describe('instance type end-to-end', () => {

    test('generic struct', () => {
        const { exprs } = setup(`
            type point = t => struct [
                x : t
                y : t
            ]
            type int-point = point [int]
            int-point
        `)
        const result = last(interpret(...exprs))
        const resultType = result instanceof Type ? result : null
        expect(
            resultType
            && resultType instanceof StructType
            && resultType.fields.length
            && resultType.fields[0].type instanceof PrimitiveType
            && resultType.fields[0].type.name === 'int'
        ).toBeTruthy()
    })

    test('undefined', () => {
        const { exprs } = setup(`
            type point = struct [
                x : t
                y : t
            ]
            point
        `)
        expectCompilationError(
            CompilationErrorCode.NotDefined,
            () => interpret(...exprs)
        )
    })

    test('not a type', () => {
        const { exprs } = setup(`
            let some-type = 123
            type point = t => struct [
                x : t
                y : t
            ]
            point [some-type]
        `)
        expectCompilationError(
            CompilationErrorCode.NotAType,
            () => interpret(...exprs)
        )
    })

    test('wrong number of arguments', () => {
        const { exprs } = setup(`
            type point = t => struct [
                x : t
                y : t
            ]
            type int-point = point [int, decimal]
            int-point
        `)
        expectCompilationError(
            CompilationErrorCode.IncorrectNumberOfArgs,
            () => interpret(...exprs)
        )
    })

})
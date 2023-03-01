import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Decl, Literal, PrimitiveType } from '@/ast'
import { setup } from './utils'

describe('decl end-to-end', () => {

    test('int', () => {
        const { exprs } = setup(`let x : int = 1`)
        const decl = last(exprs)
        interpret(...exprs)
        expect(
            decl instanceof Decl
            && decl.type.getResolved() instanceof PrimitiveType
            && decl.value instanceof Literal
            && decl.value.value === 1
        ).toBeTruthy()
    })

    test('type mismatch', () => {
        const { exprs } = setup(`let x : bool = 1`)
        expectCompilationError(
            CompilationErrorCode.UnableToUnifyType,
            () => interpret(...exprs)
        )
    })

})
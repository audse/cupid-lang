import { CompilationErrorCode } from '@/error/compilation-error'
import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { setup } from './utils'

describe('branch end-to-end', () => {

    test('branch true', () => {
        const { exprs } = setup(`if true => 1 else => 2`)
        expect(last(interpret(...exprs))).toBe(1)
    })

    test('branch false', () => {
        const { exprs } = setup(`if false => 1 else => 2`)
        expect(last(interpret(...exprs))).toBe(2)
    })

    test('branch type mismatch', () => {
        const { exprs } = setup(`if true => 1 else => true`)
        expectCompilationError(
            CompilationErrorCode.UnableToUnifyType,
            () => interpret(...exprs)
        )
    })

    test('branch condition type mismatch', () => {
        const { exprs } = setup(`if 'some string' => 1 else => 2`)
        expectCompilationError(
            CompilationErrorCode.IncorrectType,
            () => interpret(...exprs)
        )
    })

})
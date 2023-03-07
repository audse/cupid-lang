import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Ident, Literal } from '@/ast'
import { setup } from './utils'

describe('ident end-to-end', () => {

    test('undefined', () => {
        const { exprs } = setup(`
            let int x = 1
            y
        `)
        expectCompilationError(
            CompilationErrorCode.NotDefined,
            () => interpret(...exprs)
        )
    })

})
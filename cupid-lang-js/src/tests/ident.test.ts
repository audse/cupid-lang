import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'

test('undefined', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(...exprs, make.ident('x'))
    )
})
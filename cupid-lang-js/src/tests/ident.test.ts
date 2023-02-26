import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'

test('undefined', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(make.ident('x'))
    )
})
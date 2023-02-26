import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, maker, setup } from './utils'

test('int decl', () => {
    const [scope, make] = setup()
    expect(interpret(
        make.quick.constructor.int(),
        make.quick.decl.int('x', 123),
        make.ident('x')
    )[2]).toBe(123)
})

test('already defined', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.AlreadyDefined,
        () => interpret(
            make.quick.constructor.int(),
            make.quick.decl.int('x', 123),
            make.quick.decl.dec('x', 1, 5),
        )
    )
})
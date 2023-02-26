import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, maker, last, setup } from './utils'

test('int decl', () => {
    const [scope, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.quick.decl.int('x', 123),
        make.ident('x')
    ))).toBe(123)
})

test('already defined', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.AlreadyDefined,
        () => interpret(
            ...exprs,
            make.quick.decl.int('x', 123),
            make.quick.decl.dec('x', 1, 5),
        )
    )
})
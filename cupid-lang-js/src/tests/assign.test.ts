import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'

test('int assign', () => {
    const [scope, make] = setup()
    expect(interpret(
        make.quick.constructor.int(),
        make.decl(make.ident('x'), make.int(1), undefined, true),
        make.assign(make.ident('x'), make.int(2)),
        make.ident('x')
    )[3]).toBe(2)
})

test('immutable int assign', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.Immutable,
        () => interpret(
            make.quick.constructor.int(),
            make.quick.decl.int('x', 1),
            make.assign(make.ident('x'), make.int(2)),
        )
    )
})
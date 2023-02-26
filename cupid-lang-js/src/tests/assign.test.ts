import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, last, setup } from './utils'

test('int assign', () => {
    const [scope, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.decl(make.ident('x'), make.literal.int(1), undefined, true),
        make.assign(make.ident('x'), make.literal.int(2)),
        make.ident('x')
    ))).toBe(2)
})

test('immutable int assign', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.Immutable,
        () => interpret(
            ...exprs,
            make.quick.decl.int('x', 1),
            make.assign(make.ident('x'), make.literal.int(2)),
        )
    )
})
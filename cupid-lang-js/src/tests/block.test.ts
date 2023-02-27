import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { interpret, maker, last, setup, expectCompilationError } from './utils'

test('block', () => {
    const [scope, make, exprs] = setup()
    const block = make.block(make.literal.int(123))
    expect(last(interpret(...exprs, block))).toBe(123)
})

test('mutation inside block', () => {
    const [scope, make, exprs] = setup()
    const decl = make.quick.decl.int('x', 1, true)
    const block = make.block(
        make.assign(make.ident('x'), make.literal.int(2))
    )
    expect(last(interpret(
        ...exprs,
        decl,
        block,
        make.ident('x'),
    ))).toBe(2)
})

test('mutation outside block', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(
            ...exprs,
            make.block(
                make.quick.decl.int('x', 1, true),
            ),
            make.assign(make.ident('x'), make.literal.int(2)),
            make.ident('x'),
        )
    )
})

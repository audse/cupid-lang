import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { interpret, maker, last, setup, expectCompilationError } from './utils'

test('branch true', () => {
    const [scope, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.branch(
            make.literal.bool(true),
            make.literal.int(10),
            make.literal.int(20)
        )
    ))).toBe(10)
})

test('branch false', () => {
    const [scope, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.branch(
            make.literal.bool(false),
            make.literal.int(10),
            make.literal.int(20)
        )
    ))).toBe(20)
})

test('branch condition not bool', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.IncorrectType,
        () => interpret(
            ...exprs,
            make.branch(
                make.literal.int(1),
                make.literal.int(10),
                make.literal.int(20)
            )
        )
    )
})

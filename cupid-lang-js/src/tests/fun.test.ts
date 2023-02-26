import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, maker, setup } from './utils'

test('fun decl', () => {
    const [_, make] = setup()
    const addFun = make.quick.decl.addFun()
    const addIdent = make.ident('add')
    interpret(
        make.quick.constructor.int(),
        addFun,
        addIdent
    )
    const symbol = addIdent.expectSymbol().value?.expectType().getResolved().report()
    expect(symbol).toBe('(a : int, b : int) -> int')
})

test('fun call', () => {
    const [_, make] = setup()
    expect(interpret(
        make.quick.constructor.int(),
        make.call(
            make.quick.decl.addFun().value,
            make.int(1),
            make.int(2)
        )
    )[1]).toBe(3)
})

test('nested fun call', () => {
    const [_, make] = setup()
    expect(interpret(
        make.quick.constructor.int(),
        make.quick.decl.addFun(),
        make.call(
            make.ident('add'),
            make.call(make.ident('add'), make.int(1), make.int(2)),
            make.int(2)
        )
    )[2]).toBe(5)
})

test('wrong argument type', () => {
    const [_, make] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            make.quick.constructor.int(),
            make.quick.decl.addFun(),
            make.call(make.ident('add'), make.int(1), make.dec(1, 5))
        )
    )
})

test('wrong number of arguments', () => {
    const [_, make] = setup()
    expectCompilationError(
        CompilationErrorCode.IncorrectNumberOfArgs,
        () => interpret(
            make.quick.constructor.int(),
            make.quick.decl.addFun(),
            make.call(make.ident('add'), make.int(1), make.int(2), make.int(3))
        )
    )
})
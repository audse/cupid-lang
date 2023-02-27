import { PrimitiveType } from '@/ast/index'
import { CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { compile, expectCompilationError, interpret, last, maker, setup } from './utils'

test('fun decl', () => {
    const [_, make, exprs] = setup()
    const addFun = make.quick.decl.addFun()
    const addIdent = make.ident('add')
    interpret(
        ...exprs,
        addFun,
        addIdent
    )
    const symbol = addIdent.expectSymbol().value?.expectType().getResolved().report()
    expect(symbol).toBe('(a : int, b : int) -> int')
})

test('fun call', () => {
    const [_, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.call(
            make.quick.decl.addFun().value,
            make.literal.int(1),
            make.literal.int(2)
        )
    ))).toBe(3)
})

test('nested fun call', () => {
    const [_, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,
        make.quick.decl.addFun(),
        make.call(
            make.ident('add'),
            make.call(make.ident('add'), make.literal.int(1), make.literal.int(2)),
            make.literal.int(2)
        )
    ))).toBe(5)
})

test('wrong argument type', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            ...exprs,
            make.quick.decl.addFun(),
            make.call(make.ident('add'), make.literal.int(1), make.literal.dec(1, 5))
        )
    )
})

test('wrong number of arguments', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.IncorrectNumberOfArgs,
        () => interpret(
            ...exprs,
            make.quick.decl.addFun(),
            make.call(make.ident('add'), make.literal.int(1), make.literal.int(2), make.literal.int(3))
        )
    )
})
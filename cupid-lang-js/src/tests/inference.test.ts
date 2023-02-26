import { Expr, PrimitiveType } from '@/ast/index'
import { expect, test } from 'bun:test'
import { compile, interpret, last, setup } from './utils'


function inferredTypeIsPrimitive (expr: Expr, name: string): boolean {
    return (
        expr.inferredType
        && expr.inferredType instanceof PrimitiveType
        && expr.inferredType.name === name
    ) ? true : false
}

test('int literal inference', () => {
    const [scope, make, exprs] = setup()
    const int = make.literal.int(123)
    expect(inferredTypeIsPrimitive(last(compile(...exprs, int)), 'int')).toBeTruthy()
})

test('bool literal inference', () => {
    const [scope, make, exprs] = setup()
    const bool = make.literal.bool(false)
    expect(inferredTypeIsPrimitive(last(compile(...exprs, bool)), 'bool')).toBeTruthy()
})

test('decimal literal inference', () => {
    const [scope, make, exprs] = setup()
    const dec = make.literal.dec(1, 5)
    expect(inferredTypeIsPrimitive(last(compile(...exprs, dec)), 'decimal')).toBeTruthy()
})

test('none literal inference', () => {
    const [scope, make, exprs] = setup()
    const none = make.literal.none()
    expect(inferredTypeIsPrimitive(last(compile(...exprs, none)), 'none')).toBeTruthy()
})

test('typekind inference', () => {
    const [scope, make, exprs] = setup()
    const intType = make.type.primitive('int')
    expect(inferredTypeIsPrimitive(last(compile(...exprs, intType)), 'type')).toBeTruthy()
})

test('empty block inference', () => {
    const [scope, make, exprs] = setup()
    const block = make.block()
    expect(inferredTypeIsPrimitive(last(compile(...exprs, block)), 'none')).toBeTruthy()
})

test('block inference', () => {
    const [scope, make, exprs] = setup()
    const block = make.block(
        make.literal.bool(true),
        make.literal.int(1)
    )
    expect(inferredTypeIsPrimitive(last(compile(...exprs, block)), 'int')).toBeTruthy()
})

test('binop math inference', () => {
    const [scope, make, exprs] = setup()
    const binop = make.binop(
        make.literal.int(2),
        make.literal.int(1),
        '+'
    )
    expect(inferredTypeIsPrimitive(last(compile(...exprs, binop)), 'int')).toBeTruthy()
})

test('binop compare inference', () => {
    const [scope, make, exprs] = setup()
    const binop = make.binop(
        make.literal.int(2),
        make.literal.int(1),
        'is'
    )
    expect(inferredTypeIsPrimitive(last(compile(...exprs, binop)), 'bool')).toBeTruthy()
})
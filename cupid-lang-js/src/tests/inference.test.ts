import { Expr, PrimitiveType } from '@/ast/index'
import { expect, test } from 'bun:test'
import { compile, interpret, setup } from './utils'


function inferredTypeIsPrimitive (expr: Expr, name: string): boolean {
    return (
        expr.inferredType
        && expr.inferredType instanceof PrimitiveType
        && expr.inferredType.name === name
    ) ? true : false
}

test('int literal inference', () => {
    const [scope, make] = setup()
    const int = make.int(123)
    expect(inferredTypeIsPrimitive(compile(int)[0], 'int')).toBeTruthy()
})

test('bool literal inference', () => {
    const [scope, make] = setup()
    const bool = make.bool(false)
    expect(inferredTypeIsPrimitive(compile(bool)[0], 'bool')).toBeTruthy()
})

test('decimal literal inference', () => {
    const [scope, make] = setup()
    const dec = make.dec(1, 5)
    expect(inferredTypeIsPrimitive(compile(dec)[0], 'decimal')).toBeTruthy()
})

test('none literal inference', () => {
    const [scope, make] = setup()
    const none = make.none()
    expect(inferredTypeIsPrimitive(compile(none)[0], 'none')).toBeTruthy()
})

test('typekind inference', () => {
    const [scope, make] = setup()
    const intType = make.primitiveType('int')
    expect(inferredTypeIsPrimitive(compile(intType)[0], 'type')).toBeTruthy()
})

test('empty block inference', () => {
    const [scope, make] = setup()
    const block = make.block()
    expect(inferredTypeIsPrimitive(compile(block)[0], 'none')).toBeTruthy()
})

test('block inference', () => {
    const [scope, make] = setup()
    const block = make.block(
        make.bool(true),
        make.int(1)
    )
    expect(inferredTypeIsPrimitive(compile(block)[0], 'int')).toBeTruthy()
})

test('binop math inference', () => {
    const [scope, make] = setup()
    const binop = make.binop(
        make.int(2),
        make.int(1),
        '+'
    )
    expect(inferredTypeIsPrimitive(compile(binop)[0], 'int')).toBeTruthy()
})

test('binop compare inference', () => {
    const [scope, make] = setup()
    const binop = make.binop(
        make.int(2),
        make.int(1),
        'is'
    )
    expect(inferredTypeIsPrimitive(compile(binop)[0], 'bool')).toBeTruthy()
})
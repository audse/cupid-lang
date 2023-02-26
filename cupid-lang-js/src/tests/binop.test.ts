import { LiteralValue } from '@/ast/literal'
import { expect, test } from 'bun:test'
import { interpret, last, maker, setup } from './utils'

function testBinopInt (props: { op: string, l: number, r: number, expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const binop = make.binop(make.literal.int(props.l), make.literal.int(props.r), props.op)
    expect(last(interpret(...exprs, binop))).toBe(props.expect)
}

function testBinopDecimal (props: { op: string, l: [number, number], r: [number, number], expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const binop = make.binop(make.literal.dec(...props.l), make.literal.dec(...props.r), props.op)
    expect(last(interpret(...exprs, binop))).toBe(props.expect)
}

function testBinopBool (props: { op: string, l: boolean, r: boolean, expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const binop = make.binop(make.literal.bool(props.l), make.literal.bool(props.r), props.op)
    expect(last(interpret(...exprs, binop))).toBe(props.expect)
}

test('add int binop', () => {
    testBinopInt({ op: '+', l: 1, r: 2, expect: 3 })
})

test('subtract int binop', () => {
    testBinopInt({ op: '-', l: 2, r: 1, expect: 1 })
})

test('divide int binop', () => {
    testBinopInt({ op: '/', l: 4, r: 2, expect: 2 })
})

test('multiply int binop', () => {
    testBinopInt({ op: '*', l: 2, r: 2, expect: 4 })
})

test('power int binop', () => {
    testBinopInt({ op: '^', l: 3, r: 2, expect: 9 })
})

test('equal int binop', () => {
    testBinopInt({ op: 'is', l: 1, r: 1, expect: true })
})

test('not equal int binop', () => {
    testBinopInt({ op: 'not', l: 2, r: 1, expect: true })
})

test('less than dec binop', () => {
    testBinopDecimal({ op: '<', l: [1, 5], r: [2, 0], expect: true })
})

test('greater than dec binop', () => {
    testBinopDecimal({ op: '>', l: [1, 5], r: [2, 0], expect: false })
})

test('and binop', () => {
    testBinopBool({ op: 'and', l: true, r: false, expect: false })
})

test('or binop', () => {
    testBinopBool({ op: 'or', l: false, r: true, expect: true })
})
import { LiteralValue } from '@/ast/literal'
import { expect, test } from 'bun:test'
import { interpret, last, maker, setup } from './utils'

function testUnopInt (props: { op: string, expr: number, expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const unop = make.unop(make.literal.int(props.expr), props.op)
    expect(last(interpret(...exprs, unop))).toBe(props.expect)
}

function testUnopDecimal (props: { op: string, expr: [number, number], expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const unop = make.unop(make.literal.dec(...props.expr), props.op)
    expect(last(interpret(...exprs, unop))).toBe(props.expect)
}

function testUnopBool (props: { op: string, expr: boolean, expect: LiteralValue }) {
    const [scope, make, exprs] = setup()
    const unop = make.unop(make.literal.bool(props.expr), props.op)
    expect(last(interpret(...exprs, unop))).toBe(props.expect)
}

test('negative int unop', () => {
    testUnopInt({ op: '-', expr: 2, expect: -2 })
})

test('negative decimal unop', () => {
    testUnopDecimal({ op: '-', expr: [1, 5], expect: -1.5 })
})

test('not true bool unop', () => {
    testUnopBool({ op: 'not', expr: true, expect: false })
})

test('not false bool unop', () => {
    testUnopBool({ op: 'not', expr: false, expect: true })
})

import { expect, test } from 'bun:test'
import { interpret, last, maker, setup } from './utils'

test('int literal', () => {
    const [scope, make, exprs] = setup()
    const int = make.literal.int(123)
    expect(int.value).toBe(123)
    expect(last(interpret(...exprs, int))).toBe(123)
})

test('bool literal', () => {
    const [scope, make, exprs] = setup()
    const bool = make.literal.bool(false)
    expect(bool.value).toBe(false)
    expect(last(interpret(...exprs, bool))).toBe(false)
})

test('none literal', () => {
    const [scope, make, exprs] = setup()
    const none = make.literal.none()
    expect(none.value).toBe(null)
    expect(last(interpret(...exprs, none))).toBe(null)
})

test('decimal literal', () => {
    const [scope, make, exprs] = setup()
    const dec = make.literal.dec(1, 5)
    expect(dec.value).toEqual([1, 5])
    expect(last(interpret(...exprs, dec))).toEqual(1.5)
})
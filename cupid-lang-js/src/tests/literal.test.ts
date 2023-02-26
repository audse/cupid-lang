import { expect, test } from 'bun:test'
import { interpret, maker, setup } from './utils'

test('int literal', () => {
    const [scope, make] = setup()
    const int = make.int(123)
    expect(int.value).toBe(123)
    expect(interpret(int)[0]).toBe(123)
})

test('bool literal', () => {
    const [scope, make] = setup()
    const bool = make.bool(false)
    expect(bool.value).toBe(false)
    expect(interpret(bool)[0]).toBe(false)
})

test('none literal', () => {
    const [scope, make] = setup()
    const none = make.none()
    expect(none.value).toBe(null)
    expect(interpret(none)[0]).toBe(null)
})

test('decimal literal', () => {
    const [scope, make] = setup()
    const dec = make.dec(1, 5)
    expect(dec.value).toEqual([1, 5])
    expect(interpret(dec)[0]).toEqual(1.5)
})
import { expect, test } from 'bun:test'
import { interpret, maker, setup } from './utils'

test('block', () => {
    const [scope, make] = setup()
    const block = make.block(make.int(123))
    expect(interpret(block)[0]).toBe(123)
})

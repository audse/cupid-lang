import { expect, test } from 'bun:test'
import { interpret, maker, last, setup } from './utils'

test('block', () => {
    const [scope, make, exprs] = setup()
    const block = make.block(make.literal.int(123))
    expect(last(interpret(...exprs, block))).toBe(123)
})

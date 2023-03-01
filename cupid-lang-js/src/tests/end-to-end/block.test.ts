import { describe, expect, test } from 'bun:test'
import { interpret, last } from '../utils'
import { setup } from './utils'

describe('block end-to-end', () => {

    test('block', () => {
        const { exprs } = setup(`{ 1 + 1 }`)
        expect(last(interpret(...exprs))).toBe(2)
    })

    test('empty block', () => {
        const { exprs } = setup(`{}`)
        expect(last(interpret(...exprs))).toBe(null)
    })

})
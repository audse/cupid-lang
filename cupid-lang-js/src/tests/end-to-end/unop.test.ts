import { describe, expect, test } from 'bun:test'
import { interpret, last } from '../utils'
import { setup } from './utils'

describe('unop end-to-end', () => {

    test('negative int', () => {
        const { exprs } = setup(`-1`)
        expect(last(interpret(...exprs))).toBe(-1)
    })

    test('negative decimal', () => {
        const { exprs } = setup(`-1.5`)
        expect(last(interpret(...exprs))).toBe(-1.5)
    })

    test('not true bool', () => {
        const { exprs } = setup(`not true`)
        expect(last(interpret(...exprs))).toBe(false)
    })

    test('not false bool', () => {
        const { exprs } = setup(`not false`)
        expect(last(interpret(...exprs))).toBe(true)
    })

})
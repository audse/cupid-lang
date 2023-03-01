import { describe, expect, test } from 'bun:test'
import { interpret, last } from '../utils'
import { setup } from './utils'

describe('binop end-to-end', () => {

    test('add int', () => {
        const { exprs } = setup(`1 + 1`)
        expect(last(interpret(...exprs))).toBe(2)
    })

    test('subtract decimal', () => {
        const { exprs } = setup(`1.5 - 1.5`)
        expect(last(interpret(...exprs))).toBe(0.0)
    })

    test('compare bool', () => {
        const { exprs } = setup(`false or true`)
        expect(last(interpret(...exprs))).toBe(true)
    })

    test('compare str', () => {
        const { exprs } = setup(`'some str' is 'other str'`)
        expect(last(interpret(...exprs))).toBe(false)
    })

})
import { describe, expect, test } from 'bun:test'
import { interpret, last } from '../utils'
import { setup } from './utils'

describe('literal end-to-end', () => {

    test('int', () => {
        const { exprs } = setup(`100`)
        expect(last(interpret(...exprs))).toBe(100)
    })

    test('decimal', () => {
        const { exprs } = setup(`1.5`)
        expect(last(interpret(...exprs))).toBe(1.5)
    })

    test('none', () => {
        const { exprs } = setup(`none`)
        expect(last(interpret(...exprs))).toBe(null)
    })

    test('bool', () => {
        const { exprs } = setup(`false`)
        expect(last(interpret(...exprs))).toBe(false)
    })

    test('str', () => {
        const { exprs } = setup(`'some str'`)
        expect(last(interpret(...exprs))).toBe('some str')
    })

})
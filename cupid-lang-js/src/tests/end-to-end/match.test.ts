import { CompilationErrorCode } from '@/error/compilation-error'
import { describe, expect, test } from 'bun:test'
import { expectCompilationError, interpret, last } from '../utils'
import { setup } from './utils'

describe('match end-to-end', () => {

    test('match boolean true', () => {
        const { exprs } = setup(`
            match true [
                true : 1,
                false : 2,
                _ : 3
            ]
        `)
        expect(last(interpret(...exprs))).toBe(1)
    })

    test('match boolean false', () => {
        const { exprs } = setup(`
            match false [
                true : 1,
                false : 2,
                _ : 3
            ]
        `)
        expect(last(interpret(...exprs))).toBe(2)
    })

    test('match int default', () => {
        const { exprs } = setup(`
            match false [
                1 : 1,
                2 : 2
                _ : 3
            ]
        `)
        expect(last(interpret(...exprs))).toBe(3)
    })


})
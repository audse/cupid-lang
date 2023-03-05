import { describe, expect, test } from 'bun:test'
import { compile, expectCompilationError, interpret, last } from '../utils'
import { CompilationErrorCode } from '@/error/compilation-error'
import { Fun, Ident, Impl, InstanceType, Literal, PrimitiveType } from '@/ast'
import { setup } from './utils'

describe('env end-to-end', () => {

    test('map literal', () => {
        const { exprs } = setup(`
            let point = [
                x : 10,
                y : 20
            ]
            point\\x
        `)
        expect(interpret(...exprs).at(-1)).toBe(10)
    })

    test('nested literal', () => {
        const { exprs } = setup(`
            let node = [
                name : 'a',
                child : [
                    name : 'b',
                    child : none
                ]
            ]
            let c = node\\child
            c\\name
        `)
        expect(interpret(...exprs).at(-1)).toBe('b')
    })

})
import { FieldType, PrimitiveType } from '@/ast/index'
import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, last, maker, setup } from './utils'

test('env', () => {
    const [_, make, exprs] = setup()
    const env = make.env([])
    interpret(...exprs, env)
    const type = env.expectType().getResolved()
    expect(type instanceof PrimitiveType && type.name === 'env').toBeTruthy()
})

test('env lookup', () => {
    const [_, make, exprs] = setup()
    const env = make.env([
        make.quick.decl.int('x', 1)
    ], make.ident('my-env'))
    const lookup = make.lookup(
        make.ident('my-env'),
        make.ident('x')
    )
    expect(last(interpret(...exprs, env, lookup))).toBe(1)
})

test('env lookup call', () => {
    const [_, make, exprs] = setup()
    const env = make.env([
        make.quick.decl.addFun(),
    ], make.ident('my-env'))
    const lookup = make.call(
        make.lookup(
            make.ident('my-env'),
            make.ident('add')
        ),
        make.literal.int(1),
        make.literal.int(1),
    )
    expect(last(interpret(...exprs, env, lookup))).toBe(2)
})

test('type environment call', () => {
    const [_, make, exprs] = setup()
    expect(last(interpret(
        ...exprs,

        // create impl: int [ add: (int, int) -> int ]
        make.impl(
            make.type.instance(make.ident('int')),
            make.env([
                make.quick.decl.addFun()
            ])
        ),

        // create lookup: int\add(1, 2)
        make.call(
            make.lookup(
                make.literal.int(1),
                make.ident('add')
            ),
            make.literal.int(1),
            make.literal.int(1),
        )
    ))).toBe(2)
})

test('struct type field', () => {
    const [_, make, exprs] = setup()

    const intPointConstructor = make.typeConstructor(
        make.ident('int-point'),
        make.quick.instance.pointStruct('int')
    )

    // create lookup: int-point\x
    const lookup = make.lookup(
        make.ident('int-point'),
        make.ident('x'),
    )
    const result = last(interpret(
        ...exprs,
        make.quick.constructor.pointStruct(),
        intPointConstructor,
        lookup
    ))
    // intPointConstructor.body.getResolved().log()
    result.log()
    expect(
        result instanceof FieldType
        && result.ident.name === 'x'
    ).toBeTruthy()
})
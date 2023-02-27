import { FieldType, PrimitiveType } from '@/ast/index'
import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { compile, expectCompilationError, interpret, last, maker, setup } from './utils'

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

test('inferred type environment call', () => {
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

        // create lookup: 1\add(1, 2)
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


test('explicit type environment call', () => {
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
                make.ident('int'),
                make.ident('add')
            ),
            make.literal.int(1),
            make.literal.int(1),
        )
    ))).toBe(2)
})


test('incorrect type environment call', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(
            ...exprs,
            // create impl: int [ add: (int, int) -> int ]
            make.impl(
                make.type.instance(make.ident('int')),
                make.env([
                    make.quick.decl.addFun()
                ])
            ),
            // create lookup: 1.5\add(1.1, 1.2)
            make.call(
                make.lookup(
                    make.literal.dec(1, 5),
                    make.ident('add')
                ),
                make.literal.dec(1, 1),
                make.literal.dec(1, 2),
            )
        )
    )
})

test('explicit type environment call inference', () => {
    const [_, make, exprs] = setup()
    const result = last(compile(
        ...exprs,
        // create impl: int [ add: (int, int) -> int ]
        make.impl(
            make.quick.instanceType('int'),
            make.env([
                make.quick.decl.addFun()
            ])
        ),
        // create lookup: int\add(1, 2)
        make.call(
            make.quick.lookup('int', 'add'),
            make.literal.int(1),
            make.literal.int(1),
        )
    ))
    const type = result.inferredType && result.inferredType.getResolved()
    expect(
        type
        && type instanceof PrimitiveType
        && type.name === 'int'
    ).toBeTruthy()
})
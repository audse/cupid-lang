import { PrimitiveType, StructType } from '@/ast/index'
import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'


test('int type instance', () => {
    const [_, make, exprs] = setup()
    const intInstance = make.quick.instance.int()
    interpret(...exprs, intInstance)
    expect(
        intInstance.value
        && intInstance.value instanceof PrimitiveType
        && intInstance.value.name === 'int'
    ).toBeTruthy()
})

test('struct field type instance', () => {
    const [_, make, exprs] = setup()
    const pointInstance = make.quick.instance.pointStruct()
    interpret(
        ...exprs,
        make.quick.constructor.pointStruct(),
        pointInstance
    )
    const resolvedPoint = pointInstance.getResolved()
    expect(
        resolvedPoint instanceof StructType
        && resolvedPoint.fields.length === 2
        && resolvedPoint.fields[0].type instanceof PrimitiveType
        && resolvedPoint.fields[0].type.name === 'int'
    ).toBeTruthy()
})

test('struct field undefined type instance', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(
            ...exprs,
            make.typeConstructor(
                make.ident('point'),
                make.type.struct([
                    make.type.field(make.ident('x'), make.type.instance(make.ident('t'))),
                    make.type.field(make.ident('y'), make.type.instance(make.ident('t'))),
                ]),
            )
        )
    )
})

test('unable to resolve', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.NotAType,
        () => interpret(
            ...exprs,
            make.decl(make.ident('t'), make.literal.int(123)),
            make.typeConstructor(
                make.ident('some-type'),
                make.type.instance(make.ident('t'))
            )
        )
    )
})

test('wrong number of args', () => {
    const [_, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.IncorrectNumberOfArgs,
        () => interpret(
            ...exprs,
            make.quick.constructor.pointStruct(),
            make.type.instance(make.ident('point'))
        )
    )
})
import { PrimitiveType, StructType } from '@/ast/index'
import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'


test('int type instance', () => {
    const [_, make] = setup()
    const intInstance = make.quick.instance.int()
    interpret(make.quick.constructor.int(), intInstance)
    expect(
        intInstance.value
        && intInstance.value instanceof PrimitiveType
        && intInstance.value.name === 'int'
    ).toBeTruthy()
})

test('struct field type instance', () => {
    const [_, make] = setup()
    const pointInstance = make.quick.instance.pointStruct()
    interpret(
        make.quick.constructor.int(),
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
    const [_, make] = setup()
    expectCompilationError(
        CompilationErrorCode.NotDefined,
        () => interpret(
            make.typeConstructor(
                make.ident('point'),
                make.structType([
                    make.fieldType(make.ident('x'), make.instanceType(make.ident('t'))),
                    make.fieldType(make.ident('y'), make.instanceType(make.ident('t'))),
                ]),
            )
        )
    )
})

test('unable to resolve', () => {
    const [_, make] = setup()
    expectCompilationError(
        CompilationErrorCode.NotAType,
        () => interpret(
            make.decl(make.ident('int'), make.int(123)),
            make.typeConstructor(
                make.ident('some-type'),
                make.instanceType(make.ident('int'))
            )
        )
    )
})

test('wrong number of args', () => {
    const [_, make] = setup()
    expectCompilationError(
        CompilationErrorCode.IncorrectNumberOfArgs,
        () => interpret(
            make.quick.constructor.int(),
            make.quick.constructor.pointStruct(),
            make.instanceType(make.ident('point'))
        )
    )
})
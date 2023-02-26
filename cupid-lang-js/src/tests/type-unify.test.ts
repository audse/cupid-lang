import { PrimitiveType, StructType } from '@/ast/index'
import { CompilationErrorCode } from '@/error/compilation-error'
import { TypeUnifier } from '@/visitors/type-unifier'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'


test('struct field type instance int/unknown unification', () => {
    const [_, make] = setup()
    const pointInstance = make.quick.instance.pointStruct('int')
    const unknownType = make.typeConstructor(make.ident('some-type'), make.unknownType())
    const pointUnknown = make.typeConstructor(
        make.ident('p2'),
        make.structType([
            make.fieldType(make.ident('x'), make.instanceType(make.ident('some-type'))),
            make.fieldType(make.ident('y'), make.instanceType(make.ident('some-type'))),
        ])
    )
    const pointUnknownInstance = make.instanceType(make.ident('p2'))
    interpret(
        make.quick.constructor.int(),
        make.quick.constructor.pointStruct(),
        pointInstance,
        unknownType,
        pointUnknown,
        pointUnknownInstance
    )
    const unifier = new TypeUnifier()
    const resolvedType = unifier.visit(pointInstance, pointUnknownInstance)
    expect(
        resolvedType instanceof StructType
        && resolvedType.fields.length === 2
        && resolvedType.fields[0].type instanceof PrimitiveType
        && resolvedType.fields[0].type.name === 'int'
    ).toBeTruthy()
})


test('struct field type instance int/decimal unification', () => {
    const [_, make] = setup()
    const pointInstanceA = make.quick.instance.pointStruct('int')
    const pointInstanceB = make.quick.instance.pointStruct('decimal')
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => {
            interpret(
                make.quick.constructor.int(),
                make.quick.constructor.decimal(),
                make.quick.constructor.pointStruct(),
                pointInstanceA,
                pointInstanceB,
            )
            new TypeUnifier().visit(pointInstanceA, pointInstanceB)
        }
    )
})
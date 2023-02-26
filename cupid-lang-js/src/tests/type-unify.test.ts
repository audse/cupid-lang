import { PrimitiveType, StructType } from '@/ast/index'
import { CompilationErrorCode } from '@/error/compilation-error'
import { TypeUnifier } from '@/visitors/type-unifier'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'


test('struct field type instance int/unknown unification', () => {
    const [_, make, exprs] = setup()
    const pointInstance = make.quick.instance.pointStruct('int')
    const unknown = make.typeConstructor(make.ident('some-type'), make.type.unknown())
    const pointUnknown = make.typeConstructor(
        make.ident('p2'),
        make.type.struct([
            make.type.field(make.ident('x'), make.type.instance(make.ident('some-type'))),
            make.type.field(make.ident('y'), make.type.instance(make.ident('some-type'))),
        ])
    )
    const pointUnknownInstance = make.type.instance(make.ident('p2'))
    interpret(
        ...exprs,
        make.quick.constructor.pointStruct(),
        pointInstance,
        unknown,
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
    const [_, make, exprs] = setup()
    const pointInstanceA = make.quick.instance.pointStruct('int')
    const pointInstanceB = make.quick.instance.pointStruct('decimal')
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => {
            interpret(
                ...exprs,
                make.quick.constructor.pointStruct(),
                pointInstanceA,
                pointInstanceB,
            )
            new TypeUnifier().visit(pointInstanceA, pointInstanceB)
        }
    )
})
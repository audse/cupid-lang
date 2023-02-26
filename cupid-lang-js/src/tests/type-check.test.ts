import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'

test('int declared as decimal', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            make.decl(
                make.ident('x'),
                make.int(123),
                make.primitiveType('decimal')
            ),
            make.ident('x')
        )
    )
})

test('bool declared as int', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            make.quick.constructor.int(),
            make.decl(
                make.ident('x'),
                make.bool(true),
                make.quick.instance.int()
            ),
            make.ident('x')
        )
    )
})

test('mismatched binop', () => {
    const [scope, make] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(make.binop(
            make.int(1),
            make.dec(1, 5),
            '+'
        ))
    )
})
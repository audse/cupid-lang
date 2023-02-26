import { CompilationError, CompilationErrorCode } from '@/error/compilation-error'
import { expect, test } from 'bun:test'
import { expectCompilationError, interpret, setup } from './utils'

test('int declared as decimal', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            ...exprs,
            make.decl(
                make.ident('x'),
                make.literal.int(123),
                make.type.primitive('decimal')
            ),
            make.ident('x')
        )
    )
})

test('bool declared as int', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            ...exprs,
            make.decl(
                make.ident('x'),
                make.literal.bool(true),
                make.quick.instance.int()
            ),
            make.ident('x')
        )
    )
})

test('mismatched binop', () => {
    const [scope, make, exprs] = setup()
    expectCompilationError(
        CompilationErrorCode.UnableToUnifyType,
        () => interpret(
            ...exprs,
            make.binop(
                make.literal.int(1),
                make.literal.dec(1, 5),
                '+'
            )
        )
    )
})
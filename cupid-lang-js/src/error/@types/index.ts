import { Kind } from '@/ast/kind'
import { Expr as Expr1, Type as Type1, Field as Field1 } from '@/passes/@types/1-pre-create-scope'
import { Expr as Expr2, Type as Type2, Field as Field2 } from '@/passes/@types/2-pre-define-symbols'
import { Expr as Expr3, Type as Type3, Field as Field3 } from '@/passes/@types/3-pre-resolve-types'
import { Expr as Expr4, Type as Type4, Field as Field4 } from '@/passes/@types/4-pre-infer-types'
import { Expr as Expr5, Type as Type5, Field as Field5 } from '@/passes/@types/5-pre-check-types'

export enum ErrorCode {
    NotFound = 'not found',
    AlreadyDefined = 'already defined',
    NotMutable = 'not mutable',

    InvalidOperation = 'invalid operation',
    TypeMismatch = 'type mismatch',
    CannotInfer = 'cannot infer type',

    // Functions
    NotAType = 'not a type',
    NotAFunction = 'not a function',
    IncorrectNumberOfArgs = 'incorrect number of args',

    // Misc
    SomethingHappened = 'something happened',
    Unimplemented = 'not yet implemented',
    Unreachable = 'unreachable',
}

export type AnyExpr<K extends Kind = Kind> = Expr1<K> | Expr2<K> | Expr3<K> | Expr4<K> | Expr5<K>
export type AnyType = Type1 | Type2 | Type3 | Type4 | Type5
export type AnyField = Field1 | Field2 | Field3 | Field4 | Field5

export type Err = {
    code: ErrorCode
    context: string,
    expr: AnyExpr | AnyType | Step[]
}

export type Step = {
    context: string
    expr: AnyExpr | AnyType
}
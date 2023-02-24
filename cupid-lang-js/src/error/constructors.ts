import { Kind } from '@/ast/kind'
import { paren } from '@/codegen'
import { Result } from '@/types'
import { AnyExpr, AnyType, Err, err, ErrorCode, formatType } from './index'


export function typeAnnotationMismatch (expr: AnyExpr, declaredType: AnyType | null, actualType: AnyType | null): Result<any, Err> {
    const type = (typ: AnyType | null) => typ ? paren(formatType(typ)) : ''
    return err(
        ErrorCode.TypeMismatch,
        `actual type ${ type(actualType) } does not match annotated type ${ type(declaredType) }`,
        expr
    )
}

export function argTypeMismatch (param: AnyExpr<Kind.Fun>['params'][number], arg: AnyExpr<Kind.Call>['args'][number], callType: AnyType): Result<any, Err> {
    return err(
        ErrorCode.TypeMismatch,
        `expected an argument of type ${ formatType(param.type) }, instead found type ${ formatType(callType) }`,
        [
            { context: 'paramater was declared here...', expr: param.ident },
            { context: '...and called with incorrect type here', expr: arg }
        ]
    )
}

export function notFound (ident: AnyExpr<Kind.Ident>): Result<any, Err> {
    return err(
        ErrorCode.NotFound,
        `name \`${ ident.name }\` is either undefined or not in scope`,
        ident
    )
}
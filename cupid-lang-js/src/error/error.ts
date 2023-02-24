import { Result } from '@/types'
import { AnyExpr, Err, ErrorCode } from './index'

export function err (code: ErrorCode, context: string = '', expr: Err['expr']): Result<any, Err> {
    return {
        ok: false,
        err: {
            code,
            context,
            expr
        }
    }
}

export function ok<T> (result: T): Result<T, any> {
    return {
        ok: true,
        result
    }
}
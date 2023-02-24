export * from './codegen'
export * from './grammar'
export * from './parse'
export * from './scope'
export * from './tokenize'

export type Option<T> = T | null

export type Result<Ok, Err> = { ok: true, result: Ok } | { ok: false, err: Err }


export type DeepRequired<T> = Required<{
    [P in keyof T]: T[P] extends object | undefined ? DeepRequired<Required<T[P]>> : T[P]
}>

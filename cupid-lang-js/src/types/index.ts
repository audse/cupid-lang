export * from './codegen'
export * from './grammar'
export * from './parse'
export * from './tokenize'

export type Option<T> = T | null

export type Result<Ok, Err> = { ok: true, result: Ok } | { ok: false, err: Err }

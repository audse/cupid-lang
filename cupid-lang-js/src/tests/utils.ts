import { Assign, BinOp, Block, Call, Decl, Expr, FieldType, Fun, Ident, InstanceType, Literal, PrimitiveType, StructType, Type, TypeConstructor, UnknownType } from '@/ast'
import { Scope } from '@/env'
import { CompilationError, CompilationErrorCode, RuntimeError, RuntimeErrorCode } from '@/error/index'
import { Infer, Interpreter, ScopeAnalyzer, SymbolDefiner, SymbolResolver, TypeChecker, TypeInferrer, TypeResolver } from '@/visitors'
import { TypeUnifier } from '@/visitors/type-unifier'
import { expect } from 'bun:test'


export function maker (scope: Scope) {
    const int = (value: number) => new Literal({ scope, value })
    const dec = (...value: [number, number]) => new Literal({ scope, value })
    const bool = (value: boolean) => new Literal({ scope, value })
    const none = () => new Literal({ scope, value: null })
    const assign = (ident: Ident, value: Expr) => new Assign({ scope, ident, value })
    const binop = (left: Expr, right: Expr, op: string) => new BinOp({ scope, left, right, op })
    const block = (...exprs: Expr[]) => new Block({ scope, exprs })
    const ident = (name: string) => new Ident({ scope, name })
    const decl = (ident: Ident, value: Expr, type?: Type, mutable?: boolean) => new Decl({ scope, ident, value, type, mutable })
    const instanceType = (ident: Ident, args: Type[] = []) => new InstanceType({ scope, ident, args })
    const unknownType = () => new UnknownType({ scope })
    const structType = (fields: FieldType[]) => new StructType({ scope, fields })
    const fieldType = (ident: Ident, type: Type = unknownType()) => new FieldType({ scope, ident, type })
    const primitiveType = (name: string) => new PrimitiveType({ scope, name })
    const typeConstructor = (ident: Ident, body: Type, params: Ident[] = []) => new TypeConstructor({
        scope,
        ident,
        params,
        body
    })
    const fun = (params: FieldType[], body: Expr, returns: Type = unknownType()) => new Fun({ scope, params, body, returns })
    const call = (fun: Expr, ...args: Expr[]) => new Call({ scope, fun, args })
    return {
        int,
        dec,
        bool,
        none,
        assign,
        binop,
        block,
        ident,
        decl,
        fun,
        call,
        instanceType,
        unknownType,
        structType,
        fieldType,
        primitiveType,
        typeConstructor,

        quick: {
            decl: {
                int: (name: string = 'x', value: number = 1) => decl(ident(name), int(value), instanceType(ident('int'))),
                dec: (name: string, ...value: [number, number]) => decl(ident(name), dec(...value), instanceType(ident('dec'))),
                addFun: () => decl(
                    ident('add'),
                    fun([
                        fieldType(ident('a'), instanceType(ident('int'))),
                        fieldType(ident('b'), instanceType(ident('int')))
                    ], binop(ident('a'), ident('b'), '+'))
                )
            },
            constructor: {
                int: () => typeConstructor(ident('int'), primitiveType('int')),
                decimal: () => typeConstructor(ident('decimal'), primitiveType('decimal')),
                bool: () => typeConstructor(ident('bool'), primitiveType('bool')),
                none: () => typeConstructor(ident('none'), primitiveType('none')),
                pointStruct: () => typeConstructor(
                    ident('point'),
                    structType([
                        fieldType(ident('x'), instanceType(ident('t'))),
                        fieldType(ident('y'), instanceType(ident('t'))),
                    ]),
                    [ident('t')]
                )
            },
            instance: {
                int: () => instanceType(ident('int')),
                decimal: () => instanceType(ident('decimal')),
                bool: () => instanceType(ident('bool')),
                pointStruct: (name: 'int' | 'decimal' = 'int') => instanceType(
                    ident('point'),
                    [instanceType((ident(name)))]
                )
            },
            primitiveConstructors () {
                return [
                    this.constructor.int(),
                    this.constructor.decimal(),
                    this.constructor.bool(),
                    this.constructor.none(),
                ]
            }
        },
    } as const
}

export function setup () {
    const scope = new Scope()
    const make = maker(scope)
    return [scope, make] as const
}

export function compile (...exprs: Expr[]): Expr[] {
    const scopeAnalyzer = new ScopeAnalyzer()
    exprs.map(expr => scopeAnalyzer.visit(expr))

    const symbolDefiner = new SymbolDefiner()
    exprs.map(expr => symbolDefiner.visit(expr))

    const symbolResolver = new SymbolResolver()
    exprs.map(expr => symbolResolver.visit(expr))

    const typeResolver = new TypeResolver()
    exprs.map(expr => typeResolver.visit(expr))

    const typeInferrer = new TypeInferrer()
    const inferrer = new Infer()
    exprs.map(expr => typeInferrer.visit(expr, inferrer))

    const typeChecker = new TypeChecker()
    const unifier = new TypeUnifier()
    exprs.map(expr => typeChecker.visit(expr, unifier))

    return exprs
}


export function interpret (...exprs: Expr[]) {
    const compiledExprs = compile(...exprs)
    const interpreter = new Interpreter()
    return compiledExprs.map(expr => interpreter.visit(expr))
}


export function report (error: any) {
    if (error instanceof CompilationError<Expr>) error.log()
    return `${ error }`
}


export function expectCompilationError<T> (code: CompilationErrorCode, inner: () => T) {
    try {
        const result = inner()
        throw `expected compilation error ${ code }, found ${ result }`
    } catch (error) {
        if (error instanceof CompilationError && error.code !== code) {
            if ([CompilationErrorCode.AlreadyDefined, CompilationErrorCode.NotDefined].includes(error.code) && 'scope' in error.context) error.context.scope.log()
            error.log()
        }
        expect(
            error instanceof CompilationError
            && error.code === code
        ).toBeTruthy()
    }
}

export function expectRuntimeError<T> (code: RuntimeErrorCode, inner: () => T) {
    try {
        const result = inner()
        throw `expected compilation error ${ code }, found ${ result }`
    } catch (error) {
        if (error instanceof RuntimeError && error.code !== code) error.log()
        expect(
            error instanceof RuntimeError
            && error.code === code
        ).toBeTruthy()
    }
}


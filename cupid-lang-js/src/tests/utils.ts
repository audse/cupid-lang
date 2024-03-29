import { Assign, BinOp, Block, Branch, Call, Decl, Environment, Expr, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, PrimitiveType, StructType, Type, TypeConstructor, UnknownType, UnOp } from '@/ast'
import { Scope } from '@/env'
import { CompilationError, CompilationErrorCode, RuntimeError, RuntimeErrorCode } from '@/error/index'
import { Infer, Interpreter, LookupEnvironmentFinder, LookupEnvironmentResolver, LookupMemberResolver, ScopeAnalyzer, SymbolDefiner, SymbolResolver, TypeChecker, TypeInferrer, TypeResolver } from '@/visitors'
import { TypeUnifier } from '@/visitors/type-unifier'
import { expect } from 'bun:test'


export function maker (scope: Scope) {
    const literal = {
        int: (value: number) => new Literal({ scope, value }),
        dec: (...value: [number, number]) => new Literal({ scope, value }),
        bool: (value: boolean) => new Literal({ scope, value }),
        none: () => new Literal({ scope, value: null }),
    }
    const assign = (ident: Ident, value: Expr) => new Assign({ scope, ident, value })
    const binop = (left: Expr, right: Expr, op: string) => new BinOp({ scope, left, right, op })
    const block = (...exprs: Expr[]) => new Block({ scope, exprs })
    const branch = (condition: Expr, body: Expr, elseBody: Expr) => new Branch({ scope, condition, body, else: elseBody })
    const ident = (name: string) => new Ident({ scope, name })
    const decl = (ident: Ident, value: Expr, type?: Type, mutable?: boolean) => new Decl({ scope, ident, value, type, mutable })

    const fun = (params: FieldType[], body: Expr, returns: Type = type.unknown(), hasSelfParam: boolean = false) => new Fun({ scope, params, body, returns, hasSelfParam })
    const call = (fun: Expr, ...args: Expr[]) => new Call({ scope, fun, args })
    const env = (content: Expr[], ident?: Ident) => new Environment({ scope, ident, content })
    const lookup = (environment: Expr, member: Ident | Literal) => new Lookup({ scope, environment, member })
    const typeConstructor = (ident: Ident, body: Type, params: Ident[] = []) => new TypeConstructor({
        scope,
        ident,
        params,
        body,
    })
    const unop = (expr: Expr, op: string) => new UnOp({ scope, expr, op })
    const impl = (type: Type, environment: Environment) => new Impl({ scope, type, environment })
    const type = {
        instance: (ident: Ident, args: Type[] = []) => new InstanceType({ scope, ident, args }),
        unknown: () => new UnknownType({ scope }),
        struct: (fields: FieldType[]) => new StructType({ scope, fields }),
        field: (ident: Ident, type: Type = new UnknownType({ scope })) => new FieldType({ scope, ident, type }),
        primitive: (name: string) => new PrimitiveType({ scope, name }),
        fun: (params: FieldType[], returns: Type = new PrimitiveType({ scope, name: 'none' })) => new FunType({
            scope,
            params,
            returns
        })
    }
    return {
        assign,
        binop,
        block,
        branch,
        call,
        decl,
        env,
        fun,
        ident,
        impl,
        literal,
        lookup,
        type,
        typeConstructor,
        unop,

        quick: {
            lookup: (env: string, member: string) => lookup(ident(env), ident(member)),
            instanceType: (name: string, ...args: string[]) => type.instance(
                ident(name),
                args.map(arg => type.instance(ident(arg)))
            ),
            fieldType: (name: string, typeInstance: string) => type.field(ident(name), type.instance(ident(typeInstance))),
            decl: {
                int: (name: string = 'x', value: number = 1, mutable?: boolean) => decl(ident(name), literal.int(value), type.instance(ident('int')), mutable),
                dec: (name: string, ...value: [number, number]) => decl(ident(name), literal.dec(...value), type.instance(ident('dec'))),
                addFun: () => decl(
                    ident('add'),
                    fun(
                        [
                            type.field(ident('a'), type.instance(ident('int'))),
                            type.field(ident('b'), type.instance(ident('int')))
                        ],
                        binop(ident('a'), ident('b'), '+'),
                        type.instance(ident('int'))
                    )
                )
            },
            constructor: {
                int: () => typeConstructor(ident('int'), type.primitive('int')),
                decimal: () => typeConstructor(ident('decimal'), type.primitive('decimal')),
                bool: () => typeConstructor(ident('bool'), type.primitive('bool')),
                none: () => typeConstructor(ident('none'), type.primitive('none')),
                env: () => typeConstructor(ident('env'), type.primitive('env')),
                type: () => typeConstructor(ident('type'), type.primitive('type')),
                pointStruct: () => typeConstructor(
                    ident('point'),
                    type.struct([
                        type.field(ident('x'), type.instance(ident('t'))),
                        type.field(ident('y'), type.instance(ident('t'))),
                    ]),
                    [ident('t')]
                )
            },
            instance: {
                int: () => type.instance(ident('int')),
                decimal: () => type.instance(ident('decimal')),
                bool: () => type.instance(ident('bool')),
                pointStruct: (name: 'int' | 'decimal' = 'int') => type.instance(
                    ident('point'),
                    [type.instance((ident(name)))]
                )
            },
            primitiveConstructors () {
                return [
                    this.constructor.int(),
                    this.constructor.decimal(),
                    this.constructor.bool(),
                    this.constructor.none(),
                    this.constructor.env(),
                    this.constructor.type(),
                ]
            }
        },
    } as const
}

export function setup () {
    const scope = new Scope()
    const make = maker(scope)
    const exprs: Expr[] = make.quick.primitiveConstructors()
    return [scope, make, exprs] as const
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

    const lookupEnvironmentFinder = new LookupEnvironmentFinder()
    const lookupEnvironmentResolver = new LookupEnvironmentResolver()
    exprs.map(expr => lookupEnvironmentResolver.visit(expr, lookupEnvironmentFinder))

    const lookupMemberResolver = new LookupMemberResolver()
    exprs.map(expr => lookupMemberResolver.visit(expr))

    // reinfer after lookup resolution
    exprs.map(expr => inferrer.visit(expr))

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


export function last<T> (arr: T[]): T {
    return arr[arr.length - 1]
}


export function expectCompilationError<T> (code: CompilationErrorCode, inner: () => T) {
    try {
        const result = inner()
        console.error(`expected compilation error [${ code }] - instead found result`)
        throw result
    } catch (error) {
        if (error instanceof CompilationError && error.code !== code) error.log()
        else if (!(error instanceof CompilationError)) console.log(error)
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
        else if (!(error instanceof CompilationError)) console.log(error)
        expect(
            error instanceof RuntimeError
            && error.code === code
        ).toBeTruthy()
    }
}


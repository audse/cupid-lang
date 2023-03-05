import { Assign, BinOp, Block, Branch, Call, Decl, Environment, Expr, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, PrimitiveType, StructType, Type, TypeConstructor, UnknownType, UnOp } from '@/ast'
import { Scope } from '@/env'
import { CompilationError, CompilationErrorCode, RuntimeError, RuntimeErrorCode } from '@/error/index'
import { ErrorFormatter, Infer, Interpreter, LookupEnvironmentFinder, LookupEnvironmentResolver, LookupMemberResolver, ScopeAnalyzer, SymbolDefiner, SymbolResolver, TypeChecker, TypeInferrer, TypeResolver } from '@/visitors'
import { TypeUnifier } from '@/visitors/type-unifier'
import { Tokenizer } from '@/tokenize'
import { TokenParser } from '@/parse/parse'
import { Node, nodeIs, Option, token } from '@/types'
import { cupid } from '@/parse/cupid.parser'
import { IntoAst, intoAst } from '@/into-ast'
import { FileFormatter } from '@/fmt/utils'

export class Cupid {

    scope: Scope
    intoAst: IntoAst

    paths: string[] = []
    files: string[] = []
    source: Node[] = []
    exprs: Expr[] = []

    constructor () {
        const into = intoAst()
        this.intoAst = into
        this.scope = into.scope
        this.source = into.source
    }

    addFile (path: string, content: string) {
        this.paths.push(path)
        const file = this.files.push(content) - 1
        const tokens = new Tokenizer(file, content).tokenize()
        const nodes = this.parse(tokens)
        this.intoAst.setFile(file)
        const exprs = nodes.map(this.intoAst.into.bind(this.intoAst))
        exprs.map(expr => expr.file = file)
        this.exprs.push(...exprs)
    }

    addFiles (...content: [string, string][]) {
        content.map(([path, content]) => this.addFile(path, content))
    }

    parse (tokens: token.Token[]): Node[] {
        const parser = new TokenParser(tokens)
        const exprs = []
        while (parser.peek()) {
            const expr = cupid.expr(parser)
            if (expr) exprs.push(...expr)
            else {
                console.error({
                    error: `unable to parse token`,
                    token: parser.current()
                })
                throw 'unable to parse token'
            }
        }
        return exprs
    }

    #ScopeAnalyzer = new ScopeAnalyzer()
    #SymbolDefiner = new SymbolDefiner()
    #SymbolResolver = new SymbolResolver()
    #TypeResolver = new TypeResolver()
    #TypeInferrer = new TypeInferrer()
    #Inferrer = new Infer()
    #LookupEnvironmentFinder = new LookupEnvironmentFinder()
    #LookupEnvironmentResolver = new LookupEnvironmentResolver()
    #LookupMemberResolver = new LookupMemberResolver()
    #TypeChecker = new TypeChecker()
    #TypeUnifier = new TypeUnifier()

    compile () {
        return this.#try(() => {
            this.exprs.map(expr => this.#ScopeAnalyzer.visit(expr))
            this.exprs.map(expr => this.#SymbolDefiner.visit(expr))
            this.exprs.map(expr => this.#SymbolResolver.visit(expr))
            this.exprs.map(expr => this.#TypeResolver.visit(expr))
            this.exprs.map(expr => this.#TypeInferrer.visit(expr, this.#Inferrer))
            this.exprs.map(expr => this.#LookupEnvironmentResolver.visit(expr, this.#LookupEnvironmentFinder))
            this.exprs.map(expr => this.#LookupMemberResolver.visit(expr))
            // reinfer after lookup resolution
            this.exprs.map(expr => this.#Inferrer.visit(expr))
            this.exprs.map(expr => this.#TypeChecker.visit(expr, this.#TypeUnifier))
            return true
        })
    }

    interpret () {
        return this.compile() ? this.#try(() => {
            const interpreter = new Interpreter()
            return this.exprs.map(expr => interpreter.visit(expr))
        }) : null
    }

    formatter (file: number) {
        const path = this.paths[file].split('/src/')[1]
        const content = this.files[file]
        return new FileFormatter(path, content, file)
    }

    #try<T> (fun: () => T): Option<T> {
        try {
            return fun()
        } catch (error) {
            if (error instanceof CompilationError || error instanceof RuntimeError) {
                error.log()
                if (error.context instanceof Expr) {
                    if (error.context.file === -1) console.log(
                        'No file found:',
                        error.context.report(),
                        this.source[error.context.source - 1]
                    )
                    const fmt = this.formatter(error.context.file)
                    new ErrorFormatter().visit(error.context, { fmt, source: this.source })
                    console.log(fmt.useConsoleColors().useLineNumbers().build())
                }
            }
            else console.log(error)
            return null
        }
    }

}
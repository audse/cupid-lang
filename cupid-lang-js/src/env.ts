import { Option } from '@/types'
import { Type, Ident, Expr } from '@/ast'
import { CompilationError } from './error/compilation-error'
import { Reportable } from './error/index'
import { stringify } from './utils'


export interface SymbolProps {
    ident: Ident
    value?: Option<Expr>
    type?: Option<Type>
    mutable?: boolean
}

export class Symbol implements SymbolProps, Reportable {

    ident: Ident
    value: Option<Expr> = null
    type: Option<Type> = null
    mutable: boolean = false

    constructor (props: SymbolProps) {
        this.ident = props.ident
        this.value = props.value || null
        this.type = props.type || null
        this.mutable = props.mutable || false
    }

    report () {
        return stringify({
            ident: this.ident.report(),
            value: this.value?.report() || null,
            type: this.type?.report() || null,
            mutable: this.mutable
        })
    }

    log () {
        console.log(this.report())
    }
}

export enum Context {
    Block = 'block',
    Fun = 'fun',
    TypeConstructor = 'typedef',
    Call = 'call',
}

export interface ScopeProps {
    parent: Option<Scope>
    context: Context
    symbols: Symbol[]
}

export class Scope implements ScopeProps, Reportable {

    parent: Option<Scope> = null
    context: Context = Context.Block
    symbols: Symbol[] = []

    constructor (parent?: Scope, context?: Context) {
        if (context) this.context = context
        if (parent) this.parent = parent
    }

    subscope (context?: Context): Scope {
        return new Scope(this, context)
    }

    define (symbol: SymbolProps): Symbol {
        const existingSymbol = this.find(symbol.ident)
        if (existingSymbol) throw CompilationError.alreadyDefined(symbol.ident)
        const newSymbol = new Symbol(symbol)
        this.symbols.push(newSymbol)
        return newSymbol
    }

    lookup (ident: Ident): Option<Symbol> {
        const symbol = this.find(ident)
        if (symbol) return symbol
        if (this.parent) return this.parent.lookup(ident)
        return null
    }

    lookupExpect (ident: Ident): Symbol {
        const symbol = this.lookup(ident)
        if (symbol) return symbol
        throw CompilationError.notDefined(ident)
    }

    find (ident: Ident): Option<Symbol> {
        return this.symbols.find(symbol => symbol.ident.isEqual(ident)) || null
    }

    annotate (ident: Ident, props: Partial<SymbolProps>): Symbol {
        const symbol = this.lookupExpect(ident)
        if ('ident' in props && props.ident) symbol.ident = props.ident
        if ('type' in props && props.type) symbol.type = props.type
        if ('value' in props && props.value !== undefined) symbol.value = props.value
        return symbol
    }

    report (): string {
        return stringify({
            context: this.context,
            parent: this.parent?.report(),
            symbols: this.symbols.map(symbol => symbol.report()),
        })
    }

    log () {
        console.log(this.report())
    }
}
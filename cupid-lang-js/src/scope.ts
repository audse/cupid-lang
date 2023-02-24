import { Option } from '@/types'
import { Ident } from '@/ast'
import { err } from './error/error'
import { ErrorCode } from './error/index'

type IdentBase = { name: string }

export type Symbol<I extends IdentBase = IdentBase, Type = any, Value = any> = {
    ident: I
    type: Type
    value: Value
    mutable: boolean
}

export class Scope {
    parent?: Scope
    symbols: Symbol[] = []

    constructor (parent?: Scope) {
        this.parent = parent
    }

    subscope () {
        return new Scope(this)
    }

    find<I extends IdentBase = Ident, T = any, V = any> (ident: I): Option<Symbol<I, T, V>> {
        for (const symbol of this.symbols) {
            if (symbol.ident.name === ident.name) return symbol as unknown as Symbol<I, T, V>
        }
        return null
    }

    lookup<I extends IdentBase = Ident, T = any, V = any> (ident: I): Option<Symbol<I, T, V>> {
        const symbol = this.find<I, T, V>(ident)
        if (symbol) return symbol
        if (this.parent) return this.parent.lookup(ident)
        return null
    }

    annotate<I extends IdentBase = Ident, T = any, V = any> (ident: I, value: Partial<Symbol<I, T, V>>): Option<Symbol<I, T, V>> {
        const symbol = this.lookup<I, T, V>(ident)
        if (symbol) {
            Object.assign(symbol, value)
            if ('type' in value) Object.assign(symbol.ident, { inferredType: value.type })
            return symbol
        }
        return null
    }

    define<I extends IdentBase = Ident, T = any, V = any> (ident: I, value: Partial<Symbol<I, T, V>>): Symbol<I, T, V> {
        const symbol = this.find<I, T, V>(ident)
        if (symbol) throw err(ErrorCode.AlreadyDefined)
        const newSymbol = { ident, mutable: false, ...value } as Symbol<I, T, V>
        this.symbols.push(newSymbol)
        return newSymbol
    }
}

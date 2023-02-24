
export enum Kind {
    BinOp = 'binop',
    Block = 'block',
    Call = 'call',
    Decl = 'decl',
    Fun = 'fun',
    Ident = 'ident',
    IfStmt = 'ifstmt',
    Literal = 'literal',
    Map = 'map',
    Property = 'property',
    Type = 'type',
    TypeConstructor = 'typedef',
    // TypeInstance = 'typeinstance',
    UnOp = 'unop',
}

const AllKinds: Set<string> = new Set(Object.values(Kind))

export type HasKind = { kind: string }

export function isExpr<E> (expr: unknown): expr is E {
    return (
        expr
        && typeof expr === 'object'
        && 'kind' in expr
        && AllKinds.has((expr as HasKind).kind)
    ) ? true : false
}

export function isKind<E extends V, V extends HasKind = HasKind> (kind: string, value: V): value is E {
    return value.kind === kind
}

export function isBinOp<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.BinOp, value)
}

export function isBlock<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Block, value)
}

export function isCall<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Call, value)
}

export function isDecl<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Decl, value)
}

export function isFun<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Fun, value)
}

export function isIdent<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Ident, value)
}

export function isIfStmt<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.IfStmt, value)
}

export function isLiteral<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Literal, value)
}

export function isMap<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Map, value)
}

export function isProperty<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Property, value)
}

export function isType<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.Type, value)
}

export function isTypeConstructor<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.TypeConstructor, value)
}

// export function isTypeInstance<E extends V, V extends HasKind = HasKind> (value: V): value is E {
//     return isKind<E>(Kind.TypeInstance, value)
// }

export function isUnOp<E extends V, V extends HasKind = HasKind> (value: V): value is E {
    return isKind<E>(Kind.UnOp, value)
}
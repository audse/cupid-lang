import { Kind, TypeKind, Base as AstBase, Scoped, Literal, Ident, Primitive, Variable, Unknown } from '@/ast'

export type AnyTypeKind = Exclude<TypeKind, TypeKind.Instance>

export type Inferred<K extends Kind = Kind> = {
    inferredType: K extends Kind.Type ? null : Type
}

export type Base<K extends Kind = Kind> = AstBase<K> & Scoped & Inferred<K>

export type Expr<K extends Kind = Kind, T extends AnyTypeKind = AnyTypeKind> = {
    [Kind.BinOp]: Base<K> & {
        left: Expr
        right: Expr
        op: string
    }

    [Kind.Block]: Base<K> & {
        exprs: Expr[]
    }

    [Kind.Call]: Base<K> & {
        fun: Expr
        args: Expr[]
    }

    [Kind.Decl]: Base<K> & {
        ident: Expr<Kind.Ident>
        type: Expr<Kind.Type>
        value: Expr
    }

    [Kind.Fun]: Base<K> & {
        params: Field[]
        returns: Expr<Kind.Type>
        body: Expr
    }

    [Kind.Ident]: Ident & Scoped & Inferred<Kind.Ident>

    [Kind.IfStmt]: Base<K> & {
        condition: Expr
        body: Expr
        elseBody?: Expr
    }

    [Kind.Literal]: Literal & Scoped & Inferred<Kind.Literal>

    [Kind.Map]: Base<K> & {
        entries: [Expr, Expr][]
    }

    [Kind.Property]: Base<K> & {
        parent: Expr
        property: Expr
    }

    [Kind.Type]: Type<T>

    [Kind.TypeConstructor]: Base<K> & {}

    [Kind.UnOp]: Base<K> & {
        expr: Expr
        op: string
    }
}[K]

export type Field = {
    ident: Expr<Kind.Ident>
    type: Expr<Kind.Type>
}

export type Type<T extends AnyTypeKind = AnyTypeKind> = {

    [TypeKind.Fun]: Base<Kind.Type> & {
        typeKind: TypeKind.Fun
        params: Field[]
        returns: Type | null
    }

    [TypeKind.Map]: Base<Kind.Type> & {
        typeKind: TypeKind.Map
        keys: Type
        values: Type
    }

    [TypeKind.Primitive]: Primitive & Scoped & Inferred<Kind.Type>

    [TypeKind.Struct]: Base<Kind.Type> & {
        typeKind: TypeKind.Struct
        fields: Field[]
    }

    [TypeKind.Sum]: Base<Kind.Type> & {
        typeKind: TypeKind.Sum
        fields: Field[]
    }

    [TypeKind.Unknown]: Scoped & Unknown & Inferred<Kind.Type>

    [TypeKind.Variable]: Scoped & Variable & Inferred<Kind.Type>

}[T]
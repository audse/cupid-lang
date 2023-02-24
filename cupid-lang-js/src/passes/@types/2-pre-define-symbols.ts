import { Kind, Base as AstBase, Ident, Literal, Primitive, TypeKind, Scoped, Unknown, Variable } from '@/ast'

export type Base<K extends Kind = Kind> = AstBase<K> & Scoped

export type Expr<K extends Kind = Kind, T extends TypeKind = TypeKind> = {
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

    [Kind.Ident]: Scoped & Ident

    [Kind.IfStmt]: Base<K> & {
        condition: Expr
        body: Expr
        elseBody?: Expr
    }

    [Kind.Literal]: Scoped & Literal

    [Kind.Map]: Base<K> & {
        entries: [Expr, Expr][]
    }

    [Kind.Property]: Base<K> & {
        parent: Expr
        property: Expr
    }

    [Kind.Type]: Type<T>

    [Kind.TypeConstructor]: Base<K> & {
        ident: Expr<Kind.Ident>
        params: Expr<Kind.Ident>[]
        value: Type<T>
    }

    [Kind.UnOp]: Base<K> & {
        expr: Expr
        op: string
    }
}[K]

export type Field = {
    ident: Expr<Kind.Ident>
    type: Expr<Kind.Type>
}

export type Type<T extends TypeKind = TypeKind> = {

    [TypeKind.Fun]: Base<Kind.Type> & {
        typeKind: TypeKind.Fun
        params: Field[]
        returns: Type
    }

    [TypeKind.Instance]: Base<Kind.Type> & {
        typeKind: TypeKind.Instance
        ident: Expr<Kind.Ident>
        args: Type[]
    }

    [TypeKind.Map]: Base<Kind.Type> & {
        typeKind: TypeKind.Map
        keys: Type
        values: Type
    }

    [TypeKind.Primitive]: Scoped & Primitive

    [TypeKind.Struct]: Base<Kind.Type> & {
        typeKind: TypeKind.Struct
        fields: Field[]
    }

    [TypeKind.Sum]: Base<Kind.Type> & {
        typeKind: TypeKind.Sum
        fields: Field[]
    }

    [TypeKind.Unknown]: Scoped & Unknown

    [TypeKind.Variable]: Scoped & Variable

}[T]
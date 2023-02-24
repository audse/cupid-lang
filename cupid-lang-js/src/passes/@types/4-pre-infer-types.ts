import { Kind, TypeKind, Base as AstBase, Scoped, Ident, Literal, Primitive, Unknown, Variable } from '@/ast'


export type AnyTypeKind = Exclude<TypeKind, TypeKind.Instance>

export type Base<K extends Kind = Kind> = AstBase<K> & Scoped

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
        returns: Type
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
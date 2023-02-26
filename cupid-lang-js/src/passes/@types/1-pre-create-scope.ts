import { Kind, Base, Ident, Primitive, Literal, TypeKind, Unknown, LiteralValue } from '@/ast'


export type AnyTypeKind = Exclude<TypeKind, TypeKind.Variable>


export type Expr<K extends Kind = Kind, T extends AnyTypeKind = AnyTypeKind> = {

    [Kind.Assign]: Base<K> & {
        ident: Expr<Kind.Ident>
        value: Expr
    }

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

    [Kind.Ident]: Base<K> & {
        name: string
    }

    [Kind.IfStmt]: Base<K> & {
        condition: Expr
        body: Expr
        elseBody?: Expr
    }

    [Kind.Literal]: Base<K> & {
        value: LiteralValue
    }

    [Kind.Map]: Base<Kind.Map> & {
        entries: [Expr<Kind.Literal>, Expr][]
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

export type Type<T extends AnyTypeKind = AnyTypeKind> = {

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

    [TypeKind.Primitive]: Base<Kind.Type> & {
        typeKind: TypeKind.Primitive
        name: string
    }

    [TypeKind.Struct]: Base<Kind.Type> & {
        typeKind: TypeKind.Struct
        fields: Field[]
    }

    [TypeKind.Sum]: Base<Kind.Type> & {
        typeKind: TypeKind.Sum
        fields: Field[]
    }

    [TypeKind.Unknown]: Base<Kind.Type> & {
        typeKind: TypeKind.Unknown
    }

}[T]
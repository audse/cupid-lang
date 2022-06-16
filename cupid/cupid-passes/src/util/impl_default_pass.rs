
macro_rules! impl_default_passes {
    (   
        impl $_trait:ident + $_fn:ident for {
            $( Block<Expr> => $block_prev:ty; )?
            $( Expr => $expr_prev:ty; )?
            $( Field<Ident> => $field_prev:ty; )?
            $( Ident => $id:ty; )?
            $( IsTyped<Ident> => $typed_id:ty; )?
            $( Value => $val:ty; )?
        }
    ) => {
        // implement block
        $( 
            impl $_trait<crate::Block<Expr>> for crate::Block<$block_prev> {
                fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Block<Expr>> {
                    self.pass(Vec::<$block_prev>::$_fn, env)
                }
            }
        )?
        // implement expression
        $( 
            impl $_trait<Expr> for $expr_prev {
                fn $_fn(self, env: &mut crate::Env) -> PassResult<Expr> {
                    match self {
                        Self::Block(block) => Ok(Expr::Block(block.$_fn(env)?)),
                        Self::Decl(decl) => Ok(Expr::Decl(decl.$_fn(env)?)),
                        Self::Function(function) => Ok(Expr::Function(function.$_fn(env)?)),
                        Self::Ident(ident) => Ok(Expr::Ident(ident.$_fn(env)?)),
                        Self::TypeDef(type_def) => Ok(Expr::TypeDef(type_def.$_fn(env)?)),
                        Self::Value(value) => Ok(Expr::Value(value.$_fn(env)?))
                    }
                }
            }
        )?
        // implement field
        $(
            impl $_trait<crate::Field<crate::Ident>> for crate::Field<$field_prev> {
                fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Field<crate::Ident>> {
                    self.pass(<$field_prev>::$_fn, Option::<$field_prev>::$_fn, env)
                }
            }
        )?
        // implement ident
        $(
            impl $_trait<$id> for $id {
                fn $_fn(self, env: &mut crate::Env) -> PassResult<$id> { 
                    self.pass(Vec::<crate::IsTyped<Self>>::$_fn, Self::$_fn, env)
                }
            }
        )?
        // implement IsTyped<Ident>
        $(
            impl $_trait<$typed_id> for $typed_id {
                fn $_fn(self, env: &mut crate::Env) -> PassResult<$typed_id> { 
                    self.pass(Vec::<Self>::$_fn, crate::Ident::$_fn, env)
                 }
            }
        )?
        // implement value
        $(
            impl $_trait<$val> for $val {
                fn $_fn(self, _: &mut crate::Env) -> PassResult<$val> { Ok(self) }
            }
        )?
        impl $_trait<crate::Mut> for crate::Mut {
            fn $_fn(self, _: &mut crate::Env) -> PassResult<crate::Mut> { Ok(self) }
        }
    };
}

pub(crate) use impl_default_passes;
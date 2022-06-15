
macro_rules! impl_default_passes {
    (   
        impl $_trait:ident + $_fn:ident for {
            $( Block<Expr> => $block_prev:ty; )?
            $( Expr => $expr_prev:ty; )?
            $( Field<Ident> => $field_prev:ty; )?
            $( Value => $val:ty; )?
        }
    ) => {
        // implement block
        $( 
            impl $_trait<crate::Block<Expr>> for crate::Block<$block_prev> {
                fn $_fn(self, env: &mut crate::env::environment::Env) -> PassResult<crate::Block<Expr>> {
                    self.pass(Vec::<$block_prev>::$_fn, env)
                }
            }
        )?
        // implement expression
        $( 
            impl $_trait<Expr> for $expr_prev {
                fn $_fn(self, env: &mut crate::env::environment::Env) -> PassResult<Expr> {
                    match self {
                        Self::Block(block) => Ok(Expr::Block(block.$_fn(env)?)),
                        Self::Decl(decl) => Ok(Expr::Decl(decl.$_fn(env)?)),
                        Self::Function(function) => Ok(Expr::Function(function.$_fn(env)?)),
                        Self::Ident(ident) => Ok(Expr::Ident(ident.$_fn(env)?)),
                        Self::TypeDef(type_def) => Ok(Expr::TypeDef(type_def.$_fn(env)?)),
                        Self::Value(value) => Ok(Expr::Value(value.$_fn(env)?)),
                        _ => todo!()
                    }
                }
            }
        )?
        // implement value
        $(
            impl $_trait<$val> for $val {
                fn $_fn(self, _: &mut crate::env::environment::Env) -> PassResult<$val> { Ok(self) }
            }
        )?
        // implement field
        $(
            impl $_trait<crate::Field<Ident>> for crate::Field<$field_prev> {
                fn $_fn(self, env: &mut crate::env::environment::Env) -> PassResult<crate::Field<Ident>> {
                    self.pass(<$field_prev>::$_fn, Option::<$field_prev>::$_fn, env)
                }
            }
        )?
    };
}

pub(crate) use impl_default_passes;
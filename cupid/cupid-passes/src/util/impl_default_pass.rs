
macro_rules! impl_default_passes {
    (   
        impl $_trait:ident + $_fn:ident for {
            $( Block<Expr> => Block<$block_prev:ty>; )?
            $( Expr => $expr_prev:ty; )?
            $( crate::$t:tt $(<$g:tt>)? ; )*
        }
    ) => {
        // implement block
        $( 
            impl $_trait<crate::Block<Expr>> for crate::Block<$block_prev> {
                #[trace::trace]
                fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Block<Expr>> {
                    self.pass(Vec::<$block_prev>::$_fn, env)
                }
            }
        )?
        // implement expression
        $( 
            impl $_trait<Expr> for $expr_prev {
                #[trace::trace]
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
        $( crate::util::impl_default_passes! { $t $(<$g>)?, $_trait, $_fn } )*
    };
    (Field<Address>, $_trait:ident, $_fn:ident) => {
        impl $_trait<crate::Field<crate::Address>> for crate::Field<crate::Address> {
            #[trace::trace]
            fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Field<crate::Address>> {
                Ok(self)
            }
        }
    };
    (Field<Ident>, $_trait:ident, $_fn:ident) => {
        impl $_trait<crate::Field<crate::Ident>> for crate::Field<crate::Ident> {
            #[trace::trace]
            fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Field<crate::Ident>> {
                self.pass(<crate::Ident>::$_fn, Option::<crate::Ident>::$_fn, env)
            }
        }
    };
    (Ident, $_trait:ident, $_fn:ident) => {
        impl $_trait<crate::Ident> for crate::Ident {
            #[trace::trace]
            fn $_fn(self, env: &mut crate::Env) -> PassResult<crate::Ident> { 
                self.pass(Vec::<Self>::$_fn, Self::$_fn, env)
            }
        }
    };
    (Value, $_trait:ident, $_fn:ident) => {
        impl $_trait<crate::Value> for crate::Value {
            #[trace::trace]
            fn $_fn(self, _: &mut crate::Env) -> PassResult<crate::Value> { Ok(self) }
        }
        impl $_trait<crate::Mut> for crate::Mut {
            fn $_fn(self, _: &mut crate::Env) -> PassResult<crate::Mut> { Ok(self) }
        }
    };
    (Mut, $_trait:ident, $_fn:ident) => {
        impl $_trait<crate::Mut> for crate::Mut {
            fn $_fn(self, _: &mut crate::Env) -> PassResult<crate::Mut> { Ok(self) }
        }
    };
    () => {};
}

pub(crate) use impl_default_passes;
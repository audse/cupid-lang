#[macro_export]
macro_rules! auto_impl {
    (
        $v:vis trait $name:ident<'src> where Self: Sized $(,)? {
            fn $fn_name:ident(self $(, $param:ident: $param_ty:ty)*) -> Result<Self, CupidError>;
        }
    ) => {
        $v trait $name<'src> where Self: Sized {
            fn $fn_name(self $(, $param: $param_ty)*) -> Result<Self, CupidError>;
        }

        impl<'src, T: $name<'src>> $name<'src> for Vec<T> {
            fn $fn_name(self $(, $param: $param_ty)*) -> Result<Self, CupidError> {
                self.into_iter().map(|item| item.$fn_name( $($param),* )).collect()
            }
        }

        impl<'src, T: $name<'src>> $name<'src> for Box<T> {
            fn $fn_name(self $(, $param: $param_ty)*) -> Result<Self, CupidError> {
                Ok(Box::new((*self).$fn_name($($param),*)?))
            }
        }

        impl<'src, T: $name<'src>> $name<'src> for Option<T> {
            fn $fn_name(self $(, $param: $param_ty)*) -> Result<Self, CupidError> {
                match self {
                    Some(inner) => Ok(Some(inner.$fn_name($($param),*)?)),
                    None => Ok(None)
                }
            }
        }

        impl<'src> $name<'src> for Expr<'src> {
            fn $fn_name(self $(, $param: $param_ty)*) -> Result<Self, CupidError> {
                for_expr_variant!(self => |inner| Ok(inner.$fn_name($($param),*)?.into()))
            }
        }
    };
}

#[macro_export]
macro_rules! pass {
    ( Array::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.items = $self.items.$fn_name($ctx)?;
    };
    ( BinOp::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.left = $self.left.$fn_name($ctx)?;
        $self.right = $self.right.$fn_name($ctx)?;
    };
    ( Block::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.body = $self.body.$fn_name($ctx)?;
    };
    ( Break::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.value = $self.value.$fn_name($ctx)?;
    };
    ( Call::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.callee = $self.callee.$fn_name($ctx)?;
        $self.args = $self.args.$fn_name($ctx)?;
    };
    ( Class::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.fields = $self.fields.$fn_name($ctx)?;
        $self.methods = $self.methods.$fn_name($ctx)?;
    };
    ( Define::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.value = $self.value.$fn_name($ctx)?;
    };
    ( Fun::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.params = $self.params.$fn_name($ctx)?;
        $self.body = $self.body.$fn_name($ctx)?;
    };
    ( GetProperty::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.receiver = $self.receiver.$fn_name($ctx)?;
    };
    ( If::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.condition = $self.condition.$fn_name($ctx)?;
        $self.body = $self.body.$fn_name($ctx)?;
        $self.else_body = $self.else_body.$fn_name($ctx)?;
    };
    ( Invoke::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.receiver = $self.receiver.$fn_name($ctx)?;
        $self.args = $self.args.$fn_name($ctx)?;
    };
    ( InvokeSuper::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.args = $self.args.$fn_name($ctx)?;
    };
    ( Loop::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.body = $self.body.$fn_name($ctx)?;
    };
    ( Method::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.fun = $self.fun.$fn_name($ctx)?;
    };
    ( Return::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.value = $self.value.$fn_name($ctx)?;
    };
    ( Set::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.value = $self.value.$fn_name($ctx)?;
    };
    ( SetProperty::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.receiver = $self.receiver.$fn_name($ctx)?;
        $self.value = $self.value.$fn_name($ctx)?;
    };
    ( UnOp::$fn_name:ident($self:ident, $ctx:expr) ) => {
        $self.expr = $self.expr.$fn_name($ctx)?;
    };
}

#[macro_export]
macro_rules! base_pass {
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for { $( $t:tt ),*  $(,)? } ) => {
        $(
            base_pass! { impl $name::$fn_name($param: $param_ty) for $t }
        )*
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Array ) => {
        impl<'src> $name<'src> for Array<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Array::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for BinOp ) => {
        impl<'src> $name<'src> for BinOp<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(BinOp::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Block) => {
        impl<'src> $name<'src> for Block<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Block::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Break) => {
        impl<'src> $name<'src> for Break<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Break::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Call) => {
        impl<'src> $name<'src> for Call<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Call::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Class) => {
        impl<'src> $name<'src> for Class<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Class::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Constant) => {
        impl<'src> $name<'src> for Constant<'src> {
            fn $fn_name(self , $param: $param_ty) -> Result<Self, CupidError> {
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Define) => {
        impl<'src> $name<'src> for Define<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Define::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Fun) => {
        impl<'src> $name<'src> for Fun<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Fun::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Get) => {
        impl<'src> $name<'src> for Get<'src> {
            fn $fn_name(self , $param: $param_ty) -> Result<Self, CupidError> {
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for GetProperty) => {
        impl<'src> $name<'src> for GetProperty<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(GetProperty::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for GetSuper) => {
        impl<'src> $name<'src> for GetSuper<'src> {
            fn $fn_name(self , $param: $param_ty) -> Result<Self, CupidError> {
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for If) => {
        impl<'src> $name<'src> for If<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(If::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Invoke) => {
        impl<'src> $name<'src> for Invoke<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Invoke::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for InvokeSuper) => {
        impl<'src> $name<'src> for InvokeSuper<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(InvokeSuper::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Loop) => {
        impl<'src> $name<'src> for Loop<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Loop::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Method) => {
        impl<'src> $name<'src> for Method<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Method::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Return) => {
        impl<'src> $name<'src> for Return<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Return::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for Set) => {
        impl<'src> $name<'src> for Set<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(Set::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for SetProperty) => {
        impl<'src> $name<'src> for SetProperty<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(SetProperty::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
    ( impl $name:ident::$fn_name:ident($param:ident: $param_ty:ty) for UnOp) => {
        impl<'src> $name<'src> for UnOp<'src> {
            fn $fn_name(mut self , $param: $param_ty) -> Result<Self, CupidError> {
                pass!(UnOp::$fn_name(self, $param));
                Ok(self)
            }
        }
    };
}

use std::cell::{Ref, RefMut};

use crate::{cst::SourceId, for_expr_variant, pointer::Pointer, scope::Scope, ty::Type};

use super::Expr;

#[derive(Debug, Clone)]
pub struct ExprHeader<'src> {
    pub ty: Type<'src>,
    pub scope: Pointer<Scope<'src>>,
    pub source: SourceId,
}

pub trait Header<'src> {
    fn header(&self) -> &ExprHeader<'src>;
    fn header_mut(&mut self) -> &mut ExprHeader<'src>;
    fn scope(&self) -> Ref<Scope<'src>> {
        self.header().scope.borrow()
    }
    fn scope_mut(&mut self) -> RefMut<Scope<'src>> {
        self.header_mut().scope.borrow_mut()
    }
}

impl<'src> Header<'src> for Expr<'src> {
    fn header(&self) -> &ExprHeader<'src> {
        for_expr_variant!(self => |inner| inner.header())
    }
    fn header_mut(&mut self) -> &mut ExprHeader<'src> {
        for_expr_variant!(self => |inner| inner.header_mut())
    }
}

impl<'src, T: Header<'src>> Header<'src> for Box<T> {
    fn header(&self) -> &ExprHeader<'src> {
        (**self).header()
    }
    fn header_mut(&mut self) -> &mut ExprHeader<'src> {
        (**self).header_mut()
    }
}

#[macro_export]
macro_rules! with_header {
    (
        #[derive( $($der:ty),* )]
        $v:vis struct $name:ident<'src> {
            $( $field_v:vis $field_name:ident: $field_ty:ty ),* $(,)?
        }
    ) => {
        #[derive( $($der),* )]
        $v struct $name<'src> {
            pub header: ExprHeader<'src>,
            $( $field_v $field_name: $field_ty ),*
        }
        impl<'src> Header<'src> for $name<'src> {
            fn header(&self) -> &ExprHeader<'src> {
                &self.header
            }
            fn header_mut(&mut self) -> &mut ExprHeader<'src> {
                &mut self.header
            }
        }
    };
}

pub trait GetTy<'src> {
    fn ty(&self) -> Type<'src>;
    fn set_ty(&mut self, ty: Type<'src>);
}

impl<'src, T: GetTy<'src>> GetTy<'src> for Pointer<T> {
    fn ty(&self) -> Type<'src> {
        self.borrow().ty()
    }
    fn set_ty(&mut self, ty: Type<'src>) {
        self.borrow_mut().set_ty(ty)
    }
}

impl<'src, T: Header<'src>> GetTy<'src> for T {
    fn ty(&self) -> Type<'src> {
        self.header().ty
    }
    fn set_ty(&mut self, ty: Type<'src>) {
        self.header_mut().ty = ty;
    }
}

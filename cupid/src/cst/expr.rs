use super::{
    array::ArraySource, binop::BinOpSource, block::BlockSource, call::CallSource,
    class::ClassSource, constant::ConstantSource, define::DefineSource, fun::FunSource,
    get::GetSource, get_property::GetPropertySource, get_super::GetSuperSource,
    invoke::InvokeSource, invoke_super::InvokeSuperSource, r#break::BreakSource, r#if::IfSource,
    r#loop::LoopSource, r#return::ReturnSource, set::SetSource, set_property::SetPropertySource,
    unop::UnOpSource,
};

pub trait UnwrapEnum<T> {
    fn unwrapped(&self) -> &T;
}

macro_rules! unwrappable {
    (

        #[derive(Debug)]
        pub enum $name:ident <'src> {
            $( $variant:ident ( $inner:ty ) ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub enum $name <'src> {
            $( $variant ($inner) ),*
        }
        $(
            impl<'src> UnwrapEnum<$inner> for $name<'src> {
                fn unwrapped(&self) -> &$inner {
                    match self {
                        Self::$variant(inner) => inner,
                        _ => panic!(
                            "Expected enum variant {}::{}, instead found {:#?}",
                            stringify!($name),
                            stringify!($inner),
                            self
                        )
                    }
                }
            }
        )*
    }
}

unwrappable! {
    #[derive(Debug)]
    pub enum ExprSource<'src> {
        Array(ArraySource<'src>),
        BinOp(BinOpSource<'src>),
        Block(BlockSource<'src>),
        Break(BreakSource<'src>),
        Call(CallSource<'src>),
        Class(ClassSource<'src>),
        Constant(ConstantSource<'src>),
        Define(DefineSource<'src>),
        Fun(FunSource<'src>),
        GetProperty(GetPropertySource<'src>),
        GetSuper(GetSuperSource<'src>),
        Get(GetSource<'src>),
        If(IfSource<'src>),
        InvokeSuper(InvokeSuperSource<'src>),
        Invoke(InvokeSource<'src>),
        Loop(LoopSource<'src>),
        Return(ReturnSource<'src>),
        SetProperty(SetPropertySource<'src>),
        Set(SetSource<'src>),
        UnOp(UnOpSource<'src>),
    }
}

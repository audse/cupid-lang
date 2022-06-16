macro_rules! make_pass_node {
    ($_:ident::$($tail:tt)*) => {
        crate::util::make_pass_node! { $($tail)* }
    };
    (Decl) => {
        cupid_util::node_builder! {
            #[derive(Debug, Default, Clone)]
            pub DeclBuilder => pub Decl {
                pub ident: crate::Ident,
                pub type_annotation: Option<crate::Ident>,
                pub value: Box<Expr>,
                pub mutable: crate::Mut,
            }
        }
    };
    (Function) => {
        cupid_util::node_builder! {
            #[derive(Debug, Default, Clone)]
            pub FunctionBuilder => pub Function {
                pub params: Vec<Decl>,
                pub return_type_annotation: Option<crate::Ident>,
                pub body: Vec<Expr>,
            }
        }
    };
    (TypeDef) => {
        cupid_util::node_builder! {
            #[derive(Debug, Default, Clone)]
            pub TypeDefBuilder => pub TypeDef {
                pub ident: crate::Ident,
                pub fields: Vec<crate::Field<crate::Ident>>,
            }
        }
    };
    () => {};
}

macro_rules! define_pass_method {
    ($node:ident + $_fn:ident { $( $field:ident $(.$methods:ident())* ),* } ) => {
        fn pass(self, env: &mut crate::Env) -> crate::PassResult<$node> {
            let Self { $($field),*  , attr, ..} = self;
            Ok($node::build()
                .attr(attr)
                $( .$field($field.$_fn(env)? $(.$methods())*) )*
                .build())
        }
    };
}

/// Creates a `pass` method on a given struct. This method builds an instance of the current
/// passes' struct, by calling a given function on each field.
macro_rules! make_pass_method {
    ($_:ident::$($tail:tt)*) => {
        crate::util::make_pass_method! { $($tail)* }
    };
    (Decl => $_fn:ident) => {
        crate::util::define_pass_method! { Decl + $_fn { ident, type_annotation, mutable, value }}
    };
    (Function => $_fn:ident) => {
        crate::util::define_pass_method! { Function + $_fn { params, body, return_type_annotation }}
    };
    (TypeDef => $_fn:ident) => {
        crate::util::define_pass_method! { TypeDef + $_fn { ident, fields }}
    };
    () => {};
}

/// Defines a given struct in the caller's module, with the same structure
/// as the pre-analysis version of that struct. This enables us to skip passes
/// by just reusing the same nodes.
/// 
/// A function name can be provided to create a `pass` method for
/// this struct.
/// 
/// A trait impl signature can be provided to create both a `pass` method for 
/// this struct, and an implementation of the current pass.
/// TODO allow not just pre_analysis

macro_rules! reuse_node {
    // reuse_node! { pre_analysis::Decl }
    ($from:ty) => { crate::util::make_pass_node! { $from } };

    // reuse_node! { pre_analysis::Decl => Pass<analyze_scope> }
    ($from:ident$(::$tail:ident)* => Pass<$_fn:ident>) => {
        crate::util::make_pass_node! { $from$(::$tail)* }
        impl $from$(::$tail)* {
            crate::util::make_pass_method! { $from$(::$tail)* => $_fn }
        }
    };

    // reuse_node! { pre_analysis::Decl => AnalyzeScope<analyze_scope>  }
    ($from:ident$(::$tail:ident)* => $_trait:ident<$to:ty, $_fn:ident>) => {
        crate::util::make_pass_node! { $from$(::$tail)* }
        impl $from$(::$tail)* {
            crate::util::make_pass_method! { $from$(::$tail)* => $_fn }
        }
        impl $_trait<$to> for $from$(::$tail)* {
            fn $_fn(self, env: &mut crate::Env) -> crate::PassResult<$to> {
                self.pass(env)
            }
        }
    }
}

pub (crate) use { 
    make_pass_node, 
    make_pass_method, 
    define_pass_method,
    reuse_node,
};

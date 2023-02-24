use crate::{for_expr, for_stmt, map_expr, map_stmt, Error};
use cupid_ast::{attr::GetAttr, expr, stmt, types};
use cupid_env::{environment::Env, expr_closure::ExprClosure};
use cupid_util::InvertOption;
use std::{cell::RefCell, rc::Rc};

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait AnalyzeScope
where
    Self: Sized,
{
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

pub trait CreateScope
where
    Self: Sized + GetAttr,
{
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        env.add_closure(self.attr().source, parent)
    }
}

impl<T: CreateScope> CreateScope for std::cell::RefCell<T> {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        self.borrow().create_scope(parent, env)
    }
}

impl AnalyzeScope for expr::Expr {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.analyze_scope(env)?)
    }
}

impl CreateScope for expr::Expr {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        if let expr::Expr::Empty = self {
            parent.unwrap()
        } else {
            for_expr!(self => |expr| expr.create_scope(parent, env))
        }
    }
}

impl AnalyzeScope for stmt::Stmt {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.analyze_scope(env)?)
    }
}

impl CreateScope for stmt::Stmt {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        for_stmt!(self => |stmt| stmt.create_scope(parent, env))
    }
}

impl AnalyzeScope for expr::block::Block {}

impl CreateScope for expr::block::Block {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let block = env.add_closure(self.attr.source, parent);
        for expression in self.expressions.iter() {
            expression.create_scope(Some(block.clone()), env);
        }
        block
    }
}

impl AnalyzeScope for stmt::allocate::Allocation {}
impl CreateScope for stmt::allocate::Allocation {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        match self {
            Self::Expr(e) => e.create_scope(parent, env),
            Self::Type(t) => t.create_scope(parent, env),
            Self::Trait(t) => t.create_scope(parent, env),
        }
    }
}

impl AnalyzeScope for stmt::allocate::Allocate {}
impl CreateScope for stmt::allocate::Allocate {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.ident.create_scope(Some(scope.clone()), env);
        self.value.create_scope(Some(scope.clone()), env);
        scope
    }
}

impl AnalyzeScope for stmt::assign::Assign {}

impl CreateScope for stmt::assign::Assign {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        self.0.create_scope(parent, env)
    }
}

impl AnalyzeScope for stmt::decl::Decl {}

impl CreateScope for stmt::decl::Decl {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = self.allocate.create_scope(parent, env);
        if let Some(type_annotation) = &self.type_annotation {
            type_annotation.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for expr::function::Function {}

impl CreateScope for expr::function::Function {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.body.create_scope(Some(scope.clone()), env);
        for param in self.params.iter() {
            param.create_scope(Some(scope.clone()), env);
        }
        if let Some(ret_type) = &self.return_type_annotation {
            ret_type.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for expr::function_call::FunctionCall {}

impl CreateScope for expr::function_call::FunctionCall {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.function.create_scope(Some(scope.clone()), env);
        for arg in self.args.iter() {
            arg.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for expr::ident::Ident {}

impl CreateScope for expr::ident::Ident {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        env.add_closure(self.attr.source, parent)
    }
}

impl AnalyzeScope for expr::namespace::Namespace {}

impl CreateScope for expr::namespace::Namespace {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let namespace = env.add_closure(self.attr.source, parent);
        self.namespace.create_scope(Some(namespace.clone()), env);
        self.value.create_scope(Some(namespace.clone()), env);
        namespace
    }
}

impl AnalyzeScope for types::traits::Trait {}

impl CreateScope for types::traits::Trait {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.ident.create_scope(Some(scope.clone()), env);
        for method in self.methods.iter() {
            method.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for types::typ::Type {}

impl CreateScope for types::typ::Type {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.ident.create_scope(Some(scope.clone()), env);
        for field in self.fields.iter() {
            field.create_scope(Some(scope.clone()), env);
        }
        for method in self.methods.iter() {
            method.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for stmt::implement::Impl {}
impl CreateScope for stmt::implement::Impl {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        let scope = env.add_closure(self.attr.source, parent);
        self.trait_ident.create_scope(Some(scope.clone()), env);
        self.type_ident.create_scope(Some(scope.clone()), env);
        for method in self.methods.iter() {
            method.create_scope(Some(scope.clone()), env);
        }
        scope
    }
}

impl AnalyzeScope for stmt::trait_def::TraitDef {}

impl CreateScope for stmt::trait_def::TraitDef {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        self.0.create_scope(parent, env)
    }
}

impl AnalyzeScope for stmt::type_def::TypeDef {}

impl CreateScope for stmt::type_def::TypeDef {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        self.0.create_scope(parent, env)
    }
}

impl AnalyzeScope for expr::value::Value {}

impl CreateScope for expr::value::Value {
    fn create_scope(
        &self,
        parent: Option<Rc<RefCell<ExprClosure>>>,
        env: &mut Env,
    ) -> Rc<RefCell<ExprClosure>> {
        env.add_closure(self.attr.source, parent)
    }
}

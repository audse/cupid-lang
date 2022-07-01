use crate::{map_expr, map_stmt, Error};
use cupid_ast::{attr::Attr, expr, stmt, types};
use cupid_env::environment::{Context, Env};
use cupid_util::InvertOption;

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

impl AnalyzeScope for expr::Expr {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.analyze_scope(env)?)
    }
}

impl AnalyzeScope for stmt::Stmt {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.analyze_scope(env)?)
    }
}

impl AnalyzeScope for expr::block::Block {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let scope = env.scope.add_scope(Context::Block);
        env.inside_closure(scope, |env| {
            Ok(Self {
                expressions: self.expressions.analyze_scope(env)?,
                attr: Attr {
                    scope: env.scope.current(),
                    ..self.attr
                },
            })
        })
    }
}

impl AnalyzeScope for stmt::decl::Decl {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.analyze_scope(env)?,
            type_annotation: self.type_annotation.analyze_scope(env)?,
            value: self.value.analyze_scope(env)?,
            attr: Attr {
                scope: env.scope.current(),
                ..self.attr
            },
            ..self
        })
    }
}

impl AnalyzeScope for expr::function::Function {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let return_type_annotation = self.return_type_annotation.analyze_scope(env)?;
        let closure = env.scope.add_closure(Context::Function);
        env.inside_closure(closure, |env| {
            Ok(Self {
                params: self.params.analyze_scope(env)?,
                body: self.body.analyze_scope(env)?,
                return_type_annotation,
                attr: Attr {
                    scope: env.scope.current(),
                    ..self.attr
                },
            })
        })
    }
}

impl AnalyzeScope for expr::function_call::FunctionCall {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            function: self.function.analyze_scope(env)?,
            args: self.args.analyze_scope(env)?,
            attr: Attr { scope: env.scope.current(), ..self.attr }
        })
    }
}

impl AnalyzeScope for expr::ident::Ident {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        // let namespace = self.namespace.analyze_scope(env)?;
        // // use namespace's scope, if there is one. otherwise, current scope
        // let scope = namespace
        //     .as_ref()
        //     .map(|n| n.attr.scope)
        //     .unwrap_or_else(|| env.scope.current());
        Ok(Self {
            // namespace,
            generics: self.generics.analyze_scope(env)?,
            attr: Attr { scope: env.scope.current(), ..self.attr },
            ..self
        })
    }
}

impl AnalyzeScope for expr::namespace::Namespace {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            namespace: self.namespace.analyze_scope(env)?,
            value: self.value.analyze_scope(env)?,
            attr: Attr { scope: env.scope.current(), ..self.attr }
        })
    }
}

impl AnalyzeScope for types::traits::Trait {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let ident = self.ident.analyze_scope(env)?;
        let scope = env.scope.add_toplevel_closure(Context::Trait);
        env.inside_closure(scope, |env| {
            Ok(Self {
                ident,
                methods: self.methods.analyze_scope(env)?,
                attr: Attr { scope, ..self.attr },
                ..self
            })
        })
    }
}

impl AnalyzeScope for types::typ::Type {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let ident = self.ident.analyze_scope(env)?;
        let scope = env.scope.add_toplevel_closure(Context::Type);
        env.inside_closure(scope, |env| {
            Ok(Self {
                ident,
                fields: self.fields.analyze_scope(env)?,
                methods: self.methods.analyze_scope(env)?,
                attr: Attr { scope, ..self.attr },
                ..self
            })
        })
    }
}

impl AnalyzeScope for stmt::trait_def::TraitDef {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.analyze_scope(env)?,
            attr: Attr {
                scope: env.scope.current(),
                ..self.attr
            },
            ..self
        })
    }
}

impl AnalyzeScope for stmt::type_def::TypeDef {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.analyze_scope(env)?,
            attr: Attr {
                scope: env.scope.current(),
                ..self.attr
            },
            ..self
        })
    }
}

impl AnalyzeScope for expr::value::Value {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            attr: Attr {
                scope: env.scope.current(),
                ..self.attr
            },
            ..self
        })
    }
}

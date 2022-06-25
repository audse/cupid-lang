use crate::{map_expr, map_stmt, Error};
use cupid_ast::{attr::Attr, expr, stmt, types::traits::Trait};
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
        Ok(Self {
            expressions: self.expressions.analyze_scope(env)?,
            attr: Attr {
                scope: env.scope.current(),
                ..self.attr
            },
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
        })
    }
}

impl AnalyzeScope for expr::function::Function {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let closure = env.scope.add_closure(Context::Function);
        env.inside_closure(closure, |env| {
            Ok(Self {
                params: self.params.analyze_scope(env)?,
                body: self.body.analyze_scope(env)?,
                attr: Attr {
                    scope: env.scope.current(),
                    ..self.attr
                },
            })
        })
    }
}

impl AnalyzeScope for expr::ident::Ident {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let namespace = self.namespace.analyze_scope(env)?;
        // use namespace's scope, if there is one. otherwise, current scope
        let scope = namespace
            .as_ref()
            .map(|n| n.attr.scope)
            .unwrap_or_else(|| env.scope.current());
        Ok(Self {
            namespace,
            generics: self.generics.analyze_scope(env)?,
            attr: Attr { scope, ..self.attr },
            ..self
        })
    }
}

impl AnalyzeScope for stmt::trait_def::TraitDef {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let scope = env.scope.add_toplevel_closure(Context::Trait);
        env.inside_closure(scope, |env| {
            let methods: Result<Vec<(expr::ident::Ident, expr::function::Function)>, Error> = self
                .value
                .methods
                .into_iter()
                .map(|(ident, function)| {
                    Ok((ident.analyze_scope(env)?, function.analyze_scope(env)?))
                })
                .collect();
            Ok(Self {
                value: Trait {
                    methods: methods?,
                    ..self.value
                },
                attr: Attr { scope, ..self.attr },
                ..self
            })
        })
    }
}

impl AnalyzeScope for stmt::type_def::TypeDef {
    fn analyze_scope(self, env: &mut Env) -> Result<Self, Error> {
        let scope = env.scope.add_toplevel_closure(Context::Type);
        Ok(Self {
            attr: Attr { scope, ..self.attr },
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

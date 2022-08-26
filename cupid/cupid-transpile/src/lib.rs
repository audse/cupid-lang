#![feature(let_else)]
use cupid_ast::{expr, stmt};

pub trait Transpile {
    fn transpile(&self) -> String;
}

pub fn transpile_vec(items: &[impl Transpile]) -> Vec<String> {
    items
        .iter()
        .map(Transpile::transpile)
        .collect::<Vec<String>>()
}

impl Transpile for expr::Expr {
    fn transpile(&self) -> String {
        use expr::Expr::*;
        match self {
            Block(block) => block.transpile(),
            Function(function) => function.transpile(),
            FunctionCall(function_call) => function_call.transpile(),
            Ident(ident) => ident.transpile(),
            Namespace(namespace) => namespace.transpile(),
            Stmt(statement) => statement.transpile(),
            Value(value) => value.transpile(),
            _ => String::new(),
        }
    }
}

impl Transpile for expr::block::Block {
    fn transpile(&self) -> String {
        transpile_vec(&self.expressions).join("\n")
    }
}

impl Transpile for expr::function::Function {
    fn transpile(&self) -> String {
        transpile_as_closure(self)
    }
}

fn transpile_as_closure(function: &expr::function::Function) -> String {
    let Some((ret_expr, exprs)) = function.body.expressions.split_last() else { todo!() };
    let exprs = transpile_vec(exprs).join("\n");

    // arrow functions with braces are parsed as two blocks: arrow block and brace block
    // this leads to weird return statements
    let (ret_expr, exprs) = match ret_expr {
        expr::Expr::Stmt(stmt) => (stmt.transpile(), exprs),
        expr::Expr::Block(block) if function.body.expressions.len() == 1 => {
            let Some((ret_expr, exprs)) = block.expressions.split_last() else { todo!() };
            let exprs = transpile_vec(exprs).join("\n");
            match ret_expr {
                expr::Expr::Stmt(stmt) => (stmt.transpile(), exprs),
                _ => (format!("return {};", ret_expr.transpile()), exprs),
            }
        }
        _ => (format!("return {};", ret_expr.transpile()), exprs),
    };
    format!(
        "({}) => {{
            {exprs}
            {ret_expr}
        }}",
        function
            .params
            .iter()
            .map(transpile_as_param)
            .collect::<Vec<String>>()
            .join(", "),
    )
}

fn transpile_as_param(decl: &stmt::decl::Decl) -> String {
    let type_annotation: String = decl
        .type_annotation
        .as_ref()
        .map(|t| format!("/** @type {{ {} }} */", t.transpile()))
        .unwrap_or_default();
    match *(decl.value.borrow()) {
        expr::Expr::Empty => format!("{type_annotation} {}", decl.ident.transpile()),
        _ => format!(
            "{type_annotation} {} = {}",
            decl.ident.transpile(),
            decl.value.borrow().transpile()
        ),
    }
}

fn transpile_as_function(function: &expr::function::Function) -> String {
    let fun = transpile_as_closure(function);
    fun.replacen(" => ", " ", 1)
}

impl Transpile for expr::function_call::FunctionCall {
    fn transpile(&self) -> String {
        format!(
            "{ident}({args})",
            ident = self.function.transpile(),
            args = transpile_vec(&self.args).join(", ")
        )
    }
}

impl Transpile for expr::ident::Ident {
    fn transpile(&self) -> String {
        self.name.to_string() // TODO generics
    }
}

impl Transpile for expr::namespace::Namespace {
    fn transpile(&self) -> String {
        format!("{}({})", self.namespace.transpile(), self.value.transpile())
    }
}

impl Transpile for expr::value::Val {
    fn transpile(&self) -> String {
        use expr::value::Val::*;
        match self {
            VBoolean(true) => "true".to_string(),
            VBoolean(false) => "false".to_string(),
            VDecimal(a, b) => format!("{a}.{b}"),
            VInteger(i) => i.to_string(),
            VString(s) => s.to_string(),
            VChar(c) => c.to_string(),
            VNone => "null".to_string(),
        }
    }
}

impl Transpile for expr::value::Value {
    fn transpile(&self) -> String {
        self.inner.transpile()
    }
}

impl Transpile for stmt::Stmt {
    fn transpile(&self) -> String {
        use stmt::Stmt::*;
        match self {
            Assign(_) => String::new(),
            Decl(decl) => decl.transpile(),
            TraitDef(trait_def) => trait_def.transpile(),
            TypeDef(type_def) => type_def.transpile(),
        }
    }
}

impl<T: Transpile> Transpile for Option<T> {
    fn transpile(&self) -> String {
        self.as_ref().map(|s| s.transpile()).unwrap_or_default()
    }
}

impl Transpile for stmt::decl::Decl {
    fn transpile(&self) -> String {
        let type_annotation: String = self
            .type_annotation
            .as_ref()
            .map(|t| format!("/** @type {{ {} }} */", t.transpile()))
            .unwrap_or_default();
        match &*(self.value.borrow()) {
            expr::Expr::Function(function) => format!(
                "function {ident} {value}",
                ident = self.ident.transpile(),
                value = transpile_as_function(&function)
            ),
            _ => format!(
                "{type_annotation} {mutability} {ident} = {value};",
                mutability = self.mutable.transpile(),
                ident = self.ident.transpile(),
                value = self.value.borrow().transpile()
            ),
        }
    }
}

impl Transpile for stmt::decl::Mut {
    fn transpile(&self) -> String {
        match self {
            stmt::decl::Mut::Mutable => "let",
            stmt::decl::Mut::Immutable => "const",
        }
        .to_string()
    }
}

fn transpile_methods(methods: &[stmt::decl::Decl]) -> String {
    methods
        .iter()
        .map(|m| match &*(m.value.borrow()) {
            expr::Expr::Function(function) => format!(
                "function {ident} (obj, {params}) {{ \n {body} \n }}",
                ident = m.ident.transpile(),
                params = function
                    .params
                    .iter()
                    .map(transpile_as_param)
                    .collect::<Vec<String>>()
                    .join(", "),
                body = function.body.transpile()
            ),
            _ => todo!(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn transpile_methods_return(methods: &[stmt::decl::Decl]) -> String {
    format!(
        "return {{ \n {} \n }};",
        methods
            .iter()
            .map(|m| {
                let ident = m.ident.transpile();
                format!("{ident}: {ident}.bind(this, obj)")
            })
            .collect::<Vec<String>>()
            .join(",\n")
    )
}

impl Transpile for stmt::trait_def::TraitDef {
    fn transpile(&self) -> String {
        format!(
            "function {ident} (obj) {{
                {expressions}
                {returns}
            }}",
            ident = self.ident.transpile(),
            expressions = transpile_methods(&self.value.borrow().methods),
            returns = transpile_methods_return(&self.value.borrow().methods)
        )
    }
}

impl Transpile for stmt::type_def::TypeDef {
    fn transpile(&self) -> String {
        let ident = self.ident.transpile();
        format!(
            "/** @typedef {ident} */
            function type{ident} (obj) {{
                {expressions}
                {returns}
            }}",
            expressions = transpile_methods(&self.value.borrow().methods),
            returns = transpile_methods_return(&self.value.borrow().methods)
        )
    }
}

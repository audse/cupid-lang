pub mod analyze_scope;
pub mod resolve_type_names;
pub mod resolve_names;
pub mod infer_types;
pub mod check_types;
pub mod check_flow;
pub mod lint;
pub(self) mod utils;

pub struct Error;

macro_rules! map_expr {
    ($to:ident => |$exp:ident| $inside:expr) => {{
        use expr::Expr::*;
        match $to {
            Block($exp) => Ok(Block($inside)),
            Function($exp) => Ok(Function($inside)),
            Ident($exp) => Ok(Ident($inside)),
            Value($exp) => Ok(Value($inside)),
            Empty => Ok(Empty),
        }
    }};
}

macro_rules! map_stmt {
    ($to:ident => |$stm:ident| $inside:expr) => {{
        use stmt::Stmt::*;
        match $to {
            Decl($stm) => Ok(Decl($inside)),
            TraitDef($stm) => Ok(TraitDef($inside)),
            TypeDef($stm) => Ok(TypeDef($inside)),
        }
    }};
}

pub(crate) use {map_expr, map_stmt};
use cupid_ast::expr::value::Val;
use std::ops::Add;

struct OpVal(Val);

impl Add for OpVal {
    type Output = Result<Val, i32>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (Val::VInteger(x), Val::VInteger(y)) => Ok(Val::VInteger(x + y)),
            // (Val::VDecimal(a, b), Val::VDecimal(x, y) => Ok())
            _ => Err(0),
        }
    }
}

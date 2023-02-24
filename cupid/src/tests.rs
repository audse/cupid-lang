#![cfg(test)]
use cupid_ast::expr::Expr;
use cupid_debug::error::Error;
use cupid_env::environment::Env;
use cupid_lex::lexer::*;
use cupid_parse::parse::*;
use cupid_semantics::{
    analyze_scope::{AnalyzeScope, CreateScope},
    check_flow::CheckFlow,
    check_types::CheckTypes,
    infer_types::InferTypes,
    lint::Lint,
    resolve_names::ResolveNames,
    resolve_type_names::ResolveTypeNames,
};
use std::rc::Rc;

pub(self) type TestResult<T> = Result<T, Error>;

#[allow(unused)]
pub(self) fn compile(string: &str) -> TestResult<Vec<Expr>> {
    let mut parser = Parser::new_with(Rc::new(string.to_string()), Env::default());
    let mut parsed = parser.parse(Lexer::new().lex(string)).unwrap();
    let mut env = parser.env;
    for expr in parsed.iter() {
        expr.create_scope(env.get_closure(0), &mut env);
    }
    parsed
        // .resolve_packages(&mut env)?
        .analyze_scope(&mut env)?
        .resolve_type_names(&mut env)?
        .resolve_names(&mut env)?
        .infer_types(&mut env)?
        .check_types(&mut env)?
        .check_flow(&mut env)?
        .lint(&mut env)
}

mod test_ast_nodes {
    use super::{compile, TestResult};

    #[test]
    fn test_bin_op() -> TestResult<()> {
        compile(
            "
			trait equal (t) = [
				is: left : t, right : t => false
			]
			1 is 1 # produces 1.is(1)
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_type_def_1() -> TestResult<()> {
        compile("type int = []")?;
        Ok(())
    }

    #[test]
    fn test_type_def_2() -> TestResult<()> {
        compile(
            "
			type int = []
			type person = [
				age : int
			]
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_type_def_3() -> TestResult<()> {
        compile(
            "
			type int = []
			type dec = []
			sum number = [
				integer : int,
				decimal : dec
			]
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_trait_def() -> TestResult<()> {
        compile(
            r"
			type int = []
			trait add = [
				add_it: left : int, right : int => left # todo
			]
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_decl_1() -> TestResult<()> {
        compile(
            r"
			type int = []
			let x : int = 0
			x # ignore unused warning
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_assign() -> TestResult<()> {
        compile(
            r"
			type int = []
			let x : int = 0
			x = 1
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_decl_2() -> TestResult<()> {
        compile(
            r"
			type int = []
			let mut x : int = 0
			x # ignore unused warning
		",
        )?;
        Ok(())
    }

    #[test]
    fn test_fun() -> TestResult<()> {
        compile(
            r"
			type int = []
			let sq = num : int => { num }
			sq(1)
			",
        )?;
        Ok(())
    }
}

mod test_error_codes {
    use super::{compile, TestResult};
    use cupid_debug::code::ErrorCode::*;

    #[test] // 103
    #[ignore = "not yet implemented"]
    fn test_unclosed_delimiter() -> TestResult<()> {
        let result = compile("[").unwrap_err();
        assert!(result.code == UnclosedDelimiter);
        Ok(())
    }

    #[test] // 304
    fn test_unused_variable() -> TestResult<()> {
        let result = compile(
            r"
			type int = []
			let x : int = 1
			",
        )
        .unwrap_err();
        assert!(result.code == UnusedVariable);
        Ok(())
    }

    #[test] // 400
    fn test_type_mismatch_1() -> TestResult<()> {
        let result = compile(
            r"
			type int = []
			let sq = num : int => { num }
			sq(true)
			",
        )
        .unwrap_err();
        assert!(result.code == TypeMismatch);
        Ok(())
    }

    #[test]
    fn test_type_mismatch_2() -> TestResult<()> {
        let result = compile(
            r"
			type int = []
			let x : int = 0
			x = false
		",
        )
        .unwrap_err();
        assert!(result.code == TypeMismatch);
        Ok(())
    }

    #[test] // 401
    #[ignore = "need to find a good test case"]
    fn test_cannot_infer() -> TestResult<()> {
        todo!()
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_out_of_scope() -> TestResult<()> {
        todo!()
    }

    #[test] // 404
    fn test_not_found_1() -> TestResult<()> {
        let result = compile(r"let x : int = 1").unwrap_err();
        assert!(result.code == NotFound);
        Ok(())
    }

    #[test] // 404
    fn test_not_found_2() -> TestResult<()> {
        let result = compile(
            r"
			let x : int = 1
			x
			{
				let y : int = 2
			}
			y
			",
        )
        .unwrap_err();
        assert!(result.code == NotFound);
        Ok(())
    }

    #[test] // 404
    fn test_not_found_3() -> TestResult<()> {
        let result = compile(
            r"
			type int = []
			let sq = num : int => { num }
			num
			",
        )
        .unwrap_err();
        assert!(result.code == NotFound);
        Ok(())
    }

    #[test] // 405
    #[ignore = "not yet implemented"]
    fn test_cannot_access() -> TestResult<()> {
        todo!()
    }

    #[test] // 406
    #[ignore = "variable shadowing is currently allowed"]
    fn test_already_defined() -> TestResult<()> {
        todo!()
    }

    #[test] // 409
    #[ignore = "type unification is not yet finished"]
    fn test_cannot_unify() -> TestResult<()> {
        todo!()
    }

    #[test] // 417
    fn test_expected_type() -> TestResult<()> {
        let result = compile(
            "
			type int = []
			let x : int = 1
			let y : x = 2
		",
        )
        .unwrap_err();
        assert!(result.code == ExpectedType);
        Ok(())
    }

    #[test] // 418
    #[ignore = "not yet implemented"]
    fn test_unexpected_type() -> TestResult<()> {
        todo!()
    }

    #[test] // 419
    #[ignore = "need to find a good test case"]
    fn test_expected_function() -> TestResult<()> {
        todo!()
    }

    #[test] // 420
    #[ignore = "not yet implemented"]
    fn test_expected_trait() -> TestResult<()> {
        todo!()
    }

    #[test] // 421
    #[ignore = "not yet implemented"]
    fn test_expected_expression() -> TestResult<()> {
        todo!()
    }

    #[test] // 422
    fn test_too_many_args() -> TestResult<()> {
        let result = compile(
            "
			type int = []
			let sq = num : int => { num }
			sq(1, 2, 3)
		",
        )
        .unwrap_err();
        assert!(result.code == TooManyArgs);
        Ok(())
    }

    #[test] // 423
    fn test_not_enough_args() -> TestResult<()> {
        let result = compile(
            "
			type int = []
			let sq = num : int => { num }
			sq()
		",
        )
        .unwrap_err();
        assert!(result.code == NotEnoughArgs);
        Ok(())
    }
}

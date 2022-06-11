/// This module includes helper macro functions to quickly build
/// AST nodes. The goal is to make testing a little simpler
/// and more readable.

#[macro_export]
macro_rules! primitive {
	($name:tt) => { cupid_ast::Type::build().primitive($name).build() }
}

#[macro_export]
macro_rules! array {
	($name:tt) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("e"))
			.fields(cupid_builder::fields!("element_type" => "e"))
			.base_type(cupid_ast::BaseType::Array)
			.build()
	};
	($element_type:expr) => { cupid_builder::array!("array", $element_type) };
	($name:tt, $element_type:expr) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("e"))
			.fields(cupid_builder::fields!("element_type": $element_type))
			.base_type(cupid_ast::BaseType::Array)
			.build()
	};
	() => { cupid_builder::array!("array") };
}

#[macro_export]
macro_rules! generics {
	( $( $name:tt ),* ) => {
		cupid_ast::GenericList(vec![
			$( (cupid_ast::Untyped($name.into())) ),*
		])
	};
}

#[macro_export]
macro_rules! fields {
	( $( $name:tt => $val:tt ),* ) => {
		cupid_ast::FieldSet(vec![
			$( cupid_ast::Field {
				name: $name.into(),
				type_hint: Some(cupid_ast::Untyped($val.into()))
			} ),*
		])
	};
	( $( $name:tt : $val:expr ),* ) => {
		cupid_ast::FieldSet(vec![
			$( cupid_ast::Field {
				name: $name.into(),
				type_hint: Some($val)
			} ),*
		])
	};
	( $( $name:tt ),* ) => {
		cupid_ast::FieldSet(vec![
			$( cupid_ast::Field {
				name: $name.into(),
				type_hint: None
			} ),*
		])
	};
}
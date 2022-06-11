/// This module includes helper macro functions to quickly build
/// AST nodes. The goal is to make testing a little simpler
/// and more readable.

#[macro_export]
macro_rules! primitive {
	($name:tt) => { cupid_ast::Type::build().primitive($name).build() }
}

#[macro_export]
macro_rules! ident {
	($name:tt) => { cupid_ast::Ident::new_name($name) }
}

#[macro_export]
macro_rules! array {
	($name:tt) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("e"))
			.fields(cupid_builder::fields!("element_type" => "e"))
			.base_array()
			.build()
	};
	($element_type:expr) => { cupid_builder::array!("array", $element_type) };
	($name:tt, $element_type:expr) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("e"))
			.fields(cupid_builder::fields!("element_type": $element_type))
			.base_array()
			.build()
	};
	() => { cupid_builder::array!("array") };
}

#[macro_export]
macro_rules! map {
	($name:tt) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("k", "v"))
			.fields(cupid_builder::fields!("key_type" => "k", "val_type" => "v"))
			.base_array()
			.build()
	};
	($key_type:expr, $val_type:expr) => { cupid_builder::map!("map", $key_type, $val_type) };
	($name:tt, $key_type:expr, $val_type:expr) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!("k", "v"))
			.fields(cupid_builder::fields!("key_type": $key_type, "val_type": $val_type))
			.base_array()
			.build()
	};
	() => { cupid_builder::map!("map") };
}

#[macro_export]
macro_rules! build {
	( $name:tt $generics:tt = $fields:tt ) => {
		cupid_ast::Type::build()
			.name_str($name)
			.generics(cupid_builder::generics!$generics)
			.fields(cupid_builder::fields!$fields)
			.build()
	}
}

#[macro_export]
macro_rules! generics {
	( $( $name:tt ),* ) => {
		cupid_ast::GenericList(vec![
			$( (cupid_ast::Untyped($name.into())) ),*
		])
	};
	( $( $types:expr ),* ) => {
		cupid_ast::GenericList(vec![
			$( $types ),*
		])
	};
	( $( $name:tt : $ty:expr ),* ) => {
		cupid_ast::GenericList(vec![
			$( (cupid_ast::IsTyped($name.into(), $ty)) ),*
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
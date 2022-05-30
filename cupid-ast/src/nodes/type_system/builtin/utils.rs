
#[macro_export]
macro_rules! generics {
	// Creates an instance of `GenericList` from a &str identifier 
	// and an optional &str value
	($($g:tt),*) => { GenericList::from(vec![$($g),*]) };
	($($g:tt: $v:tt),*) => { GenericList(vec![
		$(Untyped(Ident::new_name($v))),*
	])}
}

#[macro_export]
macro_rules! fields {
	// Creates an instance of `FieldSet` from a list of `&str` identifiers
	// e.g. fields!["a", ..] => FieldSet::Unnamed(Str(a), ..)
	// fields!["a": "b", ..] => FieldSet::Named((Str(a), TypeIdent(b)), ..)
	($($f:tt),*) => { 
		FieldSet::Unnamed(vec![ $( Untyped(primitive($f).into_ident()) ),* ])
	};
	($($name:tt: $f:tt),*) => {
		FieldSet::Named(vec![ $( 
			(Cow::Borrowed($f), Untyped(primitive($f).into_ident())) 
		),* ])
	};
}

#[macro_export]
macro_rules! traits {
	// Turns a list of trait constants into owned identifiers
	// E.g. [EQUAL, ..] => [Ident(equal!), ..]
	($($t:ident),*) => { vec![$($t.into_ident()),*] }
}
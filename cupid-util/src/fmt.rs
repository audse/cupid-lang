#[macro_export]
macro_rules! fmt_list {
	($list:expr) => { $list.iter().map(|x| x.to_string()).collect::<Vec<String>>() };
	($list:expr, $sep:tt) => { $list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join($sep) };
	($list:expr, $closure:expr) => { $list.iter().map($closure).collect::<Vec<String>>() };
	($list:expr, $closure:expr, $sep:tt) => { $list.iter().map($closure).collect::<Vec<String>>().join($sep) };
}

#[macro_export]
macro_rules! fmt_option {
	($option:expr) => { 
		if let Some(x) = $option { 
			x.to_string()
		} else { 
			String::new() 
		}
	};
	($option:expr, |$some:ident| $closure:expr) => { 
		if let Some($some) = $option { $closure } else { String::new() }
	};
}

#[macro_export]
macro_rules! fmt_if_nonempty {
	($list:expr, $closure:expr) => {
		if $list.is_empty() { String::new() } else { $closure }
	}
}

#[macro_export]
macro_rules! fmt_option_fn {
	($($name:ident: $t:ty),*) => {
		$(
			fn $name(x: &Option<$t>) -> String {
				fmt_option!(x)
			}
		)*
	}
}

#[macro_export]
macro_rules! log {
	
	( $( $(@debug=$debug:tt)? $(@pretty=$pretty:tt)? $e:expr ),* ) => {{
		$(

			print!("{}", quick_fmt_str!($(@debug=$debug)? $(@pretty=$pretty)? $e));
		)*
	}};
}

#[macro_export]
macro_rules! quick_fmt {
	
	( $( $(@debug=$debug:tt)? $(@pretty=$pretty:tt)? $e:expr ),* ) => {{
		let mut string = String::new();
		$(

			string += &quick_fmt_str!($(@debug=$debug)? $(@pretty=$pretty)? $e);
		)*
		string
	}};
}

#[macro_export]
macro_rules! quick_fmt_str {
	( @debug=$debug:tt @pretty=$pretty:tt $e:expr ) => {
		if $pretty {
			format!("{:#?}", $e)
		} else if $debug {
			format!("{:?}", $e)
		} else {
			format!("{}", $e)
		}
	};
	( @debug=$debug:tt $e:expr ) => {
		if $debug {
			format!("{:?}", $e)
		} else {
			format!("{}", $e)
		}
	};
	( @pretty=$pretty:tt $e:expr ) => {
		if $pretty {
			format!("{:#?}", $e)
		} else {
			format!("{:?}", $e)
		}
	};
	( $e:expr ) => {
		format!("{}", $e)
	};
}
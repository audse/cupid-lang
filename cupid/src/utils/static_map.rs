#[allow(unused_macros)]
macro_rules! StaticMap {
	($v:tt static $name:ident: $type_args:ty = { $($key:ident: $value:expr),* }) => {
		lazy_static! {
			$v static ref $name: $type_args = {
				let mut map = HashMap::new();
				$( map.insert(stringify!($key), $value); )*
				map
			};
		}
	};
	(static $name:ident: $type_args:ty = { $($key:ident: $value:expr),* }) => {
		lazy_static! {
			static ref $name: $type_args = {
				let mut map = HashMap::new();
				$( map.insert(stringify!($key), $value); )*
				map
			};
		}
	};
	($v:tt static $name:ident: $type_args:ty = { $($key:tt: $value:expr),* }) => {
		lazy_static! {
			$v static ref $name: $type_args = {
				let mut map = HashMap::new();
				$( map.insert($key, $value); )*
				map
			};
		}
	};
	(static $name:ident: $type_args:ty = { $($key:tt: $value:expr),* }) => {
		lazy_static! {
			static ref $name: $type_args = {
				let mut map = HashMap::new();
				$( map.insert($key, $value); )*
				map
			};
		}
	};
}

#[allow(unused_imports)]
pub(crate) use StaticMap;
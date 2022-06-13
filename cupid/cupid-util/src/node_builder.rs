
/// Constructs a struct and a builder struct with additional fields
/// present on all AST nodes.
/// ## Example
/// ```
/// cupid_util::node_builder! {
///     #[derive(Debug)]
///     PersonABuilder => PersonA {
///         name: &'static str,
///         age: usize,
///     }
/// }
/// // is equivalent to
/// struct PersonB {
///     name: &'static str,
///     age: usize,
///     source: usize,
///     closure: usize,
///     typ: usize,
/// }
/// struct PersonBBuilder {
///     name: &'static str,
///     age: usize,
///     source: usize,
///     closure: usize,
///     typ: usize,
/// }
/// // plus a few `impl` blocks
/// ```
/// ### Notes
/// 1. Generic inline trait bounds work, `where` clauses do not
/// 2. The visibility of the struct is applied to all extra generated functions
/// 3. Only works with named fields, no tuple structs
#[macro_export]
macro_rules! node_builder {
	( 
		$(#[$derive:meta])?
		$bv:vis $builder_name:ident => $v:vis 
		$struct_name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
		
		{
			$( 
				$(#[$fderive:meta])?
				$fv:vis $field:ident : $t:ty
			),* $(,)?
		}
	) => {
		$(#[$derive])?
		$v struct $struct_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
            $v source: usize,
            $v closure: usize,
            $v typ: usize,
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}
		$(#[$derive])?
		$bv struct $builder_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?  {
            $v source: usize,
            $v closure: usize,
            $v typ: usize,
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}

		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $struct_name $(< $( $lt ),+ >)? {
			pub fn build() -> $builder_name $(< $( $lt ),+ >)? {
				$builder_name::new()
			}
			$v fn builder(self) -> $builder_name $(< $( $lt ),+ >)? {
				$builder_name$(::< $( $lt ),+ >)?::from(self)
			}
		}
		
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $builder_name $(< $( $lt ),+ >)? {
			$v fn new() -> Self {
				Self::default()
			}
			$v fn build(self) -> $struct_name $(< $( $lt ),+ >)? {
				$struct_name {
					source: self.source,
					closure: self.closure,
					typ: self.typ,
					$( $field: self.$field ),*
				}
			}
			$(
				$fv fn $field(mut self, val: $t) -> Self {
					self.$field = val;
					self
				}
			)*
            $v fn source(mut self, src: usize) -> Self {
                self.source = src;
                self
            }
            $v fn closure(mut self, closure: usize) -> Self {
                self.closure = closure;
                self
            }
            $v fn typ(mut self, typ: usize) -> Self {
                self.typ = typ;
                self
            }
			$v fn meta(self, src: usize, closure: usize, typ: usize) -> Self {
				self.source(src)
					.closure(closure)
					.typ(typ)
			}
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<$struct_name$(< $( $lt ),+ >)?> for $builder_name $(< $( $lt ),+ >)? {
			fn from(s: $struct_name $(< $( $lt ),+ >)? ) -> Self {
				$builder_name$(::< $( $lt ),+ >)?  {
					source: s.source,
					closure: s.closure,
					typ: s.typ,
					$( $field: s.$field ),*
				}
			}
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<&$builder_name$(< $( $lt ),+ >)?> for $struct_name $(< $( $lt ),+ >)? {
			fn from(s: &$builder_name $(< $( $lt ),+ >)? ) -> Self {
				$struct_name$(::< $( $lt ),+ >)? {
					source: s.source,
					closure: s.closure,
					typ: s.typ,
					$( $field: s.$field.to_owned() ),*
				}
			}
		}

		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? crate::AsNode for $struct_name$(< $( $lt ),+ >)? {
			fn source(&self) -> usize { self.source }
			fn closure(&self) -> usize { self.closure }
			fn typ(&self) -> usize { self.typ }
		}
	};
}
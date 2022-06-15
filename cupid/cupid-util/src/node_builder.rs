
/// Constructs a struct and a builder struct with additional fields
/// present on all AST nodes.
/// # Examples
/// ```no_run
/// use crate::{node_builder, placeholder};
/// use cupid_passes::{pre_analysis::Expr, AsNode, Attributes};
/// 
/// // Creating a struct + builder like this:
/// 
/// node_builder! {
///     #[derive(Debug)]
///     OperationBuilder => Operation {
///         left: Expr,
///         right: Expr,
///         operator: String,
///     }
/// }
/// 
/// // Outputs this:
/// 
/// struct Operation {
///     left: Expr,
///     right: Expr,
///     operator: String,
///     attr: Attributes,
/// }
/// struct OperationBuilder {
///     left: Expr,
///     right: Expr,
///     operator: String,
///     attr: Attributes,
/// }
/// 
/// impl From<OperationBuilder> for Operation { placeholder!(...) }
/// impl From<Operation> for OperationBuilder { placeholder!(...) }
/// impl AsNode for Operation { placeholder!(...) }
/// impl Operation { placeholder!(...) }
/// impl OperationBuilder { placeholder!(...) }
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
			$v attr: crate::Attributes,
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}
		$(#[$derive])?
		$bv struct $builder_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?  {
			$v attr: crate::Attributes,
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
            $v fn build_attr(mut self, attr: crate::Attributes) -> Self {
                self.attr = attr;
                self
            }
            $v fn build_source(mut self, src: usize) -> Self {
                self.attr.source = src;
                self
            }
            $v fn build_scope(mut self, scope: (usize, usize)) -> Self {
                self.attr.scope = scope;
                self
            }
            $v fn build_typ(mut self, typ: usize) -> Self {
                self.attr.typ = typ;
                self
            }
		}
		
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $builder_name $(< $( $lt ),+ >)? {
			$v fn new() -> Self {
				Self::default()
			}
			$v fn build(self) -> $struct_name $(< $( $lt ),+ >)? {
				$struct_name {
					attr: self.attr,
					$( $field: self.$field ),*
				}
			}
			$(
				$fv fn $field(mut self, val: $t) -> Self {
					self.$field = val;
					self
				}
			)*
            $v fn attr(mut self, attr: crate::Attributes) -> Self {
                self.attr = attr;
                self
            }
            $v fn source(mut self, src: usize) -> Self {
                self.attr.source = src;
                self
            }
            $v fn scope(mut self, scope: (usize, usize)) -> Self {
                self.attr.scope = scope;
                self
            }
            $v fn typ(mut self, typ: usize) -> Self {
                self.attr.typ = typ;
                self
            }
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<$struct_name$(< $( $lt ),+ >)?> for $builder_name $(< $( $lt ),+ >)? {
			fn from(s: $struct_name $(< $( $lt ),+ >)? ) -> Self {
				$builder_name$(::< $( $lt ),+ >)?  {
					attr: s.attr,
					$( $field: s.$field ),*
				}
			}
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<&$builder_name$(< $( $lt ),+ >)?> for $struct_name $(< $( $lt ),+ >)? {
			fn from(s: &$builder_name $(< $( $lt ),+ >)? ) -> Self {
				$struct_name$(::< $( $lt ),+ >)? {
					attr: s.attr,
					$( $field: s.$field.to_owned() ),*
				}
			}
		}

		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? crate::AsNode for $struct_name$(< $( $lt ),+ >)? {
			fn source(&self) -> usize { self.attr.source }
			fn scope(&self) -> (usize, usize) { self.attr.scope }
			fn typ(&self) -> usize { self.attr.typ }
			fn set_source(&mut self, source: usize) { self.attr.source = source; }
			fn set_scope(&mut self, scope: (usize, usize)) { self.attr.scope = scope; }
			fn set_typ(&mut self, typ: usize) { self.attr.typ = typ; }
		}
	};
}
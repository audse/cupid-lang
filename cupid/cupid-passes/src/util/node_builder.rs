
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
            $v fn build_address(mut self, address: crate::Address) -> Self {
                self.attr.address = address;
                self
            }
            $v fn build_scope(mut self, scope: crate::ScopeId) -> Self {
                self.attr.scope = scope;
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
			fn address(&self) -> crate::Address { self.attr.address }
			fn scope(&self) -> crate::ScopeId { self.attr.scope }
			fn set_scope(&mut self, scope: crate::ScopeId) { self.attr.scope = scope; }
		}
	};
}

pub(crate) use node_builder;
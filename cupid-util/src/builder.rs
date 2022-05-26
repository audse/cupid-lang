// Requires impl ToOwned + derive Default

#[macro_export]
macro_rules! build_struct {
	( 
		$(#[$derive:meta])?
		$bv:vis $builder_name:ident => $v:vis $struct_name:ident {
		$( $fv:vis $field:ident: $t:ty ),* $(,)?
	}) => {
		$(#[$derive])?
		$v struct $struct_name {
			$( $fv $field: $t ),*
		}
		$(#[$derive])?
		$bv struct $builder_name {
			$( $fv $field: $t ),*
		}

		impl $struct_name {
			pub fn build() -> $builder_name {
				$builder_name::new()
			}
		}
		
		impl $builder_name {
			pub fn new() -> Self {
				Self::default()
			}
			pub fn build(self) -> $struct_name {
				$struct_name {
					$( $field: self.$field ),*
				}
			}
			$(
				$bv fn $field(mut self, val: $t) -> Self {
					self.$field = val;
					self
				}
			)*
		}
			
		impl From<$struct_name> for $builder_name {
			fn from(s: $struct_name) -> Self {
				$builder_name {
					$( $field: s.$field ),*
				}
			}
		}
			
		impl From<&$struct_name> for $builder_name {
			fn from(s: &$struct_name) -> Self {
				$builder_name {
					$( $field: s.$field.to_owned() ),*
				}
			}
		}
	};
}
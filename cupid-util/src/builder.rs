// Requires impl ToOwned + derive Default

#[macro_export]
macro_rules! build_struct {
	( 
		$(#[$derive:meta])?
		$bv:vis $builder_name:ident => $v:vis $struct_name:ident 
			$(<$($life:lifetime),* $($generic:ident $(: $($bound:ident $( + $others:tt)*)?)?),*>)? {
		$( 
			$(#[$fderive:meta])?
			$fv:vis $field:ident : $t:ty
		),* $(,)?
	}) => {
		$(#[$derive])?
		$v struct $struct_name $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)? {
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}
		$(#[$derive])?
		$bv struct $builder_name $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)?  {
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}

		impl $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)? $struct_name $(<$($life),* $($generic),*>)? {
			pub fn build() -> $builder_name $(<$($life),* $($generic),*>)? {
				$builder_name::new()
			}
		}
		
		impl $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)? $builder_name $(<$($life),* $($generic),*>)? {
			pub fn new() -> Self {
				Self::default()
			}
			pub fn build(self) -> $struct_name $(<$($life),* $($generic),*>)? {
				$struct_name $(::<$($generic),*>)? {
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
			
		impl $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)? From<$struct_name $(<$($life),* $($generic),*>)?> for $builder_name $(<$($life),* $($generic),*>)? {
			fn from(s: $struct_name $(<$($life),* $($generic),*>)?) -> Self {
				$builder_name $(::<$($generic),*>)? {
					$( $field: s.$field ),*
				}
			}
		}
			
		impl $(<$($life),* $($generic $(: $($bound $( + $others)*)?)?),*>)? From<&$struct_name $(<$($life),* $($generic),*>)?> for $builder_name $(<$($life),* $($generic),*>)? {
			fn from(s: &$struct_name $(<$($life),* $($generic),*>)?) -> Self {
				$builder_name $(::<$($generic),*>)? {
					$( $field: s.$field.to_owned() ),*
				}
			}
		}
	};
}
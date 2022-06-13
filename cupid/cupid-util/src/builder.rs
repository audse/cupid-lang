#[macro_export]
macro_rules! build_struct {
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
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}
		$(#[$derive])?
		$bv struct $builder_name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?  {
			$( 
				$(#[$fderive])?
				$fv $field: $t 
			),*
		}

		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $struct_name $(< $( $lt ),+ >)? {
			pub fn build() -> $builder_name $(< $( $lt ),+ >)? {
				$builder_name::new()
			}
		}
		
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $builder_name $(< $( $lt ),+ >)? {
			pub fn new() -> Self {
				Self::default()
			}
			pub fn build(self) -> $struct_name $(< $( $lt ),+ >)? {
				$struct_name {
					$( $field: self.$field ),*
				}
			}
			pub fn builder(self) -> $builder_name $(< $( $lt ),+ >)? {
				$builder_name$(::< $( $lt ),+ >)?::from(self)
			}
			$(
				$bv fn $field(mut self, val: $t) -> Self {
					self.$field = val;
					self
				}
			)*
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<$struct_name$(< $( $lt ),+ >)?> for $builder_name $(< $( $lt ),+ >)? {
			fn from(s: $struct_name $(< $( $lt ),+ >)? ) -> Self {
				$builder_name$(::< $( $lt ),+ >)?  {
					$( $field: s.$field ),*
				}
			}
		}
			
		impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? From<&$builder_name$(< $( $lt ),+ >)?> for $struct_name $(< $( $lt ),+ >)? {
			fn from(s: &$builder_name $(< $( $lt ),+ >)? ) -> Self {
				$struct_name$(::< $( $lt ),+ >)? {
					$( $field: s.$field.to_owned() ),*
				}
			}
		}
	};
}
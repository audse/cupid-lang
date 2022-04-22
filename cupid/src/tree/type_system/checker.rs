use crate::{Value, Type, SymbolFinder};

pub trait TypeChecker: SymbolFinder {
	fn can_assign(&self, value: &Value, custom_type: &Type) -> bool {
		let value_type = Type::from(value);
		match custom_type {
			Type::Product(_) => {
				custom_type.eq_approx(&value_type)
			},
			Type::Sum(sum_type) => {
				for option in &sum_type.symbols {
					if let Some(custom_type) = self.get_definition(&option) {
						if custom_type.eq_approx(&value_type) {
							return true;
						}
					}
				}
				false
			},
		}
	}
	
}
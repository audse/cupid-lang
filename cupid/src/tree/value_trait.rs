use crate::*;

pub trait Value {
	const TYPE: Type;
	type T;
	
	fn get_value(&self) -> Self::T;
	
	fn get_type(&self) -> Type {
		Self::TYPE
	}
	fn is_type(&self, other: Type) -> bool {
		Self::TYPE == other
	}
	fn is_map_type(&self) -> bool {
		Self::TYPE.is_map()
	}
	fn is_builtin_type(&self) -> bool {
		Self::TYPE.is_builtin()
	}
	fn is_poisoned(&self) -> bool {
		Self::TYPE == ERROR
	}
}

#[derive(PartialEq, Eq)]
struct Integer(i32);
impl Value for Integer {
	const TYPE: Type = INTEGER;
	type T = i32;
	fn get_value(&self) -> i32 {
		self.0
	}
}

struct Decimal(i32, u32);
impl Value for Decimal {
	const TYPE: Type = DECIMAL;
	type T = (i32, u32);
	fn get_value(&self) -> (i32, u32) {
		(self.0, self.1)
	}
}

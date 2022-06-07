use crate::*;

// pub trait ValueType: std::fmt::Debug + CloneValue + 'static {}

// impl Default for Box<dyn ValueType> {
// 	fn default() -> Self {
// 		Box::new(Nothing)
// 	}
// }

// pub trait CloneValue {
// 	fn clone_value(&self) -> Box<dyn ValueType>;
// }

// impl <T: 'static + ValueType + Clone> CloneValue for T {
// 	fn clone_value(&self) -> Box<dyn ValueType> {
// 		Box::new(self.clone())
// 	}
// }

// impl Clone for Box<dyn ValueType> {
// 	fn clone(&self) -> Self {
// 		self.clone_value()
// 	}
// }

// // #[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled)]
// // pub struct BoxValue(pub Value<Box<dyn ValueType>>);

// pub type BoxValue = Value<Box<dyn ValueType>>;

// pub trait EqValue {
// 	fn eq_value(&self, other: &Self) -> bool;
// }
// pub trait HashValue {
// 	fn hash_value<H: Hasher>(&self, state: &mut H);
// }
// impl<T: ValueType + Default + Clone + PartialEq> EqValue for Box<T> {
// 	fn eq_value(&self, other: &Self) -> bool {
// 		self == other
// 	}
// }
// impl<T: ValueType + Default + Clone> HashValue for Box<T> {
// 	fn hash_value<H: Hasher>(&self, state: &mut H) {
// 		(*self).hash(state)
// 	}
// }
// impl PartialEq for Box<dyn ValueType> {
// 	fn eq(&self, other: &Self) -> bool {
// 		(self as Box<T: ValueType + Default + Clone + PartialEq>).eq_value(other)
// 	}
// }
// impl Eq for Box<dyn ValueType> {}
// impl Hash for Box<dyn ValueType> {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		(self as Box<T: ValueType + Default + Clone>).hash_value(state)
// 	}
// }

// // impl<T:ValueType + PartialEq + Eq + Hash + Clone> From<T> for BoxValue {
// // 	fn from(val: T) -> Self {
// // 		Self(Value {
// // 			value: Untyped()
// // 		})
// // 	}
// // }

// // impl std::ops::Deref for BoxValue {
// // 	type Target = Value<Box<dyn ValueType>>;
// // 	fn deref(&self) -> &Self::Target {
// // 		&self.0
// // 	}
// // }

// // impl std::ops::DerefMut for BoxValue {
// // 	fn deref_mut(&mut self) -> &mut Self::Target {
// // 		&mut self.0
// // 	}
// // }

// #[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
// pub struct Decimal(pub i32, pub u32);

// #[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
// pub struct Nothing;

// #[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
// pub struct Placeholder;

// build_struct! {
// 	#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
// 	pub ValueBuilder => pub Value<T: ValueType + Default + Clone> {
// 		pub value: Typed<T>,
// 		pub attributes: Attributes,
// 	}
// }

// impl<T: ValueType + Default + Clone> std::fmt::Display for Value<T> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		write!(f, "{:?}", self.value)
// 	}
// }


// impl<T: ValueType + Default + Clone> Value<T> {
// 	pub fn to_boxed(&self) -> BoxValue {
// 		let val = self.to_owned();
// 		let value = match val.value {
// 			Untyped(v) => Untyped(Box::new(*v) as Box<dyn ValueType>),
// 			IsTyped(v, t) => IsTyped(Box::new(*v) as Box<dyn ValueType>, t)
// 		};
// 		Value {
// 			value,
// 			attributes: val.attributes
// 		}
// 	}

// }

// impl<T: ValueType + Default + Clone> UseAttributes for Value<T> {
// 	fn attributes(&self) -> &Attributes { 
// 		&self.attributes
// 	}
// 	fn attributes_mut(&mut self) -> &mut Attributes { 
// 		&mut self.attributes
// 	}
// }

// // impl ValueType for usize {}
// // impl ValueType for i32 {}
// impl ValueType for Decimal {}
// // impl ValueType for String {}
// // impl ValueType for bool {}
// impl ValueType for Type {}
// impl ValueType for Trait {}
// impl ValueType for Nothing {}
// impl ValueType for Placeholder {}
// impl ValueType for Function {}
// // impl<T: ValueType + Default + Clone> ValueType for Vec<T> {}
// // impl<T: ValueType + Default + Clone> ValueType for Box<T> {}
// impl<T: ValueType + Default + Clone> ValueType for Typed<T> {}
// impl<T: std::ops::Deref<Target = dyn ValueType> + Default + Clone + std::fmt::Debug + 'static> ValueType for T {}

// impl<T: ValueType + Default + Clone> ValueType for Value<T> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Value {
	VArray(Vec<Value>, Attributes),
	VBoolean(bool, Attributes),
	VChar(char, Attributes),
	VDecimal(i32, u32, Attributes),
	VFunction(Box<crate::Function>),
	VInteger(i32, Attributes),
	VNone(Attributes),
	VString(Cow<'static, str>, Attributes),
	VTuple(Vec<Value>, Attributes),
	VType(crate::Type),
	VTrait(crate::Trait),
	VBuiltinPlaceholder(Attributes),
}

pub use Value::*;

impl Default for Value {
	fn default() -> Self { Self::VNone(Attributes::default()) }
}

// impl TypeOf for Val {
// 	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> {
//     	Ok(infer_type(self, scope)?.into())
// 	}
// }

impl UseAttributes for Value {
	fn attributes(&self) -> &Attributes {
		match self {
			VArray(_, a) => a,
			VBoolean(_, a) => a,
			VChar(_, a) => a,
			VDecimal(_, _, a) => a,
			VFunction(function) => function.attributes(),
			VInteger(_, a) => a,
			VNone(a) => a,
			VString(_, a) => a,
			VTuple(_, a) => a,
			VType(type_val) => type_val.attributes(),
			VTrait(trait_val) => trait_val.attributes(),
			VBuiltinPlaceholder(a) => a
		}
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		match self {
			VArray(_, a) => a,
			VBoolean(_, a) => a,
			VChar(_, a) => a,
			VDecimal(_, _, a) => a,
			VFunction(function) => function.attributes_mut(),
			VInteger(_, a) => a,
			VNone(a) => a,
			VString(_, a) => a,
			VTuple(_, a) => a,
			VType(type_val) => type_val.attributes_mut(),
			VTrait(trait_val) => trait_val.attributes_mut(),
			VBuiltinPlaceholder(a) => a
		}
	}
}

// build_struct! {
// 	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
// 	pub ValueBuilder => pub Value {
// 		pub val: Typed<Val>, 

//         #[tabled(skip)]
// 		pub attributes: Attributes
// 	}
// }

// impl From<Typed<Val>> for Value {
// 	fn from(v: Typed<Val>) -> Self {
// 		Value::build().val(v).build()
// 	}
// }

// impl From<Val> for Value {
// 	fn from(v: Val) -> Self {
// 		Value::build().val(Untyped(v)).build()
// 	}
// }

// impl From<&Typed<Val>> for Value {
// 	fn from(v: &Typed<Val>) -> Self {
// 		Value::build().val(v.to_owned()).build()
// 	}
// }

// impl From<&Val> for Value {
// 	fn from(v: &Val) -> Self {
// 		Value::build().val(Untyped(v.to_owned())).build()
// 	}
// }
use crate::{
    gc::GcRef,
    objects::{
        Array, BoundMethod, Class, Closure, Function, Instance, NativeFunction, RoleImpl, Str,
    },
    vm::Vm,
};
use std::{fmt, ops::Deref};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Array(GcRef<Array>),
    Bool(bool),
    BoundMethod(GcRef<BoundMethod>),
    Class(GcRef<Class>),
    RoleImpl(GcRef<RoleImpl>),
    Closure(GcRef<Closure>),
    Function(GcRef<Function>),
    Instance(GcRef<Instance>),
    NativeFunction(NativeFunction),
    Nil,
    Float(f64),
    Int(i32),
    String(GcRef<Str>),
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum Type {
    Array,
    Bool,
    Int,
    Float,
    Nil,
    String,
    Instance(GcRef<Class>),
    Function,
    #[default]
    Unknown,
    Unit,
    Class, // a class definition
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(value) => !value,
            _ => false,
        }
    }

    pub fn as_numbers(
        &self,
        other: &Self,
        int_op: fn(a: i32, b: i32) -> Self,
        float_op: fn(a: f64, b: f64) -> Self,
    ) -> Result<Self, &'static str> {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Ok(int_op(*a, *b)),
            (Self::Int(a), Self::Float(b)) => Ok(float_op(*a as f64, *b)),
            (Self::Float(a), Self::Int(b)) => Ok(float_op(*a, *b as f64)),
            (Self::Float(a), Self::Float(b)) => Ok(float_op(*a, *b)),
            _ => Err("Operands must be numbers."),
        }
    }

    pub fn add(self, other: Self, vm: &mut Vm) -> Result<Self, &'static str> {
        if let Ok(value) =
            self.as_numbers(&other, |a, b| Value::Int(a + b), |a, b| Value::Float(a + b))
        {
            Ok(value)
        } else {
            match (self, other) {
                (Self::String(a), Self::String(b)) => {
                    let result = format!("{}{}", a.deref(), b.deref());
                    let result = vm.intern(result);
                    Ok(Self::String(result))
                }
                _ => Err("Operands must be two numbers or two strings."),
            }
        }
    }

    pub fn subtract(self, other: Self) -> Result<Self, &'static str> {
        self.as_numbers(&other, |a, b| Value::Int(a - b), |a, b| Value::Float(a - b))
    }

    pub fn multiply(self, other: Self) -> Result<Self, &'static str> {
        self.as_numbers(&other, |a, b| Value::Int(a * b), |a, b| Value::Float(a * b))
    }

    pub fn divide(self, other: Self) -> Result<Self, &'static str> {
        self.as_numbers(
            &other,
            |a, b| {
                if b == 0 {
                    Self::Float(f64::NAN)
                } else {
                    Self::Int(a / b)
                }
            },
            |a, b| Value::Float(a / b),
        )
    }

    pub fn greater(self, other: Self) -> Result<Self, &'static str> {
        self.as_numbers(&other, |a, b| Value::Bool(a > b), |a, b| Value::Bool(a > b))
    }

    pub fn lesser(self, other: Self) -> Result<Self, &'static str> {
        self.as_numbers(&other, |a, b| Value::Bool(a < b), |a, b| Value::Bool(a < b))
    }

    pub fn get_type(&self) -> Type {
        // naive approach atm
        match self {
            Self::Array(_) => Type::Array,
            Self::Bool(_) => Type::Bool,
            Self::BoundMethod(_) => Type::Function,
            Self::Class(_) => Type::Class,
            Self::Closure(closure) => Value::Function(closure.function).get_type(),
            Self::Float(_) => Type::Float,
            Self::Function(_) => Type::Function,
            Self::Instance(instance) => Type::Instance(instance.class),
            Self::Int(_) => Type::Int,
            Self::NativeFunction(_) => Type::Function,
            Self::Nil => Type::Nil,
            Self::RoleImpl(_) => Type::Nil,
            Self::String(_) => Type::String,
        }
    }

    pub fn type_check(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Array(value) => write!(f, "{}", value.deref()),
            Value::Bool(value) => write!(f, "{value}"),
            Value::BoundMethod(value) => write!(f, "{}", value.method.function.deref()),
            Value::Class(value) => write!(f, "{}", value.name.deref()),
            Value::Closure(value) => write!(f, "{}", value.function.deref()),
            Value::Function(value) => write!(f, "{}", value.name.deref()),
            Value::Instance(value) => write!(f, "{} instance", value.class.name.deref()),
            Value::NativeFunction(_) => write!(f, "<native fun>"),
            Value::Nil => write!(f, "none"),
            Value::Float(value) => write!(f, "{value}"),
            Value::Int(value) => write!(f, "{value}"),
            Value::RoleImpl(value) => write!(f, "{}", value.deref()),
            Value::String(value) => write!(f, "{}", value.deref()),
        }
    }
}

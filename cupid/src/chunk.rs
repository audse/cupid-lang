use crate::{
    gc::GcRef,
    objects::{BoundMethod, Class, Closure, Function, Instance, NativeFunction, Str},
};
use std::{
    fmt::{self, Display},
    ops::Deref,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Bool(bool),
    BoundMethod(GcRef<BoundMethod>),
    Class(GcRef<Class>),
    Closure(GcRef<Closure>),
    Function(GcRef<Function>),
    Instance(GcRef<Instance>),
    NativeFunction(NativeFunction),
    Nil,
    Number(f64),
    String(GcRef<Str>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(value) => !value,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::BoundMethod(value) => write!(f, "{}", value.method.function.deref()),
            Value::Class(value) => write!(f, "{}", value.name.deref()),
            Value::Closure(value) => write!(f, "{}", value.function.deref()),
            Value::Function(value) => write!(f, "{}", value.name.deref()),
            Value::Instance(value) => write!(f, "{} instance", value.class.name.deref()),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
            Value::Nil => write!(f, "nil"),
            Value::Number(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value.deref()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add,
    Call(u8),
    Class(u8),
    CloseUpvalue,
    Closure(u8),
    Constant(u8),
    DefineGlobal(u8),
    Divide,
    Equal,
    False,
    GetGlobal(u8),
    GetLocal(u8),
    GetProperty(u8),
    GetSuper(u8),
    GetUpvalue(u8),
    Greater,
    Inherit,
    Invoke((u8, u8)),
    Jump(u16),
    JumpIfFalse(u16),
    Less,
    Loop(u16),
    Method(u8),
    Multiply,
    Negate,
    Nil,
    Not,
    Pop,
    Log,
    Return,
    SetGlobal(u8),
    SetLocal(u8),
    SetProperty(u8),
    SetUpvalue(u8),
    Substract,
    SuperInvoke((u8, u8)),
    True,
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn write(&mut self, instruction: Instruction, line: usize) -> usize {
        self.code.push(instruction);
        self.lines.push(line);
        self.code.len() - 1
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read_constant(&self, index: u8) -> Value {
        self.constants[index as usize]
    }

    pub fn read_string(&self, index: u8) -> GcRef<Str> {
        if let Value::String(s) = self.read_constant(index) {
            s
        } else {
            panic!("Constant is not String!")
        }
    }
}

use crate::{gc::GcRef, objects::Str, value::Value};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add,
    Array(u8),
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
    Log,
    Loop(u16),
    Method(u8),
    Multiply,
    Negate,
    Nil,
    Not,
    Pop,
    Return,
    RoleImpl(u8),
    SetGlobal(u8),
    SetLocal(u8),
    SetProperty(u8),
    SetUpvalue(u8),
    Subtract,
    SuperInvoke((u8, u8)),
    True,
}

#[derive(Default, Debug)]
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

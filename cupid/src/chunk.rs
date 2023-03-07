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

// pub trait InstructionVisitor<Ctx, Result> {
//     fn visit_add(&mut self, ctx: Ctx) -> Result;
//     fn visit_array(&mut self, item_count: u8, ctx: Ctx) -> Result;
//     fn visit_call(&mut self, arg_count: u8, ctx: Ctx) -> Result;
//     fn visit_class(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_close_upvalue(&mut self, ctx: Ctx) -> Result;
//     fn visit_closure(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_constant(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_define_global(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_divide(&mut self, ctx: Ctx) -> Result;
//     fn visit_equal(&mut self, ctx: Ctx) -> Result;
//     fn visit_false(&mut self, ctx: Ctx) -> Result;
//     fn visit_get_global(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_get_local(&mut self, slot: u8, ctx: Ctx) -> Result;
//     fn visit_get_property(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_get_super(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_get_upvalue(&mut self, slot: u8, ctx: Ctx) -> Result;
//     fn visit_greater(&mut self, ctx: Ctx) -> Result;
//     fn visit_inherit(&mut self, ctx: Ctx) -> Result;
//     fn visit_invoke(&mut self, name_constant: u8, arg_count: u8, ctx: Ctx) -> Result;
//     fn visit_jump(&mut self, offset: u16, ctx: Ctx) -> Result;
//     fn visit_jump_if_false(&mut self, offset: u16, ctx: Ctx) -> Result;
//     fn visit_less(&mut self, ctx: Ctx) -> Result;
//     fn visit_loop(&mut self, offset: u16, ctx: Ctx) -> Result;
//     fn visit_log(&mut self, ctx: Ctx) -> Result;
//     fn visit_method(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_multiply(&mut self, ctx: Ctx) -> Result;
//     fn visit_negate(&mut self, ctx: Ctx) -> Result;
//     fn visit_nil(&mut self, ctx: Ctx) -> Result;
//     fn visit_not(&mut self, ctx: Ctx) -> Result;
//     fn visit_pop(&mut self, ctx: Ctx) -> Result;
//     fn visit_return(&mut self, ctx: Ctx) -> Result;
//     fn visit_role_impl(&mut self, role_name_constant: u8, ctx: Ctx) -> Result;
//     fn visit_set_global(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_set_local(&mut self, slot: u8, ctx: Ctx) -> Result;
//     fn visit_set_property(&mut self, constant: u8, ctx: Ctx) -> Result;
//     fn visit_set_upvalue(&mut self, slot: u8, ctx: Ctx) -> Result;
//     fn visit_subtract(&mut self, ctx: Ctx) -> Result;
//     fn visit_super_invoke(&mut self, class_constant: u8, arg_count: u8, ctx: Ctx) -> Result;
//     fn visit_true(&mut self, ctx: Ctx) -> Result;
// }

// impl Instruction {
//     pub fn accept<Ctx, Result>(
//         &mut self,
//         visitor: &mut dyn InstructionVisitor<Ctx, Result>,
//         ctx: Ctx,
//     ) -> Result {
//         use self::Instruction::*;
//         match self {
//             Add => visitor.visit_add(ctx),
//             Array(a) => visitor.visit_array(*a, ctx),
//             Call(a) => visitor.visit_call(*a, ctx),
//             Class(a) => visitor.visit_class(*a, ctx),
//             CloseUpvalue => visitor.visit_close_upvalue(ctx),
//             Closure(a) => visitor.visit_closure(*a, ctx),
//             Constant(a) => visitor.visit_constant(*a, ctx),
//             DefineGlobal(a) => visitor.visit_define_global(*a, ctx),
//             Divide => visitor.visit_divide(ctx),
//             Equal => visitor.visit_equal(ctx),
//             False => visitor.visit_false(ctx),
//             GetGlobal(a) => visitor.visit_get_global(*a, ctx),
//             GetLocal(a) => visitor.visit_get_local(*a, ctx),
//             GetProperty(a) => visitor.visit_get_property(*a, ctx),
//             GetSuper(a) => visitor.visit_get_super(*a, ctx),
//             GetUpvalue(a) => visitor.visit_get_upvalue(*a, ctx),
//             Greater => visitor.visit_greater(ctx),
//             Inherit => visitor.visit_inherit(ctx),
//             Invoke((a, b)) => visitor.visit_invoke(*a, *b, ctx),
//             Jump(a) => visitor.visit_jump(*a, ctx),
//             JumpIfFalse(a) => visitor.visit_jump_if_false(*a, ctx),
//             Less => visitor.visit_less(ctx),
//             Log => visitor.visit_log(ctx),
//             Loop(a) => visitor.visit_loop(*a, ctx),
//             Method(a) => visitor.visit_method(*a, ctx),
//             Multiply => visitor.visit_multiply(ctx),
//             Negate => visitor.visit_negate(ctx),
//             Nil => visitor.visit_nil(ctx),
//             Not => visitor.visit_not(ctx),
//             Pop => visitor.visit_pop(ctx),
//             Return => visitor.visit_return(ctx),
//             RoleImpl(a) => visitor.visit_role_impl(*a, ctx),
//             SetGlobal(a) => visitor.visit_set_global(*a, ctx),
//             SetLocal(a) => visitor.visit_set_local(*a, ctx),
//             SetProperty(a) => visitor.visit_set_property(*a, ctx),
//             SetUpvalue(a) => visitor.visit_set_upvalue(*a, ctx),
//             Subtract => visitor.visit_subtract(ctx),
//             SuperInvoke((a, b)) => visitor.visit_super_invoke(*a, *b, ctx),
//             True => visitor.visit_true(ctx),
//         }
//     }
// }

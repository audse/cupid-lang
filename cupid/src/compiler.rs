use std::fmt;

use crate::{
    gc::GcRef,
    objects::FunctionUpvalue,
    objects::{Function, Str},
    token::Token,
};

#[derive(Copy, Clone)]
pub struct Local<'src> {
    pub name: &'src str,
    pub depth: i32,
    pub is_captured: bool,
}

impl<'src> Local<'src> {
    pub fn new(name: &'src str, depth: i32) -> Self {
        Local {
            name,
            depth,
            is_captured: false,
        }
    }
}

impl fmt::Debug for Local<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "local {}", self.name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Function,
    Initializer,
    Method,
    Script,
}

#[derive(Debug)]
pub struct Compiler<'src> {
    pub enclosing: Option<Box<Compiler<'src>>>,
    pub function: Function,
    pub function_type: FunctionType,
    pub locals: Vec<Local<'src>>,
    pub scope_depth: i32,
}

impl<'src> Compiler<'src> {
    pub const LOCAL_COUNT: usize = std::u8::MAX as usize + 1;

    pub fn new(function_name: GcRef<Str>, kind: FunctionType) -> Box<Self> {
        let mut compiler = Compiler {
            enclosing: None,
            function: Function::new(function_name),
            function_type: kind,
            locals: Vec::with_capacity(Compiler::LOCAL_COUNT),
            scope_depth: 0,
        };

        let token = match kind {
            FunctionType::Method | FunctionType::Initializer => Token::synthetic("self"),
            _ => Token::synthetic(""),
        };
        compiler.locals.push(Local::new(token.lexeme, 0));
        Box::new(compiler)
    }

    pub fn resolve_local(&mut self, name: &str, errors: &mut Vec<&'static str>) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if name == local.name {
                if local.depth == -1 {
                    errors.push("Can't read local variable in its own initializer.");
                }
                return Some(i as u8);
            }
        }
        None
    }

    pub fn resolve_upvalue(&mut self, name: &str, errors: &mut Vec<&'static str>) -> Option<u8> {
        if let Some(enclosing) = self.enclosing.as_mut() {
            if let Some(index) = enclosing.resolve_local(name, errors) {
                enclosing.locals[index as usize].is_captured = true;
                return Some(self.add_upvalue(index, true, errors));
            }
            if let Some(index) = enclosing.resolve_upvalue(name, errors) {
                return Some(self.add_upvalue(index, false, errors));
            }
        }
        None
    }

    pub fn add_upvalue(&mut self, index: u8, is_local: bool, errors: &mut Vec<&'static str>) -> u8 {
        for (i, upvalue) in self.function.upvalues.iter().enumerate() {
            if upvalue.index == index && upvalue.is_local == is_local {
                return i as u8;
            }
        }
        let count = self.function.upvalues.len();

        if count == Compiler::LOCAL_COUNT {
            errors.push("Too many closure variables in function.");
            return 0;
        }

        let upvalue = FunctionUpvalue { index, is_local };
        self.function.upvalues.push(upvalue);
        count as u8
    }

    pub fn is_local_declared(&self, name: &'src str) -> bool {
        for local in self.locals.iter().rev() {
            if local.depth != -1 && local.depth < self.scope_depth {
                return false;
            }
            if local.name == name {
                return true;
            }
        }
        false
    }
}

pub struct ClassCompiler {
    pub enclosing: Option<Box<ClassCompiler>>,
    pub has_superclass: bool,
}

impl ClassCompiler {
    pub fn new(enclosing: Option<Box<ClassCompiler>>) -> Box<Self> {
        Box::new(ClassCompiler {
            enclosing,
            has_superclass: false,
        })
    }
}

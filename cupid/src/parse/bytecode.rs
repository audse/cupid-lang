use crate::{
    arena::{EntryId, ExprArena, UseArena},
    chunk::Instruction,
    compiler::{ClassCompiler, Compiler, FunctionType, Local},
    gc::{Gc, GcRef},
    objects::Function,
    token::TokenType,
    value::Value,
};
use std::convert::TryFrom;

use ast::{
    Array, BinOp, Block, Break, Call, Class, Constant, Define, Expr, Fun, Get, GetProperty,
    GetSuper, If, Invoke, InvokeSuper, Loop, Method, Return, Set, SetProperty, UnOp,
};

#[derive(Default)]
pub struct Errors {
    resolver: Vec<&'static str>,
}

pub struct BytecodeCompiler<'src> {
    pub expr: Vec<Expr<'src>>,
    pub arena: ExprArena<'src>,
    pub compiler: Box<Compiler<'src>>,
    pub class_compiler: Option<Box<ClassCompiler>>,
    pub gc: &'src mut Gc,
    pub errors: Errors,
    pub loop_jumps: Vec<Vec<usize>>,
}

impl<'src> BytecodeCompiler<'src> {
    pub fn new(expr: Vec<Expr<'src>>, arena: ExprArena<'src>, gc: &'src mut Gc) -> Self {
        let function_name = gc.intern("script".to_owned());
        Self {
            expr,
            arena,
            errors: Errors::default(),
            loop_jumps: vec![],
            compiler: Compiler::new(function_name, FunctionType::Script),
            class_compiler: None,
            gc,
        }
    }

    pub fn compile(mut self) -> GcRef<Function> {
        let exprs = std::mem::take(&mut self.expr);
        for expr in exprs {
            expr.compile(&mut self);
        }
        self.write_return();
        self.gc.alloc(self.compiler.function)
    }

    fn write(&mut self, instruction: Instruction) -> usize {
        let approx_line = self.compiler.function.chunk.code.len(); // TODO replace with actual line
        self.compiler.function.chunk.write(instruction, approx_line)
    }

    // fn write_line(&mut self, instruction: Instruction, line: usize) -> usize {
    //     self.compiler.function.chunk.write(instruction, line)
    // }

    fn write_pop(&mut self) -> usize {
        self.write(Instruction::Pop)
    }

    fn push(&mut self, name: &'src str, kind: FunctionType) {
        let function_name = self.gc.intern(name.to_owned());
        let new_compiler = Compiler::new(function_name, kind);
        let old_compiler = std::mem::replace(&mut self.compiler, new_compiler);
        self.compiler.enclosing = Some(old_compiler);
    }

    fn pop(&mut self) -> Function {
        self.write_return();
        match self.compiler.enclosing.take() {
            Some(enclosing) => {
                let compiler = std::mem::replace(&mut self.compiler, enclosing);
                compiler.function
            }
            None => panic!("Didn't find an enclosing compiler"),
        }
    }

    fn update_class_compiler(&mut self) {
        let old_class_compiler = self.class_compiler.take();
        let new_class_compiler = ClassCompiler::new(old_class_compiler);
        self.class_compiler.replace(new_class_compiler);
    }

    fn constant(&mut self, value: Value) -> u8 {
        let index = self.compiler.function.chunk.add_constant(value);
        match u8::try_from(index) {
            Ok(index) => index,
            Err(_) => panic!("Too many constants in one chunk."),
        }
    }

    fn ident_constant(&mut self, ident: &'src str) -> u8 {
        let identifier = self.gc.intern(ident);
        let value = Value::String(identifier);
        self.constant(value)
    }

    fn declare(&mut self, name: &'src str) {
        if self.compiler.scope_depth == 0 {
            return;
        }
        if self.compiler.is_local_declared(name) {
            panic!("Already variable with self name in self scope.")
        }
        self.add_local(name)
    }

    fn declare_constant(&mut self, name: &'src str) -> u8 {
        self.declare(name);
        if self.compiler.scope_depth > 0 {
            return 0;
        }
        self.ident_constant(name)
    }

    fn define(&mut self, index: u8) {
        if self.compiler.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.write(Instruction::DefineGlobal(index));
    }

    fn mark_initialized(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }
        let last_local = self.compiler.locals.last_mut().unwrap();
        last_local.depth = self.compiler.scope_depth;
    }

    fn add_local(&mut self, name: &'src str) {
        if self.compiler.locals.len() == Compiler::LOCAL_COUNT {
            panic!("Too many local variables in function.")
        }
        let local = Local::new(name, -1);
        self.compiler.locals.push(local);
    }

    fn patch_jump(&mut self, pos: usize) {
        let offset = self.compiler.function.chunk.code.len() - 1 - pos;
        let offset = match u16::try_from(offset) {
            Ok(offset) => offset,
            Err(_) => panic!("Too much code to jump over."),
        };
        match self.compiler.function.chunk.code[pos] {
            Instruction::JumpIfFalse(ref mut o) => *o = offset,
            Instruction::Jump(ref mut o) => *o = offset,
            _ => panic!("Instruction at position is not jump"),
        }
    }

    fn patch_loop_jumps(&mut self) {
        for jump in self.loop_jumps.last().unwrap().clone() {
            self.patch_jump(jump);
        }
        self.loop_jumps.pop();
    }

    fn start_loop(&self) -> usize {
        self.compiler.function.chunk.code.len()
    }

    fn write_loop(&mut self, start_pos: usize) {
        let offset = self.compiler.function.chunk.code.len() - start_pos;
        let offset = match u16::try_from(offset) {
            Ok(o) => o,
            Err(_) => panic!("Loop body too large."),
        };
        self.write(Instruction::Loop(offset));
    }

    fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;
        for i in (0..self.compiler.locals.len()).rev() {
            if self.compiler.locals[i].depth > self.compiler.scope_depth {
                if self.compiler.locals[i].is_captured {
                    self.write(Instruction::CloseUpvalue);
                } else {
                    self.write_pop();
                }
                self.compiler.locals.pop();
            }
        }
    }

    fn get_name(&mut self, name: &'src str) {
        let instruction = if let Some(arg) = self.resolve_local(name) {
            Instruction::GetLocal(arg)
        } else if let Some(arg) = self.resolve_upvalue(name) {
            Instruction::GetUpvalue(arg)
        } else {
            let index = self.ident_constant(name);
            Instruction::GetGlobal(index)
        };
        self.write(instruction);
    }

    fn set_name(&mut self, name: &'src str) {
        let instruction = if let Some(arg) = self.resolve_local(name) {
            Instruction::SetLocal(arg)
        } else if let Some(arg) = self.resolve_upvalue(name) {
            Instruction::SetUpvalue(arg)
        } else {
            let index = self.ident_constant(name);
            Instruction::SetGlobal(index)
        };
        self.write(instruction);
    }

    fn resolve_local(&mut self, name: &str) -> Option<u8> {
        let result = self.compiler.resolve_local(name, &mut self.errors.resolver);
        while let Some(e) = self.errors.resolver.pop() {
            panic!("{}", e);
        }
        result
    }

    fn resolve_upvalue(&mut self, name: &'src str) -> Option<u8> {
        let result = self.compiler.resolve_upvalue(name, &mut self.errors.resolver);
        while let Some(e) = self.errors.resolver.pop() {
            panic!("{}", e);
        }
        result
    }

    fn has_superclass(&mut self) {
        self.class_compiler.as_mut().unwrap().has_superclass = true;
    }

    fn write_return(&mut self) -> usize {
        match self.compiler.function_type {
            FunctionType::Initializer => self.write(Instruction::GetLocal(0)),
            _ => self.write(Instruction::Nil),
        };
        self.write(Instruction::Return)
    }

    fn expect_class_compiler(&mut self) {
        match self.class_compiler.as_ref() {
            Some(current_class) if !current_class.has_superclass => {
                panic!("Can't use 'super' in a class with no superclass.");
            }
            None => panic!("Can't use 'super' outside of a class."),
            _ => (),
        }
    }
}

pub trait ToBytecode<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>);
}

impl<'src, T: ToBytecode<'src>> ToBytecode<'src> for Vec<T> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        for item in self.iter() {
            item.compile(compiler)
        }
    }
}

impl<'src> ToBytecode<'src> for EntryId {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let expr: Expr<'src> = compiler.arena.take(*self);
        expr.compile(compiler);
        compiler.arena.replace(*self, expr);
    }
}

impl<'src> ToBytecode<'src> for Expr<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        match self {
            Self::Array(array) => array.compile(compiler),
            Self::BinOp(binop) => binop.compile(compiler),
            Self::Block(block) => block.compile(compiler),
            Self::Break(stmt) => stmt.compile(compiler),
            Self::Call(call) => call.compile(compiler),
            Self::Class(class) => class.compile(compiler),
            Self::Constant(constant) => constant.compile(compiler),
            Self::Define(define) => define.compile(compiler),
            Self::Fun(fun) => fun.compile(compiler),
            Self::Get(get) => get.compile(compiler),
            Self::GetProperty(get) => get.compile(compiler),
            Self::GetSuper(get) => get.compile(compiler),
            Self::If(stmt) => stmt.compile(compiler),
            Self::Invoke(invoke) => invoke.compile(compiler),
            Self::InvokeSuper(invoke) => invoke.compile(compiler),
            Self::Loop(value) => value.compile(compiler),
            Self::Return(stmt) => stmt.compile(compiler),
            Self::Set(set) => set.compile(compiler),
            Self::SetProperty(set) => set.compile(compiler),
            Self::UnOp(unop) => unop.compile(compiler),
        }
    }
}

impl<'src> ToBytecode<'src> for Array<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.items.compile(compiler);
        compiler.write(Instruction::Array(self.items.len() as u8));
    }
}

impl<'src> ToBytecode<'src> for BinOp<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.left.compile(compiler);
        self.right.compile(compiler);
        match self.op {
            TokenType::Plus => compiler.write(Instruction::Add),
            TokenType::Minus => compiler.write(Instruction::Subtract),
            TokenType::Star => compiler.write(Instruction::Multiply),
            TokenType::Slash => compiler.write(Instruction::Divide),
            TokenType::Greater => compiler.write(Instruction::Greater),
            TokenType::Less => compiler.write(Instruction::Less),
            TokenType::EqualEqual => compiler.write(Instruction::Equal),
            TokenType::BangEqual => compiler.write(Instruction::Not),
            kind => {
                println!("Unimplemented binary operation: {kind:#?}");
                todo!()
            }
        };
    }
}

impl<'src> ToBytecode<'src> for Block<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        compiler.begin_scope();
        self.body.compile(compiler);
        compiler.end_scope();
    }
}

impl<'src> ToBytecode<'src> for Break<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        if let Some(value) = &self.value {
            value.compile(compiler);
        }
        let break_id = compiler.write(Instruction::Jump(0xffff));
        let loop_jump = match compiler.loop_jumps.last_mut() {
            Some(loop_jump) => loop_jump,
            None => panic!("Can't break outside of a loop."),
        };
        loop_jump.push(break_id);
    }
}

impl<'src> ToBytecode<'src> for Call<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let callee = compiler.arena.expect(self.callee);
        match callee {
            Expr::Get(get) if get.name == "log" => {
                self.args.compile(compiler);
                compiler.write(Instruction::Log)
            }
            _ => {
                self.callee.compile(compiler);
                self.args.compile(compiler);
                compiler.write(Instruction::Call(self.args.len() as u8))
            }
        };
    }
}

impl<'src> ToBytecode<'src> for Class<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let name = compiler.ident_constant(self.name);
        compiler.declare(self.name);
        compiler.write(Instruction::Class(name));
        compiler.define(name);

        compiler.update_class_compiler();

        let has_superclass = self.super_class.is_some();
        if let Some(super_name) = self.super_class {
            let _index = compiler.ident_constant(super_name);
            compiler.get_name(super_name);
            compiler.begin_scope();
            compiler.add_local("super");
            compiler.define(0); // TODO this may be wrong
            compiler.get_name(self.name);
            compiler.write(Instruction::Inherit);
            compiler.has_superclass();
        }

        compiler.get_name(self.name);
        self.methods.compile(compiler);
        compiler.write_pop();

        if has_superclass {
            compiler.end_scope();
        }

        match compiler.class_compiler.take() {
            Some(c) => compiler.class_compiler = c.enclosing,
            None => compiler.class_compiler = None,
        }
    }
}

impl<'src> ToBytecode<'src> for Define<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let index = compiler.declare_constant(self.name);
        match &self.value {
            Some(value) => value.compile(compiler),
            _ => {
                compiler.write(Instruction::Nil);
            }
        }
        compiler.define(index);
    }
}

impl<'src> ToBytecode<'src> for Fun<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let global = match self.name {
            Some(name) => {
                let global = Some(compiler.declare_constant(name));
                compiler.mark_initialized();
                global
            }
            None => None,
        };

        compiler.push(self.name.unwrap_or_else(|| "__closure"), self.kind);
        compiler.begin_scope();

        for param in &self.params {
            compiler.compiler.function.arity += 1;
            if compiler.compiler.function.arity > 255 {
                panic!("Can't have more than 255 parameters.");
            }
            let param = compiler.declare_constant(param.name);
            compiler.define(param);
        }

        self.body.compile(compiler);

        let fun = compiler.pop();
        let id = compiler.gc.alloc(fun);
        let index = compiler.constant(Value::Function(id));
        compiler.write(Instruction::Closure(index));

        if let Some(global) = global {
            compiler.define(global);
        }
    }
}

impl<'src> ToBytecode<'src> for Get<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        compiler.get_name(self.name)
    }
}

impl<'src> ToBytecode<'src> for GetProperty<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.receiver.compile(compiler);
        let name = compiler.ident_constant(self.property);
        compiler.write(Instruction::GetProperty(name));
    }
}

impl<'src> ToBytecode<'src> for GetSuper<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        compiler.expect_class_compiler();
        let name = compiler.ident_constant(self.name);
        compiler.get_name("self");
        compiler.get_name("super");
        compiler.write(Instruction::GetSuper(name));
    }
}

impl<'src> ToBytecode<'src> for If<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.condition.compile(compiler);
        let then_jump = compiler.write(Instruction::JumpIfFalse(0xffff));
        compiler.write_pop();

        self.body.compile(compiler);
        let else_jump = compiler.write(Instruction::Jump(0xffff));
        compiler.patch_jump(then_jump);
        compiler.write_pop();

        if let Some(else_body) = &self.else_body {
            else_body.compile(compiler);
        }
        compiler.patch_jump(else_jump);
    }
}

impl<'src> ToBytecode<'src> for Invoke<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.receiver.compile(compiler);
        let name = compiler.ident_constant(self.callee);
        self.args.compile(compiler);
        compiler.write(Instruction::Invoke(name, self.args.len() as u8));
    }
}

impl<'src> ToBytecode<'src> for InvokeSuper<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        compiler.expect_class_compiler();
        let name = compiler.ident_constant(self.name);
        compiler.get_name("self");
        self.args.compile(compiler);
        compiler.get_name("super");
        compiler.write(Instruction::SuperInvoke(name, self.args.len() as u8));
    }
}

impl<'src> ToBytecode<'src> for Loop<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let loop_start = compiler.start_loop();
        compiler.loop_jumps.push(vec![]);
        self.body.compile(compiler);
        compiler.write_loop(loop_start);

        let exit_jump = compiler.write(Instruction::Jump(0xffff));
        compiler.patch_jump(exit_jump);
        compiler.patch_loop_jumps();
    }
}

impl<'src> ToBytecode<'src> for Method<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let constant = compiler.ident_constant(self.name);
        self.fun.compile(compiler);
        compiler.write(Instruction::Method(constant));
    }
}

impl<'src> ToBytecode<'src> for Return<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        match compiler.compiler.function_type {
            FunctionType::Script => panic!("Can't return from top-level code."),
            FunctionType::Initializer => panic!("Can't return a value from an initializer."),
            _ => match &self.value {
                Some(value) => {
                    value.compile(compiler);
                    compiler.write(Instruction::Return);
                }
                None => {
                    compiler.write_return();
                }
            },
        }
    }
}

impl<'src> ToBytecode<'src> for Set<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.value.compile(compiler);
        compiler.set_name(self.name);
    }
}

impl<'src> ToBytecode<'src> for SetProperty<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.receiver.compile(compiler);
        let name = compiler.ident_constant(self.property);
        self.value.compile(compiler);
        compiler.write(Instruction::SetProperty(name));
    }
}

impl<'src> ToBytecode<'src> for UnOp<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        self.expr.compile(compiler);
        let instruction = match self.op {
            TokenType::Bang => Instruction::Not,
            TokenType::Minus => Instruction::Negate,
            _ => panic!("Invalid unary operator"),
        };
        compiler.write(instruction);
    }
}

impl<'src> ToBytecode<'src> for Constant<'src> {
    fn compile(&self, compiler: &mut BytecodeCompiler<'src>) {
        let index = compiler.constant(self.value);
        compiler.write(Instruction::Constant(index));
    }
}

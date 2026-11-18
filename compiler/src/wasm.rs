//! API stub: behaviour is added in the GREEN stage.

pub const I32: u8 = 0x7F;
pub const I64: u8 = 0x7E;
pub const F64: u8 = 0x7C;
pub const VOID: u8 = 0x40;
pub const FUNC: u8 = 0;
pub const TABLE: u8 = 1;
pub const MEM: u8 = 2;
pub const GLOBAL: u8 = 3;

pub fn uleb(_out: &mut Vec<u8>, _v: u64) {}

pub fn sleb(_out: &mut Vec<u8>, _v: i64) {}

#[derive(Clone, Copy)]
pub struct Label(pub u32);

#[derive(Default)]
pub struct Code;

impl Code {
    pub fn new() -> Self {
        Code
    }
    pub fn op(&mut self, _opcode: u8) -> &mut Self {
        self
    }
    pub fn idx(&mut self, _opcode: u8, _n: u32) -> &mut Self {
        self
    }
    pub fn i32_const(&mut self, _v: i32) -> &mut Self {
        self
    }
    pub fn i64_const(&mut self, _v: i64) -> &mut Self {
        self
    }
    pub fn f64_const(&mut self, _v: f64) -> &mut Self {
        self
    }
    pub fn mem(&mut self, _opcode: u8, _align: u32, _offset: u32) -> &mut Self {
        self
    }
    pub fn call_indirect(&mut self, _ty: u32) -> &mut Self {
        self
    }
    pub fn block(&mut self, _bt: u8) -> Label {
        Label(0)
    }
    pub fn loop_(&mut self, _bt: u8) -> Label {
        Label(0)
    }
    pub fn if_(&mut self, _bt: u8) -> Label {
        Label(0)
    }
    pub fn else_(&mut self) -> &mut Self {
        self
    }
    pub fn end(&mut self) -> &mut Self {
        self
    }
    pub fn br(&mut self, _l: Label) -> &mut Self {
        self
    }
    pub fn br_if(&mut self, _l: Label) -> &mut Self {
        self
    }
}

#[derive(Default)]
pub struct Module;

impl Module {
    pub fn new() -> Self {
        Module
    }
    pub fn ty(&mut self, _params: &[u8], _results: &[u8]) -> u32 {
        0
    }
    pub fn import_func(&mut self, _module: &str, _name: &str, _ty: u32) -> u32 {
        0
    }
    pub fn func(&mut self, _ty: u32, _locals: &[u8], _body: Code) -> u32 {
        0
    }
    pub fn table(&mut self, _min: u32) {}
    pub fn memory(&mut self, _min_pages: u32) {}
    pub fn global_i32(&mut self, _mutable: bool, _init: i32) -> u32 {
        0
    }
    pub fn export(&mut self, _name: &str, _kind: u8, _idx: u32) {}
    pub fn elem(&mut self, _offset: u32, _funcs: &[u32]) {}
    pub fn data(&mut self, _offset: u32, _bytes: &[u8]) {}
    pub fn finish(self) -> Vec<u8> {
        Vec::new()
    }
}

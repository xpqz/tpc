//! WASM binary emitter: LEB128, section writers and a function-body builder.

pub const I32: u8 = 0x7F;
pub const I64: u8 = 0x7E;
pub const F64: u8 = 0x7C;
pub const VOID: u8 = 0x40;
pub const FUNC: u8 = 0;
pub const TABLE: u8 = 1;
pub const MEM: u8 = 2;
pub const GLOBAL: u8 = 3;

pub fn uleb(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7F) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

pub fn sleb(out: &mut Vec<u8>, mut v: i64) {
    loop {
        let b = (v & 0x7F) as u8;
        v >>= 7;
        if (v == 0 && b & 0x40 == 0) || (v == -1 && b & 0x40 != 0) {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

fn bytes(out: &mut Vec<u8>, b: &[u8]) {
    uleb(out, b.len() as u64);
    out.extend_from_slice(b);
}

fn const_expr(out: &mut Vec<u8>, v: i32) {
    out.push(0x41);
    sleb(out, v as i64);
    out.push(0x0B);
}

/// A branch target: the nesting depth at which its block was opened.
#[derive(Clone, Copy)]
pub struct Label(pub u32);

/// A function body under construction; `open` records, per open block, whether it is an `if`.
#[derive(Default)]
pub struct Code {
    buf: Vec<u8>,
    open: Vec<bool>,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn op(&mut self, opcode: u8) -> &mut Self {
        self.buf.push(opcode);
        self
    }

    pub fn idx(&mut self, opcode: u8, n: u32) -> &mut Self {
        self.buf.push(opcode);
        uleb(&mut self.buf, n as u64);
        self
    }

    pub fn i32_const(&mut self, v: i32) -> &mut Self {
        self.buf.push(0x41);
        sleb(&mut self.buf, v as i64);
        self
    }

    pub fn i64_const(&mut self, v: i64) -> &mut Self {
        self.buf.push(0x42);
        sleb(&mut self.buf, v);
        self
    }

    pub fn f64_const(&mut self, v: f64) -> &mut Self {
        self.buf.push(0x44);
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn mem(&mut self, opcode: u8, align: u32, offset: u32) -> &mut Self {
        self.idx(opcode, align);
        uleb(&mut self.buf, offset as u64);
        self
    }

    pub fn call_indirect(&mut self, ty: u32) -> &mut Self {
        self.idx(0x11, ty).op(0)
    }

    fn open(&mut self, opcode: u8, bt: u8, is_if: bool) -> Label {
        self.buf.extend_from_slice(&[opcode, bt]);
        self.open.push(is_if);
        Label(self.open.len() as u32 - 1)
    }

    pub fn block(&mut self, bt: u8) -> Label {
        self.open(0x02, bt, false)
    }

    pub fn loop_(&mut self, bt: u8) -> Label {
        self.open(0x03, bt, false)
    }

    pub fn if_(&mut self, bt: u8) -> Label {
        self.open(0x04, bt, true)
    }

    pub fn else_(&mut self) -> &mut Self {
        assert!(self.open.last() == Some(&true), "else outside if");
        *self.open.last_mut().unwrap() = false;
        self.op(0x05)
    }

    pub fn end(&mut self) -> &mut Self {
        assert!(self.open.pop().is_some(), "unbalanced end");
        self.op(0x0B)
    }

    fn depth(&self, l: Label) -> u32 {
        assert!((l.0 as usize) < self.open.len(), "label is not open");
        self.open.len() as u32 - 1 - l.0
    }

    pub fn br(&mut self, l: Label) -> &mut Self {
        let d = self.depth(l);
        self.idx(0x0C, d)
    }

    pub fn br_if(&mut self, l: Label) -> &mut Self {
        let d = self.depth(l);
        self.idx(0x0D, d)
    }

    pub fn br_table(&mut self, targets: &[Label], default: Label) -> &mut Self {
        self.idx(0x0E, targets.len() as u32);
        for &l in targets.iter().chain([&default]) {
            let d = self.depth(l);
            uleb(&mut self.buf, d as u64);
        }
        self
    }
}

/// A section's entry count and pre-encoded entries.
#[derive(Default)]
struct Sec(u32, Vec<u8>);

#[derive(Default)]
pub struct Module {
    types: Vec<(Vec<u8>, Vec<u8>)>,
    imports: Sec,
    funcs: Vec<u32>,
    bodies: Sec,
    table: Option<u32>,
    memory: Option<u32>,
    globals: Sec,
    exports: Sec,
    elems: Sec,
    datas: Sec,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ty(&mut self, params: &[u8], results: &[u8]) -> u32 {
        let sig = (params.to_vec(), results.to_vec());
        let at = self
            .types
            .iter()
            .position(|t| *t == sig)
            .unwrap_or(self.types.len());
        if at == self.types.len() {
            self.types.push(sig);
        }
        at as u32
    }

    pub fn import_func(&mut self, module: &str, name: &str, ty: u32) -> u32 {
        assert!(self.funcs.is_empty(), "import after function");
        bytes(&mut self.imports.1, module.as_bytes());
        bytes(&mut self.imports.1, name.as_bytes());
        self.imports.1.push(0);
        uleb(&mut self.imports.1, ty as u64);
        self.imports.0 += 1;
        self.imports.0 - 1
    }

    pub fn func(&mut self, ty: u32, locals: &[u8], mut body: Code) -> u32 {
        assert!(body.open.is_empty(), "unbalanced blocks at function end");
        let mut groups: Vec<(u32, u8)> = Vec::new();
        for &l in locals {
            match groups.last_mut() {
                Some((n, t)) if *t == l => *n += 1,
                _ => groups.push((1, l)),
            }
        }
        let mut entry = Vec::new();
        uleb(&mut entry, groups.len() as u64);
        for (n, t) in groups {
            uleb(&mut entry, n as u64);
            entry.push(t);
        }
        entry.append(&mut body.buf);
        entry.push(0x0B);
        bytes(&mut self.bodies.1, &entry);
        self.bodies.0 += 1;
        self.funcs.push(ty);
        self.imports.0 + self.funcs.len() as u32 - 1
    }

    pub fn table(&mut self, min: u32) {
        self.table = Some(min);
    }

    pub fn memory(&mut self, min_pages: u32) {
        self.memory = Some(min_pages);
    }

    pub fn global_i32(&mut self, mutable: bool, init: i32) -> u32 {
        self.globals.1.extend_from_slice(&[I32, mutable as u8]);
        const_expr(&mut self.globals.1, init);
        self.globals.0 += 1;
        self.globals.0 - 1
    }

    pub fn export(&mut self, name: &str, kind: u8, idx: u32) {
        bytes(&mut self.exports.1, name.as_bytes());
        self.exports.1.push(kind);
        uleb(&mut self.exports.1, idx as u64);
        self.exports.0 += 1;
    }

    pub fn elem(&mut self, offset: u32, funcs: &[u32]) {
        self.elems.1.push(0);
        const_expr(&mut self.elems.1, offset as i32);
        uleb(&mut self.elems.1, funcs.len() as u64);
        for &f in funcs {
            uleb(&mut self.elems.1, f as u64);
        }
        self.elems.0 += 1;
    }

    pub fn data(&mut self, offset: u32, data: &[u8]) {
        self.datas.1.push(0);
        const_expr(&mut self.datas.1, offset as i32);
        bytes(&mut self.datas.1, data);
        self.datas.0 += 1;
    }

    /// Sections in id order; empty ones are omitted.
    pub fn finish(self) -> Vec<u8> {
        let mut types = Sec::default();
        for (p, r) in &self.types {
            types.1.push(0x60);
            bytes(&mut types.1, p);
            bytes(&mut types.1, r);
            types.0 += 1;
        }
        let mut funcs = Sec(self.funcs.len() as u32, Vec::new());
        for &t in &self.funcs {
            uleb(&mut funcs.1, t as u64);
        }
        let limits = |v: Option<u32>, head: &[u8]| {
            let mut s = Sec::default();
            if let Some(min) = v {
                s.0 = 1;
                s.1.extend_from_slice(head);
                s.1.push(0);
                uleb(&mut s.1, min as u64);
            }
            s
        };
        let secs = [
            (1, types),
            (2, self.imports),
            (3, funcs),
            (4, limits(self.table, &[0x70])),
            (5, limits(self.memory, &[])),
            (6, self.globals),
            (7, self.exports),
            (9, self.elems),
            (10, self.bodies),
            (11, self.datas),
        ];
        let mut out = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        for (id, Sec(n, body)) in secs {
            if n > 0 {
                let mut content = Vec::new();
                uleb(&mut content, n as u64);
                content.extend(body);
                out.push(id);
                bytes(&mut out, &content);
            }
        }
        out
    }
}

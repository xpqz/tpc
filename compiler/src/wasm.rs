//! WASM binary emitter: LEB128 integers, a function-body builder and the module writer.
//!
//! A module is a header followed by sections in a fixed order (type, import, function, table,
//! memory, global, export, element, code, data), each written as `id, byte length, entry
//! count, entries`. `Code` builds one function body; its structured control flow (`block`,
//! `loop`, `if`) is addressed by `Label`, which `br` turns into the relative depth WASM
//! wants. Unbalanced `end`s are caught here, at the point of the mistake, rather than as an
//! invalid module later.

pub const I32: u8 = 0x7F;
pub const F64: u8 = 0x7C;
/// Block type of a block that leaves nothing on the stack.
pub const VOID: u8 = 0x40;
/// Export kinds.
pub const FUNC: u8 = 0;
pub const MEM: u8 = 2;
pub const GLOBAL: u8 = 3;

const FUNCREF: u8 = 0x70;
const FUNC_TYPE: u8 = 0x60;
const MODULE_HEADER: [u8; 8] = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];

const SECTION_TYPE: u8 = 1;
const SECTION_IMPORT: u8 = 2;
const SECTION_FUNCTION: u8 = 3;
const SECTION_TABLE: u8 = 4;
const SECTION_MEMORY: u8 = 5;
const SECTION_GLOBAL: u8 = 6;
const SECTION_EXPORT: u8 = 7;
const SECTION_ELEMENT: u8 = 9;
const SECTION_CODE: u8 = 10;
const SECTION_DATA: u8 = 11;

/// The instructions that `Code` itself writes; the rest of the instruction names live with the
/// code generator that uses them.
pub mod op {
    pub const BLOCK: u8 = 0x02;
    pub const LOOP: u8 = 0x03;
    pub const IF: u8 = 0x04;
    pub const ELSE: u8 = 0x05;
    pub const END: u8 = 0x0B;
    pub const BR: u8 = 0x0C;
    pub const BR_IF: u8 = 0x0D;
    pub const CALL: u8 = 0x10;
    pub const CALL_INDIRECT: u8 = 0x11;
    pub const LOCAL_GET: u8 = 0x20;
    pub const LOCAL_SET: u8 = 0x21;
    pub const LOCAL_TEE: u8 = 0x22;
    pub const I32_CONST: u8 = 0x41;
    pub const F64_CONST: u8 = 0x44;
    pub const PREFIX_FC: u8 = 0xFC;
    pub const I32_TRUNC_SAT_F64_S: u8 = 0x02;
    pub const MEMORY_COPY: u8 = 0x0A;
    pub const MEMORY_FILL: u8 = 0x0B;
}

/// Unsigned LEB128: seven bits per byte, low bits first, the high bit set on every byte but
/// the last.
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

/// Signed LEB128: as `uleb`, but the encoding ends once the remaining value is only sign
/// extension of bit 6 of the last byte (all zeros for a non-negative value, all ones for a
/// negative one). For example -1 is the single byte 0x7F.
pub fn sleb(out: &mut Vec<u8>, mut v: i64) {
    loop {
        let b = (v & 0x7F) as u8;
        v >>= 7;
        let sign_bit_set = b & 0x40 != 0;
        if (v == 0 && !sign_bit_set) || (v == -1 && sign_bit_set) {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

/// A length-prefixed byte string.
fn bytes(out: &mut Vec<u8>, b: &[u8]) {
    uleb(out, b.len() as u64);
    out.extend_from_slice(b);
}

/// A constant expression: `i32.const v; end`.
fn const_expr(out: &mut Vec<u8>, v: i32) {
    out.push(op::I32_CONST);
    sleb(out, v as i64);
    out.push(op::END);
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

    /// An instruction with one index immediate (`local.get`, `call`, ...).
    pub fn idx(&mut self, opcode: u8, n: u32) -> &mut Self {
        self.buf.push(opcode);
        uleb(&mut self.buf, n as u64);
        self
    }

    pub fn call(&mut self, function: u32) -> &mut Self {
        self.idx(op::CALL, function)
    }

    pub fn local_get(&mut self, local: u32) -> &mut Self {
        self.idx(op::LOCAL_GET, local)
    }

    pub fn local_set(&mut self, local: u32) -> &mut Self {
        self.idx(op::LOCAL_SET, local)
    }

    pub fn local_tee(&mut self, local: u32) -> &mut Self {
        self.idx(op::LOCAL_TEE, local)
    }

    pub fn i32_const(&mut self, v: i32) -> &mut Self {
        self.buf.push(op::I32_CONST);
        sleb(&mut self.buf, v as i64);
        self
    }

    pub fn f64_const(&mut self, v: f64) -> &mut Self {
        self.buf.push(op::F64_CONST);
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    /// A load or store: alignment hint, then the constant byte offset added to the address.
    pub fn mem(&mut self, opcode: u8, align: u32, offset: u32) -> &mut Self {
        self.idx(opcode, align);
        uleb(&mut self.buf, offset as u64);
        self
    }

    pub fn call_indirect(&mut self, ty: u32) -> &mut Self {
        self.idx(op::CALL_INDIRECT, ty).op(0)
    }

    /// Appends the instructions of another balanced body.
    pub fn append(&mut self, other: Code) {
        assert!(other.open.is_empty(), "unbalanced blocks");
        self.buf.extend_from_slice(&other.buf);
    }

    /// `memory.copy` of (dest, src, len) on the stack, memory 0 to memory 0.
    pub fn memory_copy(&mut self) -> &mut Self {
        self.idx(op::PREFIX_FC, op::MEMORY_COPY as u32).op(0).op(0)
    }

    /// `memory.fill` of (address, byte, len) on the stack.
    pub fn memory_fill(&mut self) -> &mut Self {
        self.idx(op::PREFIX_FC, op::MEMORY_FILL as u32).op(0)
    }

    /// `i32.trunc_sat_f64_s`: the f64 on the stack to i32, clamping instead of trapping.
    pub fn truncate_f64_saturating(&mut self) -> &mut Self {
        self.op(op::PREFIX_FC).op(op::I32_TRUNC_SAT_F64_S)
    }

    fn open(&mut self, opcode: u8, block_type: u8, is_if: bool) -> Label {
        self.buf.extend_from_slice(&[opcode, block_type]);
        self.open.push(is_if);
        Label(self.open.len() as u32 - 1)
    }

    pub fn block(&mut self, block_type: u8) -> Label {
        self.open(op::BLOCK, block_type, false)
    }

    pub fn loop_(&mut self, block_type: u8) -> Label {
        self.open(op::LOOP, block_type, false)
    }

    pub fn if_(&mut self, block_type: u8) -> Label {
        self.open(op::IF, block_type, true)
    }

    pub fn else_(&mut self) -> &mut Self {
        assert!(self.open.last() == Some(&true), "else outside if");
        *self.open.last_mut().unwrap() = false;
        self.op(op::ELSE)
    }

    pub fn end(&mut self) -> &mut Self {
        assert!(self.open.pop().is_some(), "unbalanced end");
        self.op(op::END)
    }

    /// How many blocks out from the innermost one `label` is.
    fn depth(&self, label: Label) -> u32 {
        assert!((label.0 as usize) < self.open.len(), "label is not open");
        self.open.len() as u32 - 1 - label.0
    }

    pub fn br(&mut self, label: Label) -> &mut Self {
        let depth = self.depth(label);
        self.idx(op::BR, depth)
    }

    pub fn br_if(&mut self, label: Label) -> &mut Self {
        let depth = self.depth(label);
        self.idx(op::BR_IF, depth)
    }
}

/// A section's entry count and its already encoded entries.
#[derive(Default)]
struct Section {
    count: u32,
    entries: Vec<u8>,
}

#[derive(PartialEq)]
struct FuncType {
    params: Vec<u8>,
    results: Vec<u8>,
}

#[derive(Default)]
pub struct Module {
    types: Vec<FuncType>,
    imports: Section,
    funcs: Vec<u32>,
    bodies: Section,
    table: Option<u32>,
    memory: Option<u32>,
    globals: Section,
    exports: Section,
    elems: Section,
    datas: Section,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    /// The index of the function type, adding it if it is new.
    pub fn ty(&mut self, params: &[u8], results: &[u8]) -> u32 {
        let signature = FuncType {
            params: params.to_vec(),
            results: results.to_vec(),
        };
        if let Some(index) = self.types.iter().position(|t| *t == signature) {
            return index as u32;
        }
        self.types.push(signature);
        self.types.len() as u32 - 1
    }

    pub fn import_func(&mut self, module: &str, name: &str, ty: u32) -> u32 {
        assert!(self.funcs.is_empty(), "import after function");
        bytes(&mut self.imports.entries, module.as_bytes());
        bytes(&mut self.imports.entries, name.as_bytes());
        self.imports.entries.push(0);
        uleb(&mut self.imports.entries, ty as u64);
        self.imports.count += 1;
        self.imports.count - 1
    }

    /// Adds a function body and returns its function index (imports come first).
    pub fn func(&mut self, ty: u32, locals: &[u8], mut body: Code) -> u32 {
        assert!(body.open.is_empty(), "unbalanced blocks at function end");
        // Locals are encoded as runs of (count, type).
        let mut runs: Vec<(u32, u8)> = Vec::new();
        for &local in locals {
            match runs.last_mut() {
                Some((count, ty)) if *ty == local => *count += 1,
                _ => runs.push((1, local)),
            }
        }
        let mut entry = Vec::new();
        uleb(&mut entry, runs.len() as u64);
        for (count, ty) in runs {
            uleb(&mut entry, count as u64);
            entry.push(ty);
        }
        entry.append(&mut body.buf);
        entry.push(op::END);
        bytes(&mut self.bodies.entries, &entry);
        self.bodies.count += 1;
        self.funcs.push(ty);
        self.imports.count + self.funcs.len() as u32 - 1
    }

    pub fn table(&mut self, min: u32) {
        self.table = Some(min);
    }

    pub fn memory(&mut self, min_pages: u32) {
        self.memory = Some(min_pages);
    }

    pub fn global_i32(&mut self, mutable: bool, init: i32) -> u32 {
        self.globals
            .entries
            .extend_from_slice(&[I32, mutable as u8]);
        const_expr(&mut self.globals.entries, init);
        self.globals.count += 1;
        self.globals.count - 1
    }

    pub fn export(&mut self, name: &str, kind: u8, idx: u32) {
        bytes(&mut self.exports.entries, name.as_bytes());
        self.exports.entries.push(kind);
        uleb(&mut self.exports.entries, idx as u64);
        self.exports.count += 1;
    }

    /// Initialises the table from `offset` with the given function indices.
    pub fn elem(&mut self, offset: u32, funcs: &[u32]) {
        self.elems.entries.push(0);
        const_expr(&mut self.elems.entries, offset as i32);
        uleb(&mut self.elems.entries, funcs.len() as u64);
        for &f in funcs {
            uleb(&mut self.elems.entries, f as u64);
        }
        self.elems.count += 1;
    }

    /// Initialises memory at `offset` with `data`.
    pub fn data(&mut self, offset: u32, data: &[u8]) {
        self.datas.entries.push(0);
        const_expr(&mut self.datas.entries, offset as i32);
        bytes(&mut self.datas.entries, data);
        self.datas.count += 1;
    }

    /// Sections in id order; empty ones are omitted.
    pub fn finish(self) -> Vec<u8> {
        let mut types = Section::default();
        for t in &self.types {
            types.entries.push(FUNC_TYPE);
            bytes(&mut types.entries, &t.params);
            bytes(&mut types.entries, &t.results);
            types.count += 1;
        }
        let mut funcs = Section {
            count: self.funcs.len() as u32,
            entries: Vec::new(),
        };
        for &t in &self.funcs {
            uleb(&mut funcs.entries, t as u64);
        }
        // Table and memory hold at most one entry: a minimum size and no maximum.
        let limits = |min: Option<u32>, element_type: &[u8]| {
            let mut section = Section::default();
            if let Some(min) = min {
                section.count = 1;
                section.entries.extend_from_slice(element_type);
                section.entries.push(0);
                uleb(&mut section.entries, min as u64);
            }
            section
        };
        let sections = [
            (SECTION_TYPE, types),
            (SECTION_IMPORT, self.imports),
            (SECTION_FUNCTION, funcs),
            (SECTION_TABLE, limits(self.table, &[FUNCREF])),
            (SECTION_MEMORY, limits(self.memory, &[])),
            (SECTION_GLOBAL, self.globals),
            (SECTION_EXPORT, self.exports),
            (SECTION_ELEMENT, self.elems),
            (SECTION_CODE, self.bodies),
            (SECTION_DATA, self.datas),
        ];
        let mut out = MODULE_HEADER.to_vec();
        for (id, section) in sections {
            if section.count > 0 {
                let mut content = Vec::new();
                uleb(&mut content, section.count as u64);
                content.extend(section.entries);
                out.push(id);
                bytes(&mut out, &content);
            }
        }
        out
    }
}

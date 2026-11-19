//! Walks the checked AST and emits the module (docs/abi.md).
//! Static data: a fixed header, the globals (`check::GLOBALS`..), then the string literals that
//! live bodies use. Bodies are emitted from a worklist (decision 14).
use crate::ast::{Ast, Id, Kind, List, Op};
use crate::check::{Info, H_ABS, H_ASSIGN, H_CAT, H_CMP, H_SETADD, H_SETCMP, H_SETIN, H_SETOP};
use crate::types::{Routine, SymKind, Symbol, Tag, Ty, Type};
use crate::wasm::{Code, Label, Module, F64, FUNC, GLOBAL, I32, MEM, VOID};

const PAGES: u32 = 16;
const HEADER: u32 = 256;
const LF: i32 = 256;
const SPACE: i32 = 257;
const FALSE: i32 = 258;
const TRUE: i32 = 263;
/// Scratch for digits and single characters.
const BUF: i32 = 267;
const BUF_LEN: i32 = 12;
const TOP: u32 = PAGES * 65536;
/// Bytes of a string temporary: the length byte and 255 characters.
const STRING_TEMP: u32 = 256;
const SET_TEMP: u32 = 32;
/// The shadow stack may use the top 64 KB of memory (docs/abi.md).
const STACK_LIMIT: i32 = (TOP - 65536) as i32;
const SP: u32 = 0;
const STDOUT: i32 = 1;

const UNREACHABLE: u8 = 0x00;
const CALL: u8 = 0x10;
const SELECT: u8 = 0x1B;
const LOCAL_GET: u8 = 0x20;
const LOCAL_SET: u8 = 0x21;
const LOCAL_TEE: u8 = 0x22;
const GLOBAL_GET: u8 = 0x23;
const GLOBAL_SET: u8 = 0x24;
const I32_LOAD: u8 = 0x28;
const I32_STORE: u8 = 0x36;
const I32_STORE8: u8 = 0x3A;
const RETURN: u8 = 0x0F;
const I32_EQZ: u8 = 0x45;
const I32_EQ: u8 = 0x46;
const I32_NE: u8 = 0x47;
const I32_LT_U: u8 = 0x49;
const I32_LE_S: u8 = 0x4C;
const I32_LE_U: u8 = 0x4D;
const I32_OR: u8 = 0x72;
const I32_GE_S: u8 = 0x4E;
const I32_GT_S: u8 = 0x4A;
const I32_LT_S: u8 = 0x48;
const I32_ADD: u8 = 0x6A;
const I32_MUL: u8 = 0x6C;
const I32_SUB: u8 = 0x6B;
const I32_DIV_S: u8 = 0x6D;
const I32_DIV_U: u8 = 0x6E;
const I32_REM_S: u8 = 0x6F;
const I32_REM_U: u8 = 0x70;
const I32_AND: u8 = 0x71;
const I32_XOR: u8 = 0x73;
const I32_SHL: u8 = 0x74;
const I32_SHR_S: u8 = 0x75;
const F64_EQ: u8 = 0x61;
const F64_LOAD: u8 = 0x2B;
const F64_STORE: u8 = 0x39;
const F64_TRUNC: u8 = 0x9D;
const F64_SUB: u8 = 0xA1;
const F64_MUL: u8 = 0xA2;
/// Bytes of the buffer `fmt_real` fills: the widest field (255) or the longest fixed number.
const REAL_BUF: u32 = 512;

/// Function indices: imports first, then the helpers in definition order.
const HOST_WRITE: u32 = 0;
const HOST_RTERROR: u32 = 1;
const WSTR: u32 = 2;
const WINT: u32 = 3;
const QUOT: u32 = 4;
const RDIV: u32 = 5;
/// A module that formats Reals imports `fmt_real` (index 2): every later index shifts by
/// `Layout::fmt`. A module that divides Reals defines `RDIV`: the routines shift by `Layout::div`.
const HOST_FMT: u32 = 2;
/// Index of the first user routine; the main program follows the last.
const ROUTINES: u32 = 5;

/// Module-wide numbers the bodies refer to, assigned on first reference.
struct Layout {
    /// Address of each string literal that a live body uses.
    addr: Vec<Option<u32>>,
    /// The bytes of those literals, from `data_end` on.
    lits: Vec<u8>,
    data_end: u32,
    /// WASM function index of each referenced routine.
    index: Vec<Option<u32>>,
    /// The referenced routines in index order; their bodies are emitted in this order.
    order: Vec<u32>,
    /// WASM type of each procedural type, by `Ty` index.
    wtypes: Vec<u32>,
    /// Table index of each routine whose address is taken.
    slots: Vec<u32>,
    /// 1 when the module imports `fmt_real`, else 0.
    fmt: u32,
    /// 1 when the module defines `RDIV`, else 0.
    div: u32,
    /// The support (`FMT`, `DIV`) that the bodies called.
    used: u8,
}

impl Layout {
    fn func(&mut self, r: u32) -> u32 {
        if self.index[r as usize].is_none() {
            let first = ROUTINES + self.fmt + self.div;
            self.index[r as usize] = Some(first + self.order.len() as u32);
            self.order.push(r);
        }
        self.index[r as usize].unwrap_or_default()
    }

    /// The address of the literal as a Pascal string: its length byte, then the characters.
    fn lit(&mut self, s: u32, bytes: &[u8]) -> u32 {
        let at = self.data_end + self.lits.len() as u32;
        let addr = *self.addr[s as usize].get_or_insert(at);
        if addr == at {
            self.lits.push(bytes.len() as u8);
            self.lits.extend_from_slice(bytes);
        }
        addr
    }
}

struct Gen<'a> {
    ast: &'a Ast,
    info: &'a Info,
    code: Code,
    layout: &'a mut Layout,
    /// Number of WASM locals in use: parameters, the result, declared locals, temporaries.
    locals: u32,
    /// The local holding the frame pointer, once the routine has a frame.
    fp: Option<u32>,
    /// Offset of the string temporaries in the frame: the end of the declared frame.
    frame: u32,
    /// Bytes of temporaries in use, and the most a statement used.
    temp: u32,
    temps: u32,
    /// Lexical depth of the routine; a routine of depth 2 or more has its static link in
    /// local 0.
    depth: u32,
    /// The block around the routine body; `Exit` branches to it.
    exit: Label,
    /// Each `with` record designator and the local holding its address.
    withs: Vec<(Id, u32)>,
}

const FMT: u8 = 1;
const DIV: u8 = 2;

/// Generates with the Real support found so far until the bodies need no more.
pub fn gen(ast: &Ast, info: &Info, root: Id) -> Vec<u8> {
    let mut have = 0;
    loop {
        let (m, used) = pass(ast, info, root, have);
        if used & !have == 0 {
            return m;
        }
        have |= used;
    }
}

/// One generation of the module with the support in `have`; returns the support the bodies called.
fn pass(ast: &Ast, info: &Info, root: Id, have: u8) -> (Vec<u8>, u8) {
    let mut m = Module::new();
    let mut wtypes = vec![0; info.types.v.len()];
    for (i, t) in info.types.v.iter().enumerate() {
        if let Type::Proc { params, result } = t {
            let results: &[u8] = if *result == Ty::NONE { &[] } else { &[I32] };
            wtypes[i] = m.ty(&vec![I32; params.len()], results);
        }
    }
    // Table index 0 is the trap stub.
    let mut slots = Vec::new();
    for r in &info.routines {
        slots.push(1 + slots.iter().filter(|&&s| s != 0).count() as u32);
        if !r.addressed {
            *slots.last_mut().unwrap() = 0;
        }
    }
    let mut layout = Layout {
        addr: vec![None; ast.strs.len()],
        lits: Vec::new(),
        data_end: info.data_end,
        index: vec![None; info.routines.len()],
        order: Vec::new(),
        wtypes,
        slots,
        fmt: (have & FMT) as u32,
        div: (have & DIV != 0) as u32,
        used: 0,
    };
    let write = m.ty(&[I32, I32, I32], &[]);
    let rterror = m.ty(&[I32], &[]);
    m.import_func("host", "write", write);
    m.import_func("host", "rterror", rterror);
    if have & FMT != 0 {
        let fmt = m.ty(&[F64, I32, I32, I32], &[I32]);
        m.import_func("host", "fmt_real", fmt);
    }
    m.memory(PAGES);
    m.global_i32(true, TOP as i32);
    m.func(write, &[], write_string());
    let two = m.ty(&[I32, I32], &[]);
    m.func(two, &[I32, I32, I32], write_integer(layout.fmt));
    let three = m.ty(&[I32, I32, I32], &[I32]);
    m.func(three, &[], quotient());
    if have & DIV != 0 {
        let div = m.ty(&[F64, F64], &[F64]);
        m.func(div, &[], real_quotient());
    }
    // Roots: the main program and every address-taken routine; the rest by reference.
    let Kind::Program(_, _, main) = ast.nodes[root as usize].kind else {
        unreachable!("the root is a program")
    };
    let (main_code, main_locals) = body(ast, info, &mut layout, None, main);
    for (i, r) in info.routines.iter().enumerate() {
        if r.addressed {
            layout.func(i as u32);
        }
    }
    let mut done = 0;
    while done < layout.order.len() {
        let r = &info.routines[layout.order[done] as usize];
        let results: &[u8] = if r.func && !r.result_ptr { &[I32] } else { &[] };
        let hidden = (r.depth > 1) as usize + r.result_ptr as usize;
        let ty = m.ty(&vec![I32; r.params.len() + hidden], results);
        let (code, locals) = body(ast, info, &mut layout, Some(r), r.body);
        m.func(ty, &vec![I32; locals], code);
        done += 1;
    }
    let ty = m.ty(&[], &[]);
    let f = m.func(ty, &vec![I32; main_locals], main_code);
    let stub = {
        let mut c = Code::new();
        c.op(UNREACHABLE);
        m.func(ty, &[], c)
    };
    let mut table = vec![stub];
    for (i, r) in info.routines.iter().enumerate() {
        if r.addressed {
            table.push(layout.func(i as u32));
        }
    }
    m.table(table.len() as u32);
    m.elem(0, &table);
    m.export("main", FUNC, f);
    m.export("sp", GLOBAL, SP);
    m.export("memory", MEM, 0);
    m.data(HEADER, b"\n FALSETRUE");
    m.data(info.data_end, &layout.lits);
    // The heap starts, 8-aligned, after the literals: `HeapOrg` and `HeapPtr` both hold it.
    let start = ((info.data_end + layout.lits.len() as u32 + 7) & !7).to_le_bytes();
    m.data(info.heap, &[start, start].concat());
    (m.finish(), layout.used)
}

/// A routine body wrapped in `block $exit`, between the frame prologue and epilogue, followed
/// by the result read for a function; the main program (`None`) has a frame only for string
/// temporaries. The body is generated first: the frame also holds the temporaries it used.
/// Returns the code and the number of locals after the parameters.
fn body(
    ast: &Ast,
    info: &Info,
    layout: &mut Layout,
    routine: Option<&Routine>,
    stmt: Id,
) -> (Code, usize) {
    let depth = routine.map_or(0, |r| r.depth);
    let hidden = routine.map_or(0, |r| (r.depth > 1) as u32 + r.result_ptr as u32);
    let nparams = routine.map_or(0, |r| r.params.len() as u32) + hidden;
    let mut g = Gen {
        ast,
        info,
        code: Code::new(),
        layout,
        locals: nparams + routine.map_or(0, |r| r.nlocals),
        fp: None,
        frame: routine.map_or(0, |r| r.frame),
        temp: 0,
        temps: 0,
        depth,
        exit: Label(0),
        withs: Vec::new(),
    };
    let framed = routine.filter(|r| r.framed);
    if framed.is_some() {
        g.fp();
    }
    g.exit = g.code.block(VOID);
    g.stmt(stmt);
    g.code.end();
    let inner = std::mem::take(&mut g.code);
    if let Some(fp) = g.fp {
        let frame = g.frame + g.temps;
        g.code
            .idx(GLOBAL_GET, SP)
            .i32_const(frame as i32)
            .op(I32_SUB);
        g.code
            .idx(LOCAL_TEE, fp)
            .i32_const(STACK_LIMIT)
            .op(I32_LT_U);
        g.code.if_(VOID);
        g.code.i32_const(202).idx(CALL, HOST_RTERROR).end();
        g.code.idx(LOCAL_GET, fp).idx(GLOBAL_SET, SP);
        if depth > 1 && framed.is_some() {
            g.code.idx(LOCAL_GET, fp).idx(LOCAL_GET, 0);
            g.code.mem(I32_STORE, 0, 0);
        }
        for &(local, offset, ty) in framed.map_or(&[][..], |r| &r.copies[..]) {
            g.code.idx(LOCAL_GET, fp);
            if info.types.is_str(ty) {
                let assign = g.layout.func(info.helpers[H_ASSIGN]);
                g.code
                    .i32_const(offset as i32)
                    .op(I32_ADD)
                    .idx(LOCAL_GET, local);
                g.code
                    .i32_const(info.types.capacity(ty) as i32)
                    .idx(CALL, assign);
            } else if info.types.in_memory(ty) {
                let size = info.types.size(ty) as i32;
                g.code
                    .i32_const(offset as i32)
                    .op(I32_ADD)
                    .idx(LOCAL_GET, local);
                g.code.i32_const(size).memory_copy();
            } else {
                g.code.idx(LOCAL_GET, local);
                g.store(ty, offset);
            }
        }
        g.code.append(inner);
        g.code
            .idx(LOCAL_GET, fp)
            .i32_const(frame as i32)
            .op(I32_ADD);
        g.code.idx(GLOBAL_SET, SP);
    } else {
        g.code = inner;
    }
    if let Some(r) = routine.filter(|r| r.func && !r.result_ptr) {
        g.load(info.syms[r.result_sym.0 as usize], r.result);
    }
    (g.code, (g.locals - nparams) as usize)
}

/// `wstr(ptr, len, width)`: right-justified in `width`.
fn write_string() -> Code {
    let mut c = Code::new();
    let pad = c.loop_(VOID);
    c.idx(LOCAL_GET, 2).idx(LOCAL_GET, 1).op(I32_GT_S).if_(VOID);
    c.i32_const(STDOUT)
        .i32_const(SPACE)
        .i32_const(1)
        .idx(CALL, HOST_WRITE);
    c.idx(LOCAL_GET, 2)
        .i32_const(1)
        .op(I32_SUB)
        .idx(LOCAL_SET, 2)
        .br(pad);
    c.end().end();
    c.i32_const(STDOUT)
        .idx(LOCAL_GET, 0)
        .idx(LOCAL_GET, 1)
        .idx(CALL, HOST_WRITE);
    c
}

/// `wint(value, width)`: decimal digits built backwards in `BUF`; locals p, negative, u.
fn write_integer(shift: u32) -> Code {
    let (p, neg, u) = (2, 3, 4);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0)
        .i32_const(0)
        .op(I32_LT_S)
        .idx(LOCAL_SET, neg);
    c.idx(LOCAL_GET, 0).idx(LOCAL_SET, u);
    c.idx(LOCAL_GET, neg).if_(VOID);
    c.i32_const(0)
        .idx(LOCAL_GET, 0)
        .op(I32_SUB)
        .idx(LOCAL_SET, u)
        .end();
    c.i32_const(BUF + BUF_LEN).idx(LOCAL_SET, p);
    let digits = c.loop_(VOID);
    c.idx(LOCAL_GET, p)
        .i32_const(1)
        .op(I32_SUB)
        .idx(LOCAL_SET, p);
    c.idx(LOCAL_GET, p)
        .idx(LOCAL_GET, u)
        .i32_const(10)
        .op(I32_REM_U);
    c.i32_const(b'0' as i32).op(I32_ADD).mem(I32_STORE8, 0, 0);
    c.idx(LOCAL_GET, u)
        .i32_const(10)
        .op(I32_DIV_U)
        .idx(LOCAL_SET, u);
    c.idx(LOCAL_GET, u).br_if(digits).end();
    c.idx(LOCAL_GET, neg).if_(VOID);
    c.idx(LOCAL_GET, p)
        .i32_const(1)
        .op(I32_SUB)
        .idx(LOCAL_SET, p);
    c.idx(LOCAL_GET, p)
        .i32_const(b'-' as i32)
        .mem(I32_STORE8, 0, 0)
        .end();
    c.idx(LOCAL_GET, p)
        .i32_const(BUF + BUF_LEN)
        .idx(LOCAL_GET, p)
        .op(I32_SUB);
    c.idx(LOCAL_GET, 1).idx(CALL, WSTR + shift);
    c
}

/// `quot(a, b, flags)`: `a div b`, or `a mod b` when bit 0 is set. Bit 1 marks 16-bit
/// operands. A zero divisor is runtime error 200, as is -32768 by -1 on 16 bits; on 32
/// bits `a div -1` wraps and `a mod -1` is 0.
fn quotient() -> Code {
    let (a, b, flags) = (0, 1, 2);
    let mut c = Code::new();
    c.idx(LOCAL_GET, b).op(I32_EQZ).if_(VOID);
    c.i32_const(200).idx(CALL, HOST_RTERROR).end();
    c.idx(LOCAL_GET, b).i32_const(-1).op(I32_EQ).if_(VOID);
    c.idx(LOCAL_GET, flags).i32_const(1).op(I32_GT_S);
    c.idx(LOCAL_GET, a)
        .i32_const(-32768)
        .op(I32_EQ)
        .op(I32_AND)
        .if_(VOID);
    c.i32_const(200).idx(CALL, HOST_RTERROR).end();
    c.idx(LOCAL_GET, flags).i32_const(1).op(I32_AND).if_(I32);
    c.i32_const(0)
        .else_()
        .i32_const(0)
        .idx(LOCAL_GET, a)
        .op(I32_SUB)
        .end();
    c.op(RETURN).end();
    c.idx(LOCAL_GET, a).idx(LOCAL_GET, b).op(I32_REM_S);
    c.idx(LOCAL_GET, a).idx(LOCAL_GET, b).op(I32_DIV_S);
    c.idx(LOCAL_GET, flags).i32_const(1).op(I32_AND).op(SELECT);
    c
}

/// `rdiv(a, b)`: `a / b`, but a zero divisor is runtime error 200.
fn real_quotient() -> Code {
    let mut c = Code::new();
    c.idx(LOCAL_GET, 1).f64_const(0.0).op(F64_EQ).if_(VOID);
    c.i32_const(200).idx(CALL, HOST_RTERROR).end();
    c.idx(LOCAL_GET, 0).idx(LOCAL_GET, 1).op(0xA3);
    c
}

fn real_op(op: Op) -> u8 {
    match op {
        Op::Add => 0xA0,
        Op::Sub => F64_SUB,
        Op::Mul => F64_MUL,
        Op::Eq => F64_EQ,
        Op::Ne => 0x62,
        Op::Lt => 0x63,
        Op::Gt => 0x64,
        Op::Le => 0x65,
        Op::Ge => 0x66,
        _ => unreachable!("Real operators"),
    }
}

fn binary_op(op: Op) -> u8 {
    match op {
        Op::Add => 0x6A,
        Op::Sub => 0x6B,
        Op::Mul => 0x6C,
        Op::Div => I32_DIV_S,
        Op::Mod => I32_REM_S,
        Op::And => 0x71,
        Op::Or => 0x72,
        Op::Xor => 0x73,
        Op::Shl => 0x74,
        Op::Shr => 0x76,
        Op::Eq => I32_EQ,
        Op::Ne => I32_NE,
        Op::Lt => I32_LT_S,
        Op::Gt => I32_GT_S,
        Op::Le => I32_LE_S,
        Op::Ge => I32_GE_S,
        _ => unreachable!("unary"),
    }
}

impl Gen<'_> {
    /// Re-normalises the i32 on the stack to the integer type `ty` (plan §4.6).
    fn narrow(&mut self, ty: Ty) {
        if !matches!(self.info.types.tag(ty), Some(Tag::Int | Tag::Char)) {
            return;
        }
        let (_, _, size, signed) = self.info.types.ord(ty);
        match (size, signed) {
            (4, _) => {}
            (_, false) => {
                self.code.i32_const((1 << (8 * size)) - 1).op(I32_AND);
            }
            _ => {
                let s = 32 - 8 * size as i32;
                self.code
                    .i32_const(s)
                    .op(I32_SHL)
                    .i32_const(s)
                    .op(I32_SHR_S);
            }
        }
    }

    fn new_local(&mut self) -> u32 {
        self.locals += 1;
        self.locals - 1
    }

    /// The frame pointer local, created on first use.
    fn fp(&mut self) -> u32 {
        if self.fp.is_none() {
            self.fp = Some(self.new_local());
        }
        self.fp.unwrap_or_default()
    }

    /// A new temporary of `size` bytes, returned as its frame offset. A statement's
    /// temporaries are not reused within it.
    fn new_temp(&mut self, size: u32) -> u32 {
        self.temp += size;
        self.temps = self.temps.max(self.temp);
        self.frame + self.temp - size
    }

    fn helper(&mut self, h: usize) {
        let f = self.layout.func(self.info.helpers[h]);
        self.code.idx(CALL, f);
    }

    fn temp_addr(&mut self, offset: u32) {
        let fp = self.fp();
        self.code
            .idx(LOCAL_GET, fp)
            .i32_const(offset as i32)
            .op(I32_ADD);
    }

    /// Pushes the address of the string value of `e`, which the caller must not modify:
    /// literals and variables in place, everything else in a temporary (decision 13).
    fn str(&mut self, e: Id) {
        match self.ast.nodes[e as usize].kind {
            Kind::StrLit(s) => {
                let addr = self.layout.lit(s, &self.ast.strs[s as usize]);
                self.code.i32_const(addr as i32);
            }
            Kind::CharToStr(c) => {
                let t = self.new_temp(STRING_TEMP);
                self.temp_addr(t);
                self.code.i32_const(1).mem(I32_STORE8, 0, 0);
                self.temp_addr(t);
                self.expr(c);
                self.code.mem(I32_STORE8, 0, 1);
                self.temp_addr(t);
            }
            Kind::Binary(Op::Add, a, b) => {
                let t = self.new_temp(STRING_TEMP);
                self.temp_addr(t);
                self.str(a);
                self.str(b);
                let cat = self.layout.func(self.info.helpers[H_CAT]);
                self.code.idx(CALL, cat);
                self.temp_addr(t);
            }
            Kind::Invoke(r, args) => {
                let t = self.invoke(r, args).expect("string function");
                self.temp_addr(t);
            }
            Kind::Fmt(v, w, d) => {
                // The digits go to the temporary after its length byte, which is clamped to 255.
                let (t, n) = (self.new_temp(REAL_BUF), self.new_local());
                self.temp_addr(t);
                self.code.i32_const(255);
                self.real(v);
                self.expr(w);
                match d {
                    Some(d) => self.expr(d),
                    None => {
                        self.code.i32_const(-1);
                    }
                }
                self.temp_addr(t);
                self.layout.used |= FMT;
                self.code.i32_const(1).op(I32_ADD).idx(CALL, HOST_FMT);
                self.code.idx(LOCAL_TEE, n).idx(LOCAL_GET, n);
                self.code.i32_const(255).op(I32_GT_S).op(SELECT);
                self.code.mem(I32_STORE8, 0, 0);
                self.temp_addr(t);
            }
            _ => self.address_of(e),
        }
    }

    /// Pushes the frame pointer of the routine `up` levels above the current one: the
    /// own `fp`, else the static link walked `up - 1` times.
    fn frame_ptr(&mut self, up: u32) {
        if up == 0 {
            let fp = self.fp();
            self.code.idx(LOCAL_GET, fp);
            return;
        }
        self.code.idx(LOCAL_GET, 0);
        for _ in 1..up {
            self.code.mem(I32_LOAD, 0, 0);
        }
    }

    /// Pushes the address base of a variable in memory and returns the offset to add.
    fn base(&mut self, s: Symbol) -> u32 {
        match s.kind {
            SymKind::Var => {
                self.code.i32_const(s.val as i32);
            }
            SymKind::Frame => {
                self.frame_ptr(self.depth - s.depth);
                return s.val as u32;
            }
            SymKind::FrameRef => {
                self.frame_ptr(self.depth - s.depth);
                self.code.mem(I32_LOAD, 0, s.val as u32);
            }
            _ => {
                self.code.idx(LOCAL_GET, s.val as u32);
            }
        }
        0
    }

    /// Stores the i32 on the stack to the address below it, truncated to `ty`.
    fn store(&mut self, ty: Ty, offset: u32) {
        let size = self.info.types.ord(ty).2;
        self.code.mem(
            [I32_STORE8, 0x3B, 0, I32_STORE][size as usize - 1],
            0,
            offset,
        );
    }

    fn is_agg(&self, e: Id) -> bool {
        self.info.types.is_agg(self.ast.nodes[e as usize].ty)
    }

    fn symbol(&self, v: Id) -> Symbol {
        let Kind::Var(sym) = self.ast.nodes[v as usize].kind else {
            unreachable!("checked variable")
        };
        self.info.syms[sym.0 as usize]
    }

    fn load_op(&self, ty: Ty) -> u8 {
        match self.info.types.ord(ty) {
            (_, _, 1, true) => 0x2C,
            (_, _, 1, false) => 0x2D,
            (_, _, 2, true) => 0x2E,
            (_, _, 2, false) => 0x2F,
            _ => I32_LOAD,
        }
    }

    /// Pushes the value of a variable of type `ty`.
    fn load(&mut self, s: Symbol, ty: Ty) {
        if s.kind == SymKind::Local {
            self.code.idx(LOCAL_GET, s.val as u32);
            return;
        }
        let offset = self.base(s);
        self.code.mem(self.load_op(ty), 0, offset);
    }

    /// Pushes the address of the designator `e` and returns the constant offset to add. A
    /// `with` record is read from its local, so its designator is evaluated once.
    fn addr(&mut self, e: Id) -> u32 {
        if let Some(&(_, local)) = self.withs.iter().find(|w| w.0 == e) {
            self.code.idx(LOCAL_GET, local);
            return 0;
        }
        match self.ast.nodes[e as usize].kind {
            Kind::Var(_) => {
                let s = self.symbol(e);
                self.base(s)
            }
            Kind::Deref(p) => {
                self.expr(p);
                0
            }
            Kind::Field(base, offset) => self.addr(base) + offset,
            Kind::Index(base, index, _) => {
                let offset = self.addr(base);
                let types = &self.info.types;
                let (_, lo, _, elem) = types
                    .array_of(self.ast.nodes[base as usize].ty)
                    .expect("checked array");
                let size = types.size(elem);
                if let Kind::IntLit(c) = self.ast.nodes[index as usize].kind {
                    return offset + (c - lo) as u32 * size;
                }
                self.expr(index);
                if lo != 0 {
                    self.code.i32_const(lo as i32).op(I32_SUB);
                }
                if size != 1 {
                    self.code.i32_const(size as i32).op(I32_MUL);
                }
                self.code.op(I32_ADD);
                offset
            }
            _ => unreachable!("designators"),
        }
    }

    /// Pushes the address of the designator `e`, or of the temporary holding the set value `e`
    /// (a constructor, or the result of `+ * -`).
    fn address_of(&mut self, e: Id) {
        match self.ast.nodes[e as usize].kind {
            Kind::SetLit(items) => {
                let t = self.new_temp(SET_TEMP);
                for _ in 0..3 {
                    self.temp_addr(t);
                }
                self.code.i32_const(2);
                self.helper(H_SETOP);
                let single = self.new_local();
                for &item in self.ast.items(items) {
                    self.temp_addr(t);
                    let (lo, hi) = match self.ast.nodes[item as usize].kind {
                        Kind::Range(lo, hi) => (lo, Some(hi)),
                        _ => (item, None),
                    };
                    self.expr(lo);
                    match hi {
                        Some(hi) => self.expr(hi),
                        None => {
                            self.code.idx(LOCAL_TEE, single).idx(LOCAL_GET, single);
                        }
                    }
                    self.helper(H_SETADD);
                }
                self.temp_addr(t);
                return;
            }
            Kind::Binary(op, a, b) => {
                let t = self.new_temp(SET_TEMP);
                self.temp_addr(t);
                self.address_of(a);
                self.address_of(b);
                self.code.i32_const(match op {
                    Op::Add => 0,
                    Op::Mul => 1,
                    _ => 2,
                });
                self.helper(H_SETOP);
                self.temp_addr(t);
                return;
            }
            _ => {}
        }
        let offset = self.addr(e);
        if offset > 0 {
            self.code.i32_const(offset as i32).op(I32_ADD);
        }
    }

    fn get(&mut self, v: Id) {
        let ty = self.ast.nodes[v as usize].ty;
        if matches!(self.ast.nodes[v as usize].kind, Kind::Var(_)) {
            self.load(self.symbol(v), ty);
        } else {
            let offset = self.addr(v);
            self.code.mem(self.load_op(ty), 0, offset);
        }
    }

    /// Assigns the value that `value` pushes to the variable `v`, truncated to its type: a
    /// store truncates in memory, a local is narrowed.
    fn put(&mut self, v: Id, value: impl FnOnce(&mut Self)) {
        let ty = self.ast.nodes[v as usize].ty;
        if matches!(self.ast.nodes[v as usize].kind, Kind::Var(_))
            && self.symbol(v).kind == SymKind::Local
        {
            value(self);
            self.narrow(ty);
            self.code.idx(LOCAL_SET, self.symbol(v).val as u32);
        } else {
            let offset = self.addr(v);
            value(self);
            self.store(ty, offset);
        }
    }

    /// A call of routine `r`: value arguments converted to the parameter types, `var`
    /// arguments passed as addresses.
    /// A call of a String function also passes the address of a new temporary for the
    /// result, which is returned.
    fn invoke(&mut self, r: u32, args: List) -> Option<u32> {
        let info = self.info;
        let routine = &info.routines[r as usize];
        if routine.depth > 1 {
            self.frame_ptr(self.depth + 1 - routine.depth);
        }
        let result = routine
            .result_ptr
            .then(|| self.new_temp(info.types.size(routine.result)));
        if let Some(t) = result {
            self.temp_addr(t);
        }
        self.push_args(args, &routine.params, &routine.byref);
        let index = self.layout.func(r);
        self.code.idx(CALL, index);
        result
    }

    /// Value arguments converted to the parameter types, `var` arguments as addresses.
    fn push_args(&mut self, args: List, params: &[Ty], refs: &[bool]) {
        for ((&a, &ty), &byref) in self.ast.items(args).iter().zip(params).zip(refs) {
            if byref || self.info.types.is_agg(ty) {
                self.address_of(a);
            } else if ty == Ty::REAL {
                self.real_addr(a);
            } else if self.info.types.is_str(ty) {
                self.str(a);
            } else {
                self.expr(a);
                self.narrow(ty);
            }
        }
    }

    /// A call through the procedural value `callee`: arguments, the table index, then
    /// `call_indirect` with the WASM type of the signature.
    fn indirect(&mut self, callee: Id, args: List) {
        let ty = self.ast.nodes[callee as usize].ty;
        let (params, refs, _) = self.info.types.sig(ty).expect("procedural callee");
        self.push_args(args, &params, &refs);
        self.expr(callee);
        self.code.call_indirect(self.layout.wtypes[ty.0 as usize]);
    }

    /// A unary operator or intrinsic of result type `ty`.
    fn unary(&mut self, op: Op, a: Id, ty: Ty) {
        if op == Op::Neg {
            self.code.i32_const(0);
        }
        self.expr(a);
        match op {
            Op::Not if ty == Ty::BOOLEAN => {
                self.code.op(I32_EQZ);
            }
            Op::Neg => {
                self.code.op(I32_SUB);
            }
            Op::Not => {
                self.code.i32_const(-1).op(I32_XOR);
            }
            Op::Chr => {
                self.code.i32_const(255).op(I32_AND);
            }
            Op::Odd => {
                self.code.i32_const(1).op(I32_AND);
            }
            Op::Succ | Op::Pred => {
                let step = if op == Op::Succ { 1 } else { -1 };
                self.code.i32_const(step).op(I32_ADD);
            }
            Op::Abs => {
                let index = self.layout.func(self.info.helpers[H_ABS]);
                self.code.idx(CALL, index);
            }
            Op::Sqr => {
                let t = self.new_local();
                self.code.idx(LOCAL_SET, t);
                self.code.idx(LOCAL_GET, t).idx(LOCAL_GET, t).op(I32_MUL);
            }
            _ => {}
        }
        self.narrow(ty);
    }

    fn width(&mut self, w: Option<Id>) {
        match w {
            Some(w) => self.expr(w),
            None => {
                self.code.i32_const(0);
            }
        }
    }

    fn stmt(&mut self, id: Id) {
        self.temp = 0;
        match self.ast.nodes[id as usize].kind {
            Kind::Compound(l) => self.ast.items(l).iter().for_each(|&s| self.stmt(s)),
            Kind::Assign(target, value)
                if self.info.types.is_str(self.ast.nodes[target as usize].ty) =>
            {
                let capacity = self.info.types.capacity(self.ast.nodes[target as usize].ty);
                self.address_of(target);
                self.str(value);
                let assign = self.layout.func(self.info.helpers[H_ASSIGN]);
                self.code.i32_const(capacity as i32).idx(CALL, assign);
            }
            Kind::Assign(target, value) if self.is_agg(target) => {
                let size = self.info.types.size(self.ast.nodes[target as usize].ty);
                self.address_of(target);
                self.address_of(value);
                self.code.i32_const(size as i32).memory_copy();
            }
            Kind::Assign(target, value) if self.ast.nodes[target as usize].ty == Ty::REAL => {
                let offset = self.addr(target);
                self.real(value);
                self.code.mem(F64_STORE, 0, offset);
            }
            Kind::Assign(target, value) => self.put(target, |g| g.expr(value)),
            Kind::With(records, body) => {
                let outer = self.withs.len();
                for &d in self.ast.items(records) {
                    let local = self.new_local();
                    self.address_of(d);
                    self.code.idx(LOCAL_SET, local);
                    self.withs.push((d, local));
                }
                self.stmt(body);
                self.withs.truncate(outer);
            }
            Kind::Invoke(r, args) => {
                self.invoke(r, args);
            }
            Kind::Indirect(callee, args) => self.indirect(callee, args),
            Kind::RunErr(code) => {
                self.expr(code);
                self.code.idx(CALL, HOST_RTERROR);
            }
            Kind::Exit => {
                self.code.br(self.exit);
            }
            Kind::If(cond, then, other) => {
                self.expr(cond);
                self.code.if_(VOID);
                self.stmt(then);
                if let Some(s) = other {
                    self.code.else_();
                    self.stmt(s);
                }
                self.code.end();
            }
            Kind::While(cond, body) => {
                let exit = self.code.block(VOID);
                let top = self.code.loop_(VOID);
                self.expr(cond);
                self.code.op(I32_EQZ).br_if(exit);
                self.stmt(body);
                self.code.br(top).end().end();
            }
            Kind::Repeat(body, cond) => {
                let top = self.code.loop_(VOID);
                self.ast.items(body).iter().for_each(|&s| self.stmt(s));
                self.expr(cond);
                self.code.op(I32_EQZ).br_if(top).end();
            }
            Kind::Case(selector, arms, other) => {
                // The selector is converted to Integer; each arm is a chain of range tests.
                let local = self.new_local();
                self.expr(selector);
                self.narrow(Ty::INTEGER);
                self.code.idx(LOCAL_SET, local);
                let exit = self.code.block(VOID);
                for &arm in self.ast.items(arms) {
                    let Kind::Arm(labels, body) = self.ast.nodes[arm as usize].kind else {
                        unreachable!("checked arm")
                    };
                    self.code.i32_const(0);
                    for &label in self.ast.items(labels) {
                        let Kind::Range(lo, hi) = self.ast.nodes[label as usize].kind else {
                            unreachable!("checked range")
                        };
                        let (Kind::IntLit(lo), Kind::IntLit(hi)) = (
                            self.ast.nodes[lo as usize].kind,
                            self.ast.nodes[hi as usize].kind,
                        ) else {
                            unreachable!("constant labels")
                        };
                        if lo <= hi {
                            self.code
                                .idx(LOCAL_GET, local)
                                .i32_const(lo as i32)
                                .op(I32_SUB);
                            self.code
                                .i32_const((hi - lo) as i32)
                                .op(I32_LE_U)
                                .op(I32_OR);
                        }
                    }
                    self.code.if_(VOID);
                    self.stmt(body);
                    self.code.br(exit).end();
                }
                self.ast.items(other).iter().for_each(|&s| self.stmt(s));
                self.code.end();
            }
            Kind::For(var, first, last, body, down) => {
                // The last value is compared for equality before stepping, so the
                // control variable never passes its type's limit.
                let end = self.new_local();
                self.put(var, |g| g.expr(first));
                self.expr(last);
                self.code.idx(LOCAL_SET, end);
                let exit = self.code.block(VOID);
                self.get(var);
                self.code.idx(LOCAL_GET, end);
                self.code
                    .op(if down { I32_LT_S } else { I32_GT_S })
                    .br_if(exit);
                let top = self.code.loop_(VOID);
                self.stmt(body);
                self.get(var);
                self.code.idx(LOCAL_GET, end).op(I32_EQ).br_if(exit);
                self.put(var, |g| {
                    g.get(var);
                    g.code.i32_const(if down { -1 } else { 1 }).op(I32_ADD);
                });
                self.code.br(top).end().end();
            }
            Kind::Write(args, ln) => {
                self.ast.items(args).iter().for_each(|&a| self.write_arg(a));
                if ln {
                    self.code.i32_const(STDOUT).i32_const(LF).i32_const(1);
                    self.code.idx(CALL, HOST_WRITE);
                }
            }
            _ => unreachable!("checked statements"),
        }
    }

    fn write_arg(&mut self, a: Id) {
        let (e, w, d) = match self.ast.nodes[a as usize].kind {
            Kind::Fmt(e, w, d) => (e, Some(w), d),
            _ => (a, None, None),
        };
        let n = self.ast.nodes[e as usize];
        match (self.info.types.tag(n.ty), n.kind) {
            (None, _) if n.ty == Ty::REAL => {
                // `fmt_real` (-1 for an absent width or decimals) fills the buffer, then write.
                let t = self.new_temp(REAL_BUF);
                self.code.i32_const(STDOUT);
                self.temp_addr(t);
                self.real(e);
                for f in [w, d] {
                    match f {
                        Some(f) => self.expr(f),
                        None => {
                            self.code.i32_const(-1);
                        }
                    }
                }
                self.temp_addr(t);
                self.layout.used |= FMT;
                self.code.idx(CALL, HOST_FMT).idx(CALL, HOST_WRITE);
            }
            (None, Kind::StrLit(s)) => {
                let bytes = &self.ast.strs[s as usize];
                let addr = self.layout.lit(s, bytes);
                self.code
                    .i32_const(addr as i32 + 1)
                    .i32_const(bytes.len() as i32);
                self.width(w);
                self.code.idx(CALL, WSTR + self.layout.fmt);
            }
            (None, _) if self.info.types.is_str(n.ty) => {
                let t = self.new_local();
                self.str(e);
                self.code.idx(LOCAL_SET, t);
                self.code.idx(LOCAL_GET, t).i32_const(1).op(I32_ADD);
                self.code.idx(LOCAL_GET, t).mem(0x2D, 0, 0);
                self.width(w);
                self.code.idx(CALL, WSTR + self.layout.fmt);
            }
            (Some(Tag::Int), _) => {
                self.expr(e);
                self.width(w);
                self.code.idx(CALL, WINT + self.layout.fmt);
            }
            (Some(Tag::Char), _) => {
                self.code.i32_const(BUF);
                self.expr(e);
                self.code.mem(I32_STORE8, 0, 0).i32_const(BUF).i32_const(1);
                self.width(w);
                self.code.idx(CALL, WSTR + self.layout.fmt);
            }
            _ => {
                let t = self.new_local();
                self.expr(e);
                self.code.idx(LOCAL_SET, t);
                self.code
                    .i32_const(TRUE)
                    .i32_const(FALSE)
                    .idx(LOCAL_GET, t)
                    .op(SELECT);
                self.code
                    .i32_const(4)
                    .i32_const(5)
                    .idx(LOCAL_GET, t)
                    .op(SELECT);
                self.width(w);
                self.code.idx(CALL, WSTR + self.layout.fmt);
            }
        }
    }

    /// Evaluates the Real `e` into a new temporary and returns its frame offset.
    fn spill(&mut self, e: Id) -> u32 {
        let t = self.new_temp(8);
        self.temp_addr(t);
        self.real(e);
        self.code.mem(F64_STORE, 0, 0);
        t
    }

    fn reload(&mut self, t: u32) {
        self.temp_addr(t);
        self.code.mem(F64_LOAD, 0, 0);
    }

    /// Pushes `x - trunc(x)` for the value in temporary `t`.
    fn frac(&mut self, t: u32) {
        self.reload(t);
        self.reload(t);
        self.code.op(F64_TRUNC).op(F64_SUB);
    }

    /// Pushes the address of a Real argument: a designator in place (the callee copies it),
    /// any other value in a temporary.
    fn real_addr(&mut self, e: Id) {
        match self.ast.nodes[e as usize].kind {
            Kind::Var(_) | Kind::Index(..) | Kind::Field(..) | Kind::Deref(_) => self.address_of(e),
            _ => {
                let t = self.spill(e);
                self.temp_addr(t);
            }
        }
    }

    /// Pushes the f64 value of the Real expression `e`.
    fn real(&mut self, e: Id) {
        let n = self.ast.nodes[e as usize];
        match n.kind {
            Kind::RealLit(v) => {
                self.code.f64_const(v);
            }
            Kind::IntToReal(a) => {
                self.expr(a);
                self.code.op(0xB7);
            }
            Kind::Var(_) | Kind::Index(..) | Kind::Field(..) | Kind::Deref(_) => {
                let offset = self.addr(e);
                self.code.mem(F64_LOAD, 0, offset);
            }
            Kind::Invoke(r, args) => {
                let t = self.invoke(r, args).expect("Real function");
                self.reload(t);
            }
            Kind::Binary(Op::Slash, a, b) => {
                self.real(a);
                self.real(b);
                self.layout.used |= DIV;
                self.code.idx(CALL, RDIV + self.layout.fmt);
            }
            Kind::Binary(op, a, b) => {
                self.real(a);
                self.real(b);
                self.code.op(real_op(op));
            }
            Kind::Unary(Op::Sqr, a) => {
                let t = self.spill(a);
                self.reload(t);
                self.reload(t);
                self.code.op(F64_MUL);
            }
            Kind::Unary(Op::Frac, a) => {
                let t = self.spill(a);
                self.frac(t);
            }
            Kind::Unary(op, a) => {
                self.real(a);
                self.code.op(match op {
                    Op::Neg => 0x9A,
                    Op::Abs => 0x99,
                    Op::Sqrt => 0x9F,
                    _ => F64_TRUNC,
                });
            }
            _ => unreachable!("checked Real expressions"),
        }
    }

    fn expr(&mut self, e: Id) {
        let n = self.ast.nodes[e as usize];
        match n.kind {
            Kind::IntLit(v) => {
                self.code.i32_const(v as i32);
            }
            Kind::Var(_) | Kind::Index(..) | Kind::Field(..) | Kind::Deref(_) => self.get(e),
            Kind::AddrOf(a) => self.address_of(a),
            Kind::ProcRef(r) => {
                self.code.i32_const(self.layout.slots[r as usize] as i32);
            }
            Kind::Invoke(r, args) => {
                self.invoke(r, args);
            }
            Kind::Indirect(callee, args) => self.indirect(callee, args),
            Kind::Binary(op @ (Op::And | Op::Or), a, b) if n.ty == Ty::BOOLEAN => {
                // `{$B-}`: the right operand is evaluated only when it can matter.
                self.expr(a);
                self.code.if_(I32);
                if op == Op::And {
                    self.expr(b);
                    self.code.else_().i32_const(0);
                } else {
                    self.code.i32_const(1).else_();
                    self.expr(b);
                }
                self.code.end();
            }
            Kind::Unary(op @ (Op::Round | Op::Trunc), a) => {
                if op == Op::Trunc {
                    self.real(a);
                } else {
                    // Half away from zero: trunc(x) + trunc(2 * frac(x)).
                    let t = self.spill(a);
                    self.reload(t);
                    self.code.op(F64_TRUNC);
                    self.frac(t);
                    self.code.f64_const(2.0).op(F64_MUL).op(F64_TRUNC).op(0xA0);
                }
                // `i32.trunc_sat_f64_s`
                self.code.op(0xFC).op(2);
            }
            Kind::Unary(op, a) => self.unary(op, a, n.ty),
            Kind::Binary(op, a, b) if self.ast.nodes[a as usize].ty == Ty::REAL => {
                self.real(a);
                self.real(b);
                self.code.op(real_op(op));
            }
            Kind::Binary(Op::In, a, set) => {
                self.address_of(set);
                self.expr(a);
                self.helper(H_SETIN);
            }
            Kind::Binary(op, a, b) if self.info.types.is_set(self.ast.nodes[a as usize].ty) => {
                self.address_of(a);
                self.address_of(b);
                self.helper(H_SETCMP);
                let mask = match op {
                    Op::Le => 1,
                    Op::Ge => 2,
                    _ => 3,
                };
                let test = binary_op(if op == Op::Ne { Op::Ne } else { Op::Eq });
                self.code.i32_const(mask).op(I32_AND).i32_const(0).op(test);
            }
            Kind::Binary(op, a, b) if self.info.types.is_str(self.ast.nodes[a as usize].ty) => {
                self.str(a);
                self.str(b);
                let cmp = self.layout.func(self.info.helpers[H_CMP]);
                self.code.idx(CALL, cmp).i32_const(0).op(binary_op(op));
            }
            Kind::Binary(op, a, b) => {
                self.expr(a);
                if op == Op::Shr && n.ty == Ty::INTEGER {
                    self.code.i32_const(0xFFFF).op(I32_AND);
                }
                self.expr(b);
                if matches!(op, Op::Div | Op::Mod) {
                    let flags = (op == Op::Mod) as i32 + 2 * (n.ty == Ty::INTEGER) as i32;
                    self.code.i32_const(flags).idx(CALL, QUOT + self.layout.fmt);
                    return;
                }
                self.code.op(binary_op(op));
                if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Shl | Op::Shr) {
                    self.narrow(n.ty);
                }
            }
            _ => unreachable!("checked expressions"),
        }
    }
}

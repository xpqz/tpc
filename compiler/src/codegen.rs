//! Walks the checked AST and emits the module (docs/abi.md).
//! Static data: a fixed header, the globals (`check::GLOBALS`..), then string literals.
use crate::ast::{Ast, Id, Kind, Op};
use crate::check::Info;
use crate::types::{Tag, Ty};
use crate::wasm::{Code, Module, FUNC, I32, MEM, VOID};

const PAGES: u32 = 16;
const HEADER: u32 = 256;
const LF: i32 = 256;
const SPACE: i32 = 257;
const FALSE: i32 = 258;
const TRUE: i32 = 263;
/// Scratch for digits and single characters.
const BUF: i32 = 267;
const BUF_LEN: i32 = 12;
const STDOUT: i32 = 1;

const CALL: u8 = 0x10;
const SELECT: u8 = 0x1B;
const LOCAL_GET: u8 = 0x20;
const LOCAL_SET: u8 = 0x21;
const I32_LOAD: u8 = 0x28;
const I32_STORE: u8 = 0x36;
const I32_STORE8: u8 = 0x3A;
const RETURN: u8 = 0x0F;
const I32_EQZ: u8 = 0x45;
const I32_EQ: u8 = 0x46;
const I32_NE: u8 = 0x47;
const I32_LE_S: u8 = 0x4C;
const I32_LE_U: u8 = 0x4D;
const I32_OR: u8 = 0x72;
const I32_GE_S: u8 = 0x4E;
const I32_GT_S: u8 = 0x4A;
const I32_LT_S: u8 = 0x48;
const I32_ADD: u8 = 0x6A;
const I32_SUB: u8 = 0x6B;
const I32_DIV_S: u8 = 0x6D;
const I32_DIV_U: u8 = 0x6E;
const I32_REM_S: u8 = 0x6F;
const I32_REM_U: u8 = 0x70;
const I32_AND: u8 = 0x71;
const I32_XOR: u8 = 0x73;
const I32_SHL: u8 = 0x74;
const I32_SHR_S: u8 = 0x75;

/// Function indices: imports first, then the helpers in definition order.
const HOST_WRITE: u32 = 0;
const HOST_RTERROR: u32 = 1;
const WSTR: u32 = 2;
const WINT: u32 = 3;
const QUOT: u32 = 4;

struct Gen<'a> {
    ast: &'a Ast,
    info: &'a Info,
    code: Code,
    /// Address of each string literal.
    addr: Vec<u32>,
    /// WASM locals of `main` in use; local 0 is the Boolean scratch.
    locals: u32,
}

pub fn gen(ast: &Ast, info: &Info, root: Id) -> Vec<u8> {
    let mut lits = Vec::new();
    let mut addr = Vec::new();
    for s in &ast.strs {
        addr.push(info.data_end + lits.len() as u32);
        lits.extend_from_slice(s);
    }
    let mut g = Gen {
        ast,
        info,
        code: Code::new(),
        addr,
        locals: 1,
    };
    let Kind::Program(_, _, body) = ast.nodes[root as usize].kind else {
        unreachable!("the root is a program")
    };
    g.stmt(body);
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let main = m.ty(&[], &[]);
    let rterror = m.ty(&[I32], &[]);
    m.import_func("host", "write", write);
    m.import_func("host", "rterror", rterror);
    m.memory(PAGES);
    m.global_i32(true, (PAGES * 65536) as i32);
    m.func(write, &[], write_string());
    let two = m.ty(&[I32, I32], &[]);
    m.func(two, &[I32, I32, I32], write_integer());
    let three = m.ty(&[I32, I32, I32], &[I32]);
    m.func(three, &[], quotient());
    let f = m.func(main, &vec![I32; g.locals as usize], g.code);
    m.export("main", FUNC, f);
    m.export("memory", MEM, 0);
    m.data(HEADER, b"\n FALSETRUE");
    m.data(info.data_end, &lits);
    m.finish()
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
fn write_integer() -> Code {
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
    c.idx(LOCAL_GET, 1).idx(CALL, WSTR);
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
        Op::Neg | Op::Not => unreachable!("unary"),
    }
}

impl Gen<'_> {
    /// Re-normalises the i32 on the stack to the integer type `ty` (plan §4.6).
    fn narrow(&mut self, ty: Ty) {
        if self.info.types.tag(ty) != Some(Tag::Int) {
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

    fn var_addr(&mut self, v: Id) {
        let Kind::Var(sym) = self.ast.nodes[v as usize].kind else {
            unreachable!("checked variable")
        };
        self.code
            .i32_const(self.info.syms[sym.0 as usize].val as i32);
    }

    /// Replaces the address on the stack by the variable's value.
    fn load(&mut self, v: Id) {
        let (_, _, size, signed) = self.info.types.ord(self.ast.nodes[v as usize].ty);
        let op = match (size, signed) {
            (1, true) => 0x2C,
            (1, false) => 0x2D,
            (2, true) => 0x2E,
            (2, false) => 0x2F,
            _ => I32_LOAD,
        };
        self.code.mem(op, 0, 0);
    }

    /// Stores the value on the stack at the address below it, truncating to the type.
    fn store(&mut self, v: Id) {
        let size = self.info.types.ord(self.ast.nodes[v as usize].ty).2;
        self.code
            .mem([I32_STORE8, 0x3B, 0, I32_STORE][size as usize - 1], 0, 0);
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
        match self.ast.nodes[id as usize].kind {
            Kind::Compound(l) => self.ast.items(l).iter().for_each(|&s| self.stmt(s)),
            Kind::Assign(target, value) => {
                self.var_addr(target);
                self.expr(value);
                self.store(target);
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
                let local = self.locals;
                self.locals += 1;
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
                let end = self.locals;
                self.locals += 1;
                self.var_addr(var);
                self.expr(first);
                self.store(var);
                self.expr(last);
                self.code.idx(LOCAL_SET, end);
                let exit = self.code.block(VOID);
                self.var_addr(var);
                self.load(var);
                self.code.idx(LOCAL_GET, end);
                self.code
                    .op(if down { I32_LT_S } else { I32_GT_S })
                    .br_if(exit);
                let top = self.code.loop_(VOID);
                self.stmt(body);
                self.var_addr(var);
                self.load(var);
                self.code.idx(LOCAL_GET, end).op(I32_EQ).br_if(exit);
                self.var_addr(var);
                self.var_addr(var);
                self.load(var);
                self.code.i32_const(if down { -1 } else { 1 }).op(I32_ADD);
                self.store(var);
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
        let (e, w) = match self.ast.nodes[a as usize].kind {
            Kind::Fmt(e, w) => (e, Some(w)),
            _ => (a, None),
        };
        let n = self.ast.nodes[e as usize];
        match (self.info.types.tag(n.ty), n.kind) {
            (None, Kind::StrLit(s)) => {
                let len = self.ast.strs[s as usize].len();
                self.code
                    .i32_const(self.addr[s as usize] as i32)
                    .i32_const(len as i32);
                self.width(w);
                self.code.idx(CALL, WSTR);
            }
            (Some(Tag::Int), _) => {
                self.expr(e);
                self.width(w);
                self.code.idx(CALL, WINT);
            }
            (Some(Tag::Char), _) => {
                self.code.i32_const(BUF);
                self.expr(e);
                self.code.mem(I32_STORE8, 0, 0).i32_const(BUF).i32_const(1);
                self.width(w);
                self.code.idx(CALL, WSTR);
            }
            _ => {
                self.expr(e);
                self.code.idx(LOCAL_SET, 0);
                self.code
                    .i32_const(TRUE)
                    .i32_const(FALSE)
                    .idx(LOCAL_GET, 0)
                    .op(SELECT);
                self.code
                    .i32_const(4)
                    .i32_const(5)
                    .idx(LOCAL_GET, 0)
                    .op(SELECT);
                self.width(w);
                self.code.idx(CALL, WSTR);
            }
        }
    }

    fn expr(&mut self, e: Id) {
        let n = self.ast.nodes[e as usize];
        match n.kind {
            Kind::IntLit(v) => {
                self.code.i32_const(v as i32);
            }
            Kind::Var(_) => {
                self.var_addr(e);
                self.load(e);
            }
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
            Kind::Unary(Op::Not, a) if n.ty == Ty::BOOLEAN => {
                self.expr(a);
                self.code.op(I32_EQZ);
            }
            Kind::Unary(op, a) => {
                if op == Op::Neg {
                    self.code.i32_const(0);
                }
                self.expr(a);
                if op == Op::Neg {
                    self.code.op(I32_SUB);
                } else {
                    self.code.i32_const(-1).op(I32_XOR);
                }
                self.narrow(n.ty);
            }
            Kind::Binary(op, a, b) => {
                self.expr(a);
                if op == Op::Shr && n.ty == Ty::INTEGER {
                    self.code.i32_const(0xFFFF).op(I32_AND);
                }
                self.expr(b);
                if matches!(op, Op::Div | Op::Mod) {
                    let flags = (op == Op::Mod) as i32 + 2 * (n.ty == Ty::INTEGER) as i32;
                    self.code.i32_const(flags).idx(CALL, QUOT);
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

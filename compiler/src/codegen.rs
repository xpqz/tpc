//! Code generation: walks the checked AST and emits the WASM module (docs/abi.md).
//!
//! Every Pascal expression becomes WASM stack-machine code that leaves its value on the
//! stack; a statement leaves nothing. Control flow uses WASM's structured `block`/`loop`/`if`
//! and branches (`br`) to a label: `while` is a `block` around a `loop`, `exit` branches to a
//! block around the routine body.
//!
//! Memory (linear, 16 pages): a fixed header (the characters `write` needs and a digit
//! buffer), the globals from `symbols::GLOBALS`, the string literals the live code uses, the
//! heap (grows up), and the shadow stack of frames at the top (grows down). A routine that
//! needs addressable storage allocates a frame there: the declared frame laid out by the
//! checker, then the temporaries below. A nested routine receives its parent's frame pointer
//! as a static link (local 0); a variable of an enclosing routine is reached by following
//! the links, each stored at offset 0 of its frame.
//!
//! Temporaries: a String value lives in memory, so an expression that produces one (a
//! concatenation, a Char converted to String, a function result, a formatted Real) gets a
//! 256-byte slot in the current frame. Slots are not reused within a statement; the next
//! statement starts again from the same base, and the frame is sized for the largest use.
//! Sets are 32-byte temporaries in the same way.
//!
//! Dead code elimination takes two phases. The mark phase runs the generator over the whole
//! program and throws the code away; all it keeps is which routines, runtime helpers and
//! host imports the program reaches, starting from the main program and every routine whose
//! address is taken. Function numbers are then assigned (imports first, then the defined
//! functions) and the emit phase runs the generator again to produce the module with the
//! final numbers. A routine that is never reached is never emitted.
use crate::ast::{Ast, Id, Kind, List, Op};
use crate::check::Info;
use crate::symbols::Helper;
use crate::types::{FrameCopy, ParamType, Routine, SymKind, Symbol, Tag, Ty, Type};
use crate::wasm::{Code, Label, Module, F64, FUNC, GLOBAL, I32, MEM, VOID};

/// Memory map. The first bytes hold what `write` needs; the globals follow at
/// `symbols::GLOBALS`, and the literals after the globals.
const HEADER: u32 = 256;
const LF: i32 = 256;
const SPACE: i32 = LF + 1;
const FALSE: i32 = SPACE + 1;
const TRUE: i32 = FALSE + 5;
/// Scratch for digits and single characters.
const BUF: i32 = TRUE + 4;
const BUF_LEN: i32 = 12;
const PAGES: u32 = 16;
const TOP: u32 = PAGES * 65536;
/// The shadow stack may use the top 64 KB of memory (docs/abi.md); the heap ends below it,
/// which is `HeapLimit` in `runtime/system.pas`.
const STACK_LIMIT: i32 = (TOP - 65536) as i32;
/// Bytes of a string temporary: the length byte and 255 characters.
const STRING_TEMP: u32 = 256;
/// Bytes of a set temporary.
const SET_TEMP: u32 = crate::types::SET_BYTES;
/// Bytes of the buffer `fmt_real` fills: the widest field (255) or the longest fixed number.
const REAL_BUF: u32 = 512;
/// The WASM global holding the stack pointer.
const SP: u32 = 0;
const STDOUT: i32 = 1;
/// Longest String.
const MAX_STRING: i32 = 255;
/// The runtime error numbers raised by generated code.
const ERROR_DIVISION_BY_ZERO: i32 = 200;
const ERROR_STACK_OVERFLOW: i32 = 202;

/// The operation selector of `SetOp` in `runtime/system.pas`.
const SET_UNION: i32 = 0;
const SET_INTERSECTION: i32 = 1;
const SET_DIFFERENCE: i32 = 2;
/// The bits `SetCmp` returns: the first set has an element the second lacks, and the reverse.
const SET_FIRST_HAS_EXTRA: i32 = 1;
const SET_SECOND_HAS_EXTRA: i32 = 2;
/// The flag bits of `quot`: remainder instead of quotient, and 16-bit operands.
const QUOT_REMAINDER: i32 = 1;
const QUOT_16_BIT: i32 = 2;

const UNREACHABLE: u8 = 0x00;
const RETURN: u8 = 0x0F;
const SELECT: u8 = 0x1B;
const GLOBAL_GET: u8 = 0x23;
const GLOBAL_SET: u8 = 0x24;
const I32_LOAD: u8 = 0x28;
const F64_LOAD: u8 = 0x2B;
const I32_LOAD8_S: u8 = 0x2C;
const I32_LOAD8_U: u8 = 0x2D;
const I32_LOAD16_S: u8 = 0x2E;
const I32_LOAD16_U: u8 = 0x2F;
const I32_STORE: u8 = 0x36;
const F64_STORE: u8 = 0x39;
const I32_STORE8: u8 = 0x3A;
const I32_STORE16: u8 = 0x3B;
const I32_EQZ: u8 = 0x45;
const I32_EQ: u8 = 0x46;
const I32_NE: u8 = 0x47;
const I32_LT_S: u8 = 0x48;
const I32_LT_U: u8 = 0x49;
const I32_GT_S: u8 = 0x4A;
const I32_LE_S: u8 = 0x4C;
const I32_LE_U: u8 = 0x4D;
const I32_GE_S: u8 = 0x4E;
const F64_EQ: u8 = 0x61;
const F64_NE: u8 = 0x62;
const F64_LT: u8 = 0x63;
const F64_GT: u8 = 0x64;
const F64_LE: u8 = 0x65;
const F64_GE: u8 = 0x66;
const I32_ADD: u8 = 0x6A;
const I32_SUB: u8 = 0x6B;
const I32_MUL: u8 = 0x6C;
const I32_DIV_S: u8 = 0x6D;
const I32_DIV_U: u8 = 0x6E;
const I32_REM_S: u8 = 0x6F;
const I32_REM_U: u8 = 0x70;
const I32_AND: u8 = 0x71;
const I32_OR: u8 = 0x72;
const I32_XOR: u8 = 0x73;
const I32_SHL: u8 = 0x74;
const I32_SHR_S: u8 = 0x75;
const I32_SHR_U: u8 = 0x76;
const F64_ABS: u8 = 0x99;
const F64_NEG: u8 = 0x9A;
const F64_TRUNC: u8 = 0x9D;
const F64_SQRT: u8 = 0x9F;
const F64_ADD: u8 = 0xA0;
const F64_SUB: u8 = 0xA1;
const F64_MUL: u8 = 0xA2;
const F64_DIV: u8 = 0xA3;
const F64_CONVERT_I32_S: u8 = 0xB7;

/// What the mark phase found: the defined routines to emit and the `external` routines to
/// import, each in the order they were reached, and whether the program divides Reals.
struct Reached {
    routines: Vec<u32>,
    imports: Vec<u32>,
    uses_real_div: bool,
}

/// WASM function numbers: the imports first, in the order they were reached (`write` and
/// `rterror` always lead, because the built-in functions call them), then the defined
/// functions: the three built-in helpers, `rdiv` if used, the reached routines, the main
/// program and the table's trap stub.
struct Indexes {
    host_write: u32,
    host_rterror: u32,
    write_string: u32,
    write_integer: u32,
    quotient: u32,
    real_div: Option<u32>,
    first_routine: u32,
}

impl Indexes {
    fn new(import_count: u32, host_write: u32, host_rterror: u32, uses_real_div: bool) -> Self {
        let first_defined = import_count;
        Indexes {
            host_write,
            host_rterror,
            write_string: first_defined,
            write_integer: first_defined + 1,
            quotient: first_defined + 2,
            real_div: uses_real_div.then_some(first_defined + 3),
            first_routine: first_defined + 3 + uses_real_div as u32,
        }
    }
}

/// Module-wide numbers and addresses the bodies refer to.
struct Layout {
    /// Address of each string literal that a live body uses.
    literal_addr: Vec<Option<u32>>,
    /// The bytes of those literals, from `data_end` on.
    literals: Vec<u8>,
    /// Whether a live body names each typed constant (`Info::typed_consts`); only those are
    /// emitted as data.
    typed_const_used: Vec<bool>,
    data_end: u32,
    /// True during the mark phase: routines are reached as the bodies mention them.
    marking: bool,
    /// The reached defined routines in emission order, and the reached `external` ones in
    /// import order.
    reached: Vec<u32>,
    imports: Vec<u32>,
    /// Position of each routine in `reached` or in `imports`, once reached.
    position: Vec<Option<u32>>,
    /// Whether each routine is `external`.
    external: Vec<bool>,
    uses_real_div: bool,
    indexes: Indexes,
    /// WASM type of each procedural type, by `Ty` index.
    wasm_types: Vec<u32>,
    /// Table index of each routine whose address is taken, 0 for the others.
    table_slots: Vec<u32>,
}

impl Layout {
    /// A layout for the mark phase. There are no function numbers yet, so any number does.
    fn for_marking(ast: &Ast, info: &Info) -> Self {
        Layout {
            literal_addr: vec![None; ast.strs.len()],
            literals: Vec::new(),
            typed_const_used: vec![false; info.typed_consts.len()],
            data_end: info.data_end,
            marking: true,
            reached: Vec::new(),
            imports: Vec::new(),
            position: vec![None; info.routines.len()],
            external: info.routines.iter().map(|r| r.import.is_some()).collect(),
            uses_real_div: false,
            indexes: Indexes::new(0, 0, 0, false),
            wasm_types: vec![0; info.types.count()],
            table_slots: vec![0; info.routines.len()],
        }
    }

    /// A layout for the emit phase: the routines and support found by marking are numbered.
    fn for_emitting(ast: &Ast, info: &Info, reached: &Reached, wasm_types: Vec<u32>) -> Self {
        let mut position = vec![None; info.routines.len()];
        for list in [&reached.routines, &reached.imports] {
            for (at, &r) in list.iter().enumerate() {
                position[r as usize] = Some(at as u32);
            }
        }
        let import_number =
            |helper| position[info.helper(helper) as usize].expect("always reached");
        let indexes = Indexes::new(
            reached.imports.len() as u32,
            import_number(Helper::HostWrite),
            import_number(Helper::HostRtError),
            reached.uses_real_div,
        );
        Layout {
            marking: false,
            reached: reached.routines.clone(),
            imports: reached.imports.clone(),
            position,
            uses_real_div: reached.uses_real_div,
            indexes,
            wasm_types,
            table_slots: table_slots(info),
            ..Layout::for_marking(ast, info)
        }
    }

    /// The function number of routine `r`; while marking, the first mention reaches it. An
    /// `external` routine is an import, numbered before every defined function.
    fn routine_index(&mut self, r: u32) -> u32 {
        let is_import = self.external[r as usize];
        let position = match self.position[r as usize] {
            Some(position) => position,
            None => {
                assert!(self.marking, "routine {r} was not reached while marking");
                let list = if is_import {
                    &mut self.imports
                } else {
                    &mut self.reached
                };
                list.push(r);
                let position = list.len() as u32 - 1;
                self.position[r as usize] = Some(position);
                position
            }
        };
        if is_import {
            position
        } else {
            self.indexes.first_routine + position
        }
    }

    /// The function number of the Real division helper, which marks it as needed.
    fn real_div_index(&mut self) -> u32 {
        self.uses_real_div = true;
        self.indexes.real_div.unwrap_or_default()
    }

    /// The address of the literal as a Pascal string: its length byte, then the characters.
    fn literal(&mut self, s: u32, bytes: &[u8]) -> u32 {
        if let Some(addr) = self.literal_addr[s as usize] {
            return addr;
        }
        let addr = self.data_end + self.literals.len() as u32;
        self.literal_addr[s as usize] = Some(addr);
        self.literals.push(bytes.len() as u8);
        self.literals.extend_from_slice(bytes);
        addr
    }
}

/// Table index of each routine whose address is taken; table index 0 is the trap stub, so
/// the slots start at 1. Routines whose address is not taken have no slot (0).
fn table_slots(info: &Info) -> Vec<u32> {
    let mut next_slot = 1;
    let mut slots = Vec::new();
    for routine in &info.routines {
        if routine.addressed {
            slots.push(next_slot);
            next_slot += 1;
        } else {
            slots.push(0);
        }
    }
    slots
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
    /// Bytes of the frame the checker laid out; the temporaries follow.
    declared_frame_size: u32,
    /// Bytes of temporaries in use by the current statement, and the most any statement used.
    temp_in_use: u32,
    temp_peak: u32,
    /// Lexical depth of the routine; a routine of depth 2 or more has its static link in
    /// local 0.
    depth: u32,
    /// The block around the routine body; `Exit` branches to it.
    exit: Label,
    /// Each `with` record designator and the local holding its address.
    withs: Vec<(Id, u32)>,
}

/// The generated code of one body and its number of locals after the parameters.
struct Body {
    code: Code,
    locals: usize,
}

/// Generates the module: mark what is reachable, then emit it.
pub fn gen(ast: &Ast, info: &Info, root: Id) -> Vec<u8> {
    let reached = mark(ast, info, root);
    emit(ast, info, root, &reached)
}

/// The mark phase: generates every body, discarding the code, to find what is reached.
fn mark(ast: &Ast, info: &Info, root: Id) -> Reached {
    let mut layout = Layout::for_marking(ast, info);
    // The built-in functions call `write` and `rterror`, so every module imports them, first.
    layout.routine_index(info.helper(Helper::HostWrite));
    layout.routine_index(info.helper(Helper::HostRtError));
    generate_bodies(ast, info, root, &mut layout);
    Reached {
        routines: layout.reached,
        imports: layout.imports,
        uses_real_div: layout.uses_real_div,
    }
}

/// Generates the main program, then the routines in the order they were reached. While
/// marking, generating a body can reach further routines, which join the end of the list.
fn generate_bodies(ast: &Ast, info: &Info, root: Id, layout: &mut Layout) -> (Body, Vec<Body>) {
    let Kind::Program { body: main, .. } = ast.kind(root) else {
        unreachable!("the root is a program")
    };
    let main = body(ast, info, layout, None, main);
    // A routine whose address is taken may be called through the table from anywhere.
    for (i, r) in info.routines.iter().enumerate() {
        if r.addressed {
            layout.routine_index(i as u32);
        }
    }
    let mut routines = Vec::new();
    while routines.len() < layout.reached.len() {
        let r = &info.routines[layout.reached[routines.len()] as usize];
        routines.push(body(ast, info, layout, Some(r), r.body));
    }
    (main, routines)
}

/// The WASM function type of each procedural type, registered in `m`.
fn register_procedural_types(m: &mut Module, info: &Info) -> Vec<u32> {
    let mut wasm_types = vec![0; info.types.count()];
    for (i, t) in info.types.iter().enumerate() {
        if let Type::Proc { params, result } = t {
            let results: &[u8] = if *result == Ty::NONE { &[] } else { &[I32] };
            wasm_types[i] = m.ty(&vec![I32; params.len()], results);
        }
    }
    wasm_types
}

/// The emit phase: the module with the routines found by `mark`, numbered.
fn emit(ast: &Ast, info: &Info, root: Id, reached: &Reached) -> Vec<u8> {
    let mut m = Module::new();
    let wasm_types = register_procedural_types(&mut m, info);
    let mut layout = Layout::for_emitting(ast, info, reached, wasm_types);
    emit_imports(&mut m, ast, info, reached);
    m.memory(PAGES);
    m.global_i32(true, TOP as i32);
    emit_builtin_functions(&mut m, &layout);
    let (main, routines) = generate_bodies(ast, info, root, &mut layout);
    for (&r, routine_body) in reached.routines.iter().zip(routines) {
        let r = &info.routines[r as usize];
        let results: &[u8] = if r.is_function && !r.result_ptr {
            &[I32]
        } else {
            &[]
        };
        let ty = m.ty(
            &vec![I32; r.params.len() + r.hidden_params() as usize],
            results,
        );
        m.func(ty, &vec![I32; routine_body.locals], routine_body.code);
    }
    let no_params = m.ty(&[], &[]);
    let main_function = m.func(no_params, &vec![I32; main.locals], main.code);
    let trap_stub = {
        let mut code = Code::new();
        code.op(UNREACHABLE);
        m.func(no_params, &[], code)
    };
    emit_table_and_exports(&mut m, info, &layout, trap_stub, main_function);
    m.data(HEADER, b"\n FALSETRUE");
    emit_typed_constants(&mut m, ast, info, &layout);
    m.data(info.data_end, &layout.literals);
    // The heap starts, 8-aligned, after the literals: `HeapOrg` and `HeapPtr` both hold it.
    let heap_start = align_up(info.data_end + layout.literals.len() as u32, 8).to_le_bytes();
    m.data(info.heap, &[heap_start, heap_start].concat());
    m.finish()
}

/// One data segment, at its variable's address, for each typed constant a live body names.
fn emit_typed_constants(m: &mut Module, ast: &Ast, info: &Info, layout: &Layout) {
    for (constant, &used) in info.typed_consts.iter().zip(&layout.typed_const_used) {
        if used {
            let image = static_image(ast, info, constant.ty, constant.init);
            m.data(constant.address, &image);
        }
    }
}

/// The initial memory of a typed constant of type `ty`: the bytes a variable of that type
/// holds, `Types::size(ty)` of them. Integers, Booleans, Chars, enumerations and pointers
/// are little-endian in their storage size; a Real is an f64; a string is its length byte,
/// the characters cut to the capacity and zero padding up to it; a set is 32 bytes with bit
/// `i % 8` of byte `i / 8` for element `i`; a character array is the string's characters;
/// an array is its elements one after another; a record is zeros with each initialised field
/// placed at its offset, the entries being the first fields in order.
fn static_image(ast: &Ast, info: &Info, ty: Ty, init: Id) -> Vec<u8> {
    let types = &info.types;
    match ast.kind(init) {
        Kind::IntLit(value) => value.to_le_bytes()[..types.ord(ty).size as usize].to_vec(),
        Kind::RealLit(value) => value.to_le_bytes().to_vec(),
        Kind::StrLit(s) if types.is_str(ty) => {
            let text = &ast.strs[s as usize];
            let capacity = types.capacity(ty) as usize;
            let length = text.len().min(capacity);
            let mut image = vec![0; capacity + 1];
            image[0] = length as u8;
            image[1..=length].copy_from_slice(&text[..length]);
            image
        }
        Kind::StrLit(s) => ast.strs[s as usize].clone(),
        Kind::SetLit(items) => {
            let mut image = vec![0u8; SET_TEMP as usize];
            for &item in ast.items(items) {
                let (first, last) = match ast.kind(item) {
                    Kind::Range(lo, hi) => (lo, hi),
                    _ => (item, item),
                };
                let (Kind::IntLit(first), Kind::IntLit(last)) = (ast.kind(first), ast.kind(last))
                else {
                    unreachable!("the checker accepts only constant set elements")
                };
                for element in first..=last {
                    image[element as usize / 8] |= 1 << (element % 8);
                }
            }
            image
        }
        Kind::ArrayInit(items) => {
            let (_, _, _, element) = types.array_of(ty).expect("an array initialiser");
            let image = ast.items(items).iter();
            image
                .flat_map(|&item| static_image(ast, info, element, item))
                .collect()
        }
        Kind::RecordInit(entries) => {
            let mut image = vec![0; types.size(ty) as usize];
            let fields = types.record_fields(ty);
            for (&entry, field) in ast.items(entries).iter().zip(&fields) {
                let Kind::FieldInit(_, value) = ast.kind(entry) else {
                    unreachable!("a record initialiser holds field entries")
                };
                let bytes = static_image(ast, info, field.ty, value);
                let start = field.offset as usize;
                image[start..start + bytes.len()].copy_from_slice(&bytes);
            }
            image
        }
        _ => unreachable!("the checker accepts only constant initialisers"),
    }
}

/// One WASM import from module `host` for each `external` routine the program reaches: a Real
/// parameter is an f64, every other parameter and the result of a function an i32.
fn emit_imports(m: &mut Module, ast: &Ast, info: &Info, reached: &Reached) {
    for &r in &reached.imports {
        let routine = &info.routines[r as usize];
        let import_name = routine.import.expect("an external routine");
        let params: Vec<u8> = routine
            .params
            .iter()
            .map(|p| if p.ty == Ty::REAL { F64 } else { I32 })
            .collect();
        let results: &[u8] = if routine.is_function { &[I32] } else { &[] };
        let ty = m.ty(&params, results);
        let name = ast.names[import_name as usize].to_ascii_lowercase();
        m.import_func(
            "host",
            std::str::from_utf8(&name).expect("an ASCII name"),
            ty,
        );
    }
}

/// The functions every module defines, in function-number order: `write_string`,
/// `write_integer`, `quotient`, and `real_quotient` if the program divides Reals.
fn emit_builtin_functions(m: &mut Module, layout: &Layout) {
    let write = m.ty(&[I32, I32, I32], &[]);
    m.func(write, &[], write_string(layout.indexes.host_write));
    let two_params = m.ty(&[I32, I32], &[]);
    let write_integer_code = write_integer(layout.indexes.write_string);
    m.func(two_params, &[I32, I32, I32], write_integer_code);
    let three_params = m.ty(&[I32, I32, I32], &[I32]);
    m.func(three_params, &[], quotient(layout.indexes.host_rterror));
    if layout.uses_real_div {
        let real_div = m.ty(&[F64, F64], &[F64]);
        m.func(real_div, &[], real_quotient(layout.indexes.host_rterror));
    }
}

/// Table entry 0 is the trap stub; entries 1.. are the routines whose address is taken.
fn emit_table_and_exports(
    m: &mut Module,
    info: &Info,
    layout: &Layout,
    trap_stub: u32,
    main_function: u32,
) {
    let mut table = vec![trap_stub];
    for (i, r) in info.routines.iter().enumerate() {
        if r.addressed {
            let position = layout.position[i].expect("an address-taken routine is reached");
            table.push(layout.indexes.first_routine + position);
        }
    }
    m.table(table.len() as u32);
    m.elem(0, &table);
    m.export("main", FUNC, main_function);
    m.export("sp", GLOBAL, SP);
    m.export("memory", MEM, 0);
}

fn align_up(value: u32, alignment: u32) -> u32 {
    value.div_ceil(alignment) * alignment
}

/// A routine body wrapped in `block $exit`, between the frame prologue and epilogue, followed
/// by the result read for a function; the main program (`None`) has a frame only for string
/// temporaries. The statements are generated first because the frame also holds the
/// temporaries they used, so its size is known only afterwards.
fn body(ast: &Ast, info: &Info, layout: &mut Layout, routine: Option<&Routine>, stmt: Id) -> Body {
    let hidden = routine.map_or(0, Routine::hidden_params);
    let nparams = routine.map_or(0, |r| r.params.len() as u32) + hidden;
    let mut g = Gen {
        ast,
        info,
        code: Code::new(),
        layout,
        locals: nparams + routine.map_or(0, |r| r.nlocals),
        fp: None,
        declared_frame_size: routine.map_or(0, |r| r.frame),
        temp_in_use: 0,
        temp_peak: 0,
        depth: routine.map_or(0, |r| r.depth),
        exit: Label(0),
        withs: Vec::new(),
    };
    let framed = routine.filter(|r| r.framed);
    if framed.is_some() {
        g.frame_pointer();
    }
    g.exit = g.code.block(VOID);
    g.stmt(stmt);
    g.code.end();
    let statements = std::mem::take(&mut g.code);
    match g.fp {
        Some(fp) => g.wrap_in_frame(fp, framed, statements),
        None => g.code = statements,
    }
    if let Some(r) = routine.filter(|r| r.is_function && !r.result_ptr) {
        g.load(info.syms[r.result_sym.0 as usize], r.result);
    }
    Body {
        code: g.code,
        locals: (g.locals - nparams) as usize,
    }
}

/// `write_string(pointer, length, width)`: writes the text right-justified in `width`:
/// `while width > length do (write a space; width := width - 1); write(pointer, length)`.
fn write_string(host_write: u32) -> Code {
    let (pointer, length, width) = (0, 1, 2);
    let mut c = Code::new();
    let pad = c.loop_(VOID);
    c.local_get(width).local_get(length).op(I32_GT_S).if_(VOID);
    c.i32_const(STDOUT)
        .i32_const(SPACE)
        .i32_const(1)
        .call(host_write);
    c.local_get(width)
        .i32_const(1)
        .op(I32_SUB)
        .local_set(width)
        .br(pad);
    c.end().end();
    c.i32_const(STDOUT)
        .local_get(pointer)
        .local_get(length)
        .call(host_write);
    c
}

/// `write_integer(value, width)`: the decimal digits are built backwards at the end of `BUF`
/// from the absolute value, a minus sign is put in front, and `write_string` prints them:
/// `u := abs(value); p := end; repeat p := p - 1; mem[p] := '0' + u mod 10; u := u div 10
/// until u = 0; if value < 0 then (p := p - 1; mem[p] := '-')`. Locals after the two
/// parameters: the write position, whether the value is negative, and the unsigned value.
fn write_integer(write_string_index: u32) -> Code {
    let (value, width) = (0, 1);
    let (position, negative, unsigned) = (2, 3, 4);
    let mut c = Code::new();
    c.local_get(value)
        .i32_const(0)
        .op(I32_LT_S)
        .local_set(negative);
    c.local_get(value).local_set(unsigned);
    c.local_get(negative).if_(VOID);
    c.i32_const(0)
        .local_get(value)
        .op(I32_SUB)
        .local_set(unsigned)
        .end();
    c.i32_const(BUF + BUF_LEN).local_set(position);
    let digits = c.loop_(VOID);
    c.local_get(position)
        .i32_const(1)
        .op(I32_SUB)
        .local_set(position);
    c.local_get(position)
        .local_get(unsigned)
        .i32_const(10)
        .op(I32_REM_U);
    c.i32_const(b'0' as i32).op(I32_ADD).mem(I32_STORE8, 0, 0);
    c.local_get(unsigned)
        .i32_const(10)
        .op(I32_DIV_U)
        .local_set(unsigned);
    c.local_get(unsigned).br_if(digits).end();
    c.local_get(negative).if_(VOID);
    c.local_get(position)
        .i32_const(1)
        .op(I32_SUB)
        .local_set(position);
    c.local_get(position)
        .i32_const(b'-' as i32)
        .mem(I32_STORE8, 0, 0)
        .end();
    // write_string(position, BUF_END - position, width)
    c.local_get(position)
        .i32_const(BUF + BUF_LEN)
        .local_get(position)
        .op(I32_SUB);
    c.local_get(width).call(write_string_index);
    c
}

/// `quot(a, b, flags)`: `a div b`, or `a mod b` when `QUOT_REMAINDER` is set in `flags`;
/// `QUOT_16_BIT` marks 16-bit operands. A zero divisor is runtime error 200, as is -32768 by
/// -1 on 16 bits (the quotient does not fit); on 32 bits `a div -1` wraps and `a mod -1` is 0.
/// Both are special-cased because WASM traps on them. In Pascal:
///   if b = 0 then RunError(200);
///   if b = -1 then begin
///     if (flags is 16-bit) and (a = -32768) then RunError(200);
///     if remainder then return 0 else return 0 - a
///   end;
///   if remainder then a rem b else a div b
fn quotient(host_rterror: u32) -> Code {
    let (a, b, flags) = (0, 1, 2);
    let mut c = Code::new();
    c.local_get(b).op(I32_EQZ).if_(VOID);
    c.i32_const(ERROR_DIVISION_BY_ZERO).call(host_rterror).end();
    c.local_get(b).i32_const(-1).op(I32_EQ).if_(VOID);
    // `flags > 1` is true exactly when the 16-bit bit is set (flags is 0..=3).
    c.local_get(flags).i32_const(1).op(I32_GT_S);
    c.local_get(a)
        .i32_const(-32768)
        .op(I32_EQ)
        .op(I32_AND)
        .if_(VOID);
    c.i32_const(ERROR_DIVISION_BY_ZERO).call(host_rterror).end();
    c.local_get(flags)
        .i32_const(QUOT_REMAINDER)
        .op(I32_AND)
        .if_(I32);
    c.i32_const(0)
        .else_()
        .i32_const(0)
        .local_get(a)
        .op(I32_SUB)
        .end();
    c.op(RETURN).end();
    // `select(remainder, quotient, flag)` picks the remainder when the flag is set.
    c.local_get(a).local_get(b).op(I32_REM_S);
    c.local_get(a).local_get(b).op(I32_DIV_S);
    c.local_get(flags)
        .i32_const(QUOT_REMAINDER)
        .op(I32_AND)
        .op(SELECT);
    c
}

/// `real_quotient(a, b)`: `a / b`, but a zero divisor is runtime error 200.
fn real_quotient(host_rterror: u32) -> Code {
    let mut c = Code::new();
    c.local_get(1).f64_const(0.0).op(F64_EQ).if_(VOID);
    c.i32_const(ERROR_DIVISION_BY_ZERO).call(host_rterror).end();
    c.local_get(0).local_get(1).op(F64_DIV);
    c
}

fn real_op(op: Op) -> u8 {
    match op {
        Op::Add => F64_ADD,
        Op::Sub => F64_SUB,
        Op::Mul => F64_MUL,
        Op::Eq => F64_EQ,
        Op::Ne => F64_NE,
        Op::Lt => F64_LT,
        Op::Gt => F64_GT,
        Op::Le => F64_LE,
        Op::Ge => F64_GE,
        _ => unreachable!("Real operators"),
    }
}

fn binary_op(op: Op) -> u8 {
    match op {
        Op::Add => I32_ADD,
        Op::Sub => I32_SUB,
        Op::Mul => I32_MUL,
        Op::Div => I32_DIV_S,
        Op::Mod => I32_REM_S,
        Op::And => I32_AND,
        Op::Or => I32_OR,
        Op::Xor => I32_XOR,
        Op::Shl => I32_SHL,
        Op::Shr => I32_SHR_U,
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
    // ---- Frames, temporaries and helpers ----

    fn new_local(&mut self) -> u32 {
        self.locals += 1;
        self.locals - 1
    }

    /// The frame pointer local, created on first use.
    fn frame_pointer(&mut self) -> u32 {
        if let Some(fp) = self.fp {
            return fp;
        }
        let fp = self.new_local();
        self.fp = Some(fp);
        fp
    }

    /// The routine's code between a frame prologue and epilogue: allocate the frame on the
    /// shadow stack (runtime error 202 on overflow), store the static link, copy the
    /// parameters that live in the frame, run `statements`, release the frame. `routine` is
    /// the routine with a frame of its own, if any.
    fn wrap_in_frame(&mut self, fp: u32, routine: Option<&Routine>, statements: Code) {
        let frame_size = (self.declared_frame_size + self.temp_peak) as i32;
        self.code
            .idx(GLOBAL_GET, SP)
            .i32_const(frame_size)
            .op(I32_SUB);
        self.code.local_tee(fp).i32_const(STACK_LIMIT).op(I32_LT_U);
        self.code.if_(VOID);
        self.code
            .i32_const(ERROR_STACK_OVERFLOW)
            .call(self.layout.indexes.host_rterror)
            .end();
        self.code.local_get(fp).idx(GLOBAL_SET, SP);
        if routine.is_some() && self.depth > 1 {
            // The static link, the parent's frame pointer, is parameter 0.
            self.code.local_get(fp).local_get(0).mem(I32_STORE, 0, 0);
        }
        for copy in routine.map_or(&[][..], |r| &r.copies[..]) {
            self.copy_into_frame(fp, copy);
        }
        self.code.append(statements);
        self.code
            .local_get(fp)
            .i32_const(frame_size)
            .op(I32_ADD)
            .idx(GLOBAL_SET, SP);
    }

    /// Copies a value parameter from its WASM local into its slot in the frame.
    fn copy_into_frame(&mut self, fp: u32, copy: &FrameCopy) {
        let FrameCopy { local, offset, ty } = *copy;
        let types = &self.info.types;
        self.code.local_get(fp);
        if types.is_str(ty) {
            let assign = self.helper_index(Helper::StrAssign);
            let capacity = types.capacity(ty) as i32;
            self.code
                .i32_const(offset as i32)
                .op(I32_ADD)
                .local_get(local);
            self.code.i32_const(capacity).call(assign);
        } else if types.in_memory(ty) {
            let size = types.size(ty) as i32;
            self.code
                .i32_const(offset as i32)
                .op(I32_ADD)
                .local_get(local);
            self.code.i32_const(size).memory_copy();
        } else {
            self.code.local_get(local);
            self.store(ty, offset);
        }
    }

    /// A new temporary of `size` bytes, returned as its frame offset. A statement's
    /// temporaries are not reused within it.
    fn new_temp(&mut self, size: u32) -> u32 {
        self.temp_in_use += size;
        self.temp_peak = self.temp_peak.max(self.temp_in_use);
        self.declared_frame_size + self.temp_in_use - size
    }

    /// Pushes the address of the temporary at frame offset `offset`.
    fn temp_addr(&mut self, offset: u32) {
        let fp = self.frame_pointer();
        self.code.local_get(fp).i32_const(offset as i32).op(I32_ADD);
    }

    fn helper_index(&mut self, helper: Helper) -> u32 {
        self.layout.routine_index(self.info.helper(helper))
    }

    /// Calls the System helper.
    fn helper(&mut self, helper: Helper) {
        let index = self.helper_index(helper);
        self.code.call(index);
    }

    // ---- Integer representation ----

    /// Re-normalises the i32 on the stack to the integer type `ty` (plan §4.6): every integer
    /// is kept in an i32 in its type's range, so after arithmetic that may leave the range the
    /// value is wrapped back as TP would store it.
    fn narrow(&mut self, ty: Ty) {
        if !matches!(self.info.types.tag(ty), Some(Tag::Int | Tag::Char)) {
            return;
        }
        let info = self.info.types.ord(ty);
        match (info.size, info.signed) {
            (4, _) => {}
            (_, false) => {
                // Unsigned: keep the low 8 or 16 bits.
                self.code.i32_const((1 << (8 * info.size)) - 1).op(I32_AND);
            }
            _ => {
                // Signed: shift the sign bit up to bit 31 and back, replicating it.
                let shift = 32 - 8 * info.size as i32;
                self.code
                    .i32_const(shift)
                    .op(I32_SHL)
                    .i32_const(shift)
                    .op(I32_SHR_S);
            }
        }
    }

    /// Under `{$R+}` at the node `e`, checks the i32 that `e` pushed against `lo..hi`: runtime
    /// error 201 outside. Skipped for constants and when the type of `e` already fits.
    fn check(&mut self, e: Id, lo: i64, hi: i64) {
        let node = self.ast.node(e);
        if !node.range_checks || matches!(node.kind, Kind::IntLit(_)) {
            return;
        }
        if self.info.types.tag(node.ty).is_some() {
            let own = self.info.types.ord(node.ty);
            if own.lo >= lo && own.hi <= hi {
                return;
            }
        } else {
            return;
        }
        self.code.i32_const(lo as i32).i32_const(hi as i32);
        self.helper(Helper::RangeChk);
    }

    /// `check` against the bounds of the type `ty`, when it is an ordinal type.
    fn check_ty(&mut self, e: Id, ty: Ty) {
        if self.info.types.tag(ty).is_some() {
            let range = self.info.types.ord(ty);
            self.check(e, range.lo, range.hi);
        }
    }

    // ---- Variables and addresses ----

    /// Pushes the frame pointer of the routine `up` levels above the current one: the own
    /// `fp`, else the static link (local 0) followed `up - 1` times.
    fn frame_ptr(&mut self, up: u32) {
        if up == 0 {
            let fp = self.frame_pointer();
            self.code.local_get(fp);
            return;
        }
        self.code.local_get(0);
        for _ in 1..up {
            self.code.mem(I32_LOAD, 0, 0);
        }
    }

    /// A body names the static variable at `address`; if it is a typed constant its initial
    /// image must be emitted.
    fn mark_typed_const(&mut self, address: u32) {
        let constants = &self.info.typed_consts;
        if let Ok(i) = constants.binary_search_by_key(&address, |c| c.address) {
            self.layout.typed_const_used[i] = true;
        }
    }

    /// Pushes the address base of a variable in memory and returns the constant offset to add
    /// to it.
    fn base(&mut self, s: Symbol) -> u32 {
        match s.kind {
            SymKind::Var(address) => {
                self.mark_typed_const(address);
                self.code.i32_const(address as i32);
                0
            }
            SymKind::Frame(offset) => {
                self.frame_ptr(self.depth - s.depth);
                offset
            }
            SymKind::FrameRef(offset) => {
                self.frame_ptr(self.depth - s.depth);
                self.code.mem(I32_LOAD, 0, offset);
                0
            }
            SymKind::Local(local) | SymKind::Ref(local) => {
                self.code.local_get(local);
                0
            }
            _ => unreachable!("a variable"),
        }
    }

    /// Stores the i32 on the stack to the address below it, truncated to `ty`.
    fn store(&mut self, ty: Ty, offset: u32) {
        let opcode = match self.info.types.ord(ty).size {
            1 => I32_STORE8,
            2 => I32_STORE16,
            _ => I32_STORE,
        };
        self.code.mem(opcode, 0, offset);
    }

    /// The symbol of the checked variable node `v`.
    fn symbol(&self, v: Id) -> Symbol {
        let Kind::Var(sym) = self.ast.kind(v) else {
            unreachable!("checked variable")
        };
        self.info.syms[sym.0 as usize]
    }

    /// The WASM local that holds variable `v`, if it is a plain local.
    fn plain_local(&self, v: Id) -> Option<u32> {
        if !matches!(self.ast.kind(v), Kind::Var(_)) {
            return None;
        }
        match self.symbol(v).kind {
            SymKind::Local(local) => Some(local),
            _ => None,
        }
    }

    fn load_op(&self, ty: Ty) -> u8 {
        let info = self.info.types.ord(ty);
        match (info.size, info.signed) {
            (1, true) => I32_LOAD8_S,
            (1, false) => I32_LOAD8_U,
            (2, true) => I32_LOAD16_S,
            (2, false) => I32_LOAD16_U,
            _ => I32_LOAD,
        }
    }

    /// Pushes the value of a variable of type `ty`.
    fn load(&mut self, s: Symbol, ty: Ty) {
        if let SymKind::Local(local) = s.kind {
            self.code.local_get(local);
            return;
        }
        let offset = self.base(s);
        self.code.mem(self.load_op(ty), 0, offset);
    }

    /// Pushes the address of the designator `e` and returns the constant offset to add. A
    /// `with` record is read from its local, so its designator is evaluated once.
    fn addr(&mut self, e: Id) -> u32 {
        if let Some(&(_, local)) = self.withs.iter().find(|w| w.0 == e) {
            self.code.local_get(local);
            return 0;
        }
        match self.ast.kind(e) {
            Kind::Var(_) => {
                let s = self.symbol(e);
                self.base(s)
            }
            Kind::Deref(pointer) => {
                self.expr(pointer);
                0
            }
            Kind::Field(base, offset) => self.addr(base) + offset,
            Kind::Index { base, index, .. } => self.element_addr(base, index),
            _ => unreachable!("designators"),
        }
    }

    /// The address of `base[index]`: the base address plus `(index - lo) * element size`. A
    /// constant index folds into the returned offset.
    fn element_addr(&mut self, base: Id, index: Id) -> u32 {
        let offset = self.addr(base);
        let types = &self.info.types;
        let (_, lo, hi, element) = types.array_of(self.ast.ty(base)).expect("checked array");
        let size = types.size(element);
        if let Kind::IntLit(c) = self.ast.kind(index) {
            return offset + (c - lo) as u32 * size;
        }
        self.expr(index);
        self.check(index, lo, hi);
        if lo != 0 {
            self.code.i32_const(lo as i32).op(I32_SUB);
        }
        if size != 1 {
            self.code.i32_const(size as i32).op(I32_MUL);
        }
        self.code.op(I32_ADD);
        offset
    }

    /// Pushes the value of the variable or designator `v`.
    fn get(&mut self, v: Id) {
        let ty = self.ast.ty(v);
        if matches!(self.ast.kind(v), Kind::Var(_)) {
            self.load(self.symbol(v), ty);
        } else {
            let offset = self.addr(v);
            self.code.mem(self.load_op(ty), 0, offset);
        }
    }

    /// Assigns the value that `value` pushes to the variable `v`, truncated to its type: a
    /// store truncates in memory, a local is narrowed.
    fn put(&mut self, v: Id, value: impl FnOnce(&mut Self)) {
        let ty = self.ast.ty(v);
        match self.plain_local(v) {
            Some(local) => {
                value(self);
                self.narrow(ty);
                self.code.local_set(local);
            }
            None => {
                let offset = self.addr(v);
                value(self);
                self.store(ty, offset);
            }
        }
    }

    // ---- Strings, sets and Reals in memory ----

    /// Pushes the address of the string value of `e`, which the caller must not modify:
    /// literals and variables in place, everything else in a temporary (decision 13).
    fn str(&mut self, e: Id) {
        match self.ast.kind(e) {
            Kind::StrLit(s) => {
                let addr = self.layout.literal(s, &self.ast.strs[s as usize]);
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
            Kind::Binary {
                op: Op::Add,
                left,
                right,
            } => {
                let t = self.new_temp(STRING_TEMP);
                self.temp_addr(t);
                self.str(left);
                self.str(right);
                self.helper(Helper::StrCat);
                self.temp_addr(t);
            }
            Kind::Invoke(r, args) => {
                let t = self.invoke(r, args).expect("string function");
                self.temp_addr(t);
            }
            Kind::Fmt {
                value,
                width,
                decimals,
            } => self.formatted_real_as_string(value, width, decimals),
            _ => self.address_of(e),
        }
    }

    /// `x : w : d` used as a String: `fmt_real` writes the digits into a temporary after its
    /// length byte, then the length (clamped to 255) is stored in that byte.
    fn formatted_real_as_string(&mut self, value: Id, width: Id, decimals: Option<Id>) {
        let (t, length) = (self.new_temp(REAL_BUF), self.new_local());
        // The address and the clamp limit wait on the stack for the length `fmt_real` returns.
        self.temp_addr(t);
        self.code.i32_const(MAX_STRING);
        self.real(value);
        self.expr(width);
        self.decimals_or_absent(decimals);
        self.temp_addr(t);
        let fmt_real = self.helper_index(Helper::FmtReal);
        self.code.i32_const(1).op(I32_ADD).call(fmt_real);
        // select(255, length, length > 255) is min(length, 255).
        self.code.local_tee(length).local_get(length);
        self.code.i32_const(MAX_STRING).op(I32_GT_S).op(SELECT);
        self.code.mem(I32_STORE8, 0, 0);
        self.temp_addr(t);
    }

    /// Pushes the decimals argument of `fmt_real`, -1 when there is none.
    fn decimals_or_absent(&mut self, decimals: Option<Id>) {
        match decimals {
            Some(decimals) => self.expr(decimals),
            None => {
                self.code.i32_const(-1);
            }
        }
    }

    /// Pushes the address of the designator `e`, or of the temporary holding the set value `e`
    /// (a constructor, or the result of `+ * -`).
    fn address_of(&mut self, e: Id) {
        match self.ast.kind(e) {
            Kind::SetLit(items) => self.set_constructor(items),
            Kind::Binary { op, left, right } => self.set_operation(op, left, right),
            _ => {
                let offset = self.addr(e);
                if offset > 0 {
                    self.code.i32_const(offset as i32).op(I32_ADD);
                }
            }
        }
    }

    /// Builds `[a, b..c]` in a temporary with `SetAdd` for each element or range; pushes its
    /// address.
    fn set_constructor(&mut self, items: List) {
        let t = self.new_temp(SET_TEMP);
        // SetOp(t, t, t, difference): t - t clears the temporary to the empty set.
        for _ in 0..3 {
            self.temp_addr(t);
        }
        self.code.i32_const(SET_DIFFERENCE);
        self.helper(Helper::SetOp);
        let single = self.new_local();
        for &item in self.ast.items(items) {
            self.temp_addr(t);
            let (lo, hi) = match self.ast.kind(item) {
                Kind::Range(lo, hi) => (lo, Some(hi)),
                _ => (item, None),
            };
            self.expr(lo);
            match hi {
                Some(hi) => self.expr(hi),
                None => {
                    // A single element is the range `lo..lo`.
                    self.code.local_tee(single).local_get(single);
                }
            }
            self.helper(Helper::SetAdd);
        }
        self.temp_addr(t);
    }

    /// `a + b`, `a * b` or `a - b` on sets into a temporary; pushes its address.
    fn set_operation(&mut self, op: Op, left: Id, right: Id) {
        let t = self.new_temp(SET_TEMP);
        self.temp_addr(t);
        self.address_of(left);
        self.address_of(right);
        self.code.i32_const(match op {
            Op::Add => SET_UNION,
            Op::Mul => SET_INTERSECTION,
            _ => SET_DIFFERENCE,
        });
        self.helper(Helper::SetOp);
        self.temp_addr(t);
    }

    // ---- Calls ----

    /// A call of routine `r`: value arguments converted to the parameter types, `var`
    /// arguments passed as addresses. The static link comes first, then, for a function
    /// whose result lives in memory, the address of a new temporary for the result, which is
    /// returned.
    fn invoke(&mut self, r: u32, args: List) -> Option<u32> {
        let info = self.info;
        let routine = &info.routines[r as usize];
        if routine.has_static_link() {
            self.frame_ptr(self.depth + 1 - routine.depth);
        }
        let result = routine
            .result_ptr
            .then(|| self.new_temp(info.types.size(routine.result)));
        if let Some(t) = result {
            self.temp_addr(t);
        }
        self.push_args(args, &routine.params, routine.import.is_some());
        let index = self.layout.routine_index(r);
        self.code.call(index);
        result
    }

    /// Value arguments converted to the parameter types, `var` arguments as addresses. A Real
    /// goes by address, except to an import, which receives the f64 itself.
    fn push_args(&mut self, args: List, params: &[ParamType], to_import: bool) {
        for (&arg, param) in self.ast.items(args).iter().zip(params) {
            let ty = param.ty;
            if param.by_ref || self.info.types.is_agg(ty) {
                self.address_of(arg);
            } else if ty == Ty::REAL && to_import {
                self.real(arg);
            } else if ty == Ty::REAL {
                self.real_addr(arg);
            } else if self.info.types.is_str(ty) {
                self.str(arg);
            } else {
                self.expr(arg);
                self.check_ty(arg, ty);
                self.narrow(ty);
            }
        }
    }

    /// A call through the procedural value `callee`: arguments, the table index, then
    /// `call_indirect` with the WASM type of the signature.
    fn indirect(&mut self, callee: Id, args: List) {
        let ty = self.ast.ty(callee);
        let (params, _) = self.info.types.sig(ty).expect("procedural callee");
        self.push_args(args, &params, false);
        self.expr(callee);
        self.code
            .call_indirect(self.layout.wasm_types[ty.0 as usize]);
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
            Op::Abs => self.helper(Helper::AbsLong),
            Op::Sqr => {
                let t = self.new_local();
                self.code.local_set(t);
                self.code.local_get(t).local_get(t).op(I32_MUL);
            }
            _ => {}
        }
        self.narrow(ty);
    }

    // ---- Statements ----

    fn stmt(&mut self, id: Id) {
        // Each statement starts with no temporaries in use.
        self.temp_in_use = 0;
        match self.ast.kind(id) {
            Kind::Compound(statements) => self.statements(statements),
            Kind::Assign(target, value) => self.assign(target, value),
            Kind::With(records, body) => self.with_statement(records, body),
            Kind::Invoke(r, args) => {
                self.invoke(r, args);
            }
            Kind::Indirect(callee, args) => self.indirect(callee, args),
            Kind::RunErr(code) => {
                self.expr(code);
                self.code.call(self.layout.indexes.host_rterror);
            }
            Kind::InitPart(body) => {
                // `Exit` in a unit's initialisation leaves that part only.
                let enclosing = self.exit;
                self.exit = self.code.block(VOID);
                self.stmt(body);
                self.code.end();
                self.exit = enclosing;
            }
            Kind::Exit => {
                self.code.br(self.exit);
            }
            Kind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expr(condition);
                self.code.if_(VOID);
                self.stmt(then_branch);
                if let Some(else_branch) = else_branch {
                    self.code.else_();
                    self.stmt(else_branch);
                }
                self.code.end();
            }
            Kind::While(condition, body) => self.while_statement(condition, body),
            Kind::Repeat(body, condition) => {
                let top = self.code.loop_(VOID);
                self.statements(body);
                self.expr(condition);
                self.code.op(I32_EQZ).br_if(top).end();
            }
            Kind::Case {
                selector,
                arms,
                else_body,
            } => self.case_statement(selector, arms, else_body),
            Kind::For {
                variable,
                first,
                last,
                body,
                downto,
            } => self.for_statement(variable, first, last, body, downto),
            Kind::Write(args, newline) => {
                for &arg in self.ast.items(args) {
                    self.write_arg(arg);
                }
                if newline {
                    self.code.i32_const(STDOUT).i32_const(LF).i32_const(1);
                    self.code.call(self.layout.indexes.host_write);
                }
            }
            _ => unreachable!("checked statements"),
        }
    }

    fn statements(&mut self, list: List) {
        for &statement in self.ast.items(list) {
            self.stmt(statement);
        }
    }

    /// An assignment, by the kind of value: a String is copied with truncation to the
    /// target's capacity, an aggregate with `memory.copy`, a Real as an f64, and anything
    /// else as an i32 stored at the target's size.
    fn assign(&mut self, target: Id, value: Id) {
        let ty = self.ast.ty(target);
        let types = &self.info.types;
        if types.is_str(ty) {
            let capacity = types.capacity(ty);
            self.address_of(target);
            self.str(value);
            let assign = self.helper_index(Helper::StrAssign);
            self.code.i32_const(capacity as i32).call(assign);
        } else if types.is_agg(ty) {
            let size = types.size(ty);
            self.address_of(target);
            self.address_of(value);
            self.code.i32_const(size as i32).memory_copy();
        } else if ty == Ty::REAL {
            let offset = self.addr(target);
            self.real(value);
            self.code.mem(F64_STORE, 0, offset);
        } else {
            self.put(target, |g| {
                g.expr(value);
                g.check_ty(value, ty);
            });
        }
    }

    /// `with r do s`: the address of each record is computed once into a local; field names
    /// the checker resolved through the record read it from there.
    fn with_statement(&mut self, records: List, body: Id) {
        let outer = self.withs.len();
        for &record in self.ast.items(records) {
            let local = self.new_local();
            self.address_of(record);
            self.code.local_set(local);
            self.withs.push((record, local));
        }
        self.stmt(body);
        self.withs.truncate(outer);
    }

    /// `block $exit (loop $top (br_if $exit (not cond)); body; br $top)`.
    fn while_statement(&mut self, condition: Id, body: Id) {
        let exit = self.code.block(VOID);
        let top = self.code.loop_(VOID);
        self.expr(condition);
        self.code.op(I32_EQZ).br_if(exit);
        self.stmt(body);
        self.code.br(top).end().end();
    }

    /// The selector is converted to Integer; each arm is a chain of range tests, and a
    /// matching arm branches to the end of the statement, skipping the `else` part.
    fn case_statement(&mut self, selector: Id, arms: List, else_body: List) {
        let local = self.new_local();
        self.expr(selector);
        self.narrow(Ty::INTEGER);
        self.code.local_set(local);
        let exit = self.code.block(VOID);
        for &arm in self.ast.items(arms) {
            let Kind::Arm(labels, body) = self.ast.kind(arm) else {
                unreachable!("checked arm")
            };
            // The arm's condition, an OR of its labels' tests, starting from false.
            self.code.i32_const(0);
            for &label in self.ast.items(labels) {
                self.case_label_test(local, label);
            }
            self.code.if_(VOID);
            self.stmt(body);
            self.code.br(exit).end();
        }
        self.statements(else_body);
        self.code.end();
    }

    /// ORs into the condition on the stack whether the selector in `local` is in the label's
    /// range. `lo <= x <= hi` is one unsigned comparison, `x - lo <= hi - lo`, since a
    /// negative `x - lo` wraps to a huge unsigned number.
    fn case_label_test(&mut self, local: u32, label: Id) {
        let Kind::Range(lo, hi) = self.ast.kind(label) else {
            unreachable!("checked range")
        };
        let (Kind::IntLit(lo), Kind::IntLit(hi)) = (self.ast.kind(lo), self.ast.kind(hi)) else {
            unreachable!("constant labels")
        };
        if lo > hi {
            return;
        }
        self.code.local_get(local).i32_const(lo as i32).op(I32_SUB);
        self.code
            .i32_const((hi - lo) as i32)
            .op(I32_LE_U)
            .op(I32_OR);
    }

    /// The last value is compared for equality before stepping, so the control variable
    /// never passes its type's limit.
    fn for_statement(&mut self, variable: Id, first: Id, last: Id, body: Id, downto: bool) {
        let end = self.new_local();
        self.put(variable, |g| g.expr(first));
        self.expr(last);
        self.code.local_set(end);
        let exit = self.code.block(VOID);
        // Skip the loop if the first value is already past the last.
        self.get(variable);
        self.code.local_get(end);
        let past_end = if downto { I32_LT_S } else { I32_GT_S };
        self.code.op(past_end).br_if(exit);
        let top = self.code.loop_(VOID);
        self.stmt(body);
        self.get(variable);
        self.code.local_get(end).op(I32_EQ).br_if(exit);
        let step = if downto { -1 } else { 1 };
        self.put(variable, |g| {
            g.get(variable);
            g.code.i32_const(step).op(I32_ADD);
        });
        self.code.br(top).end().end();
    }

    // ---- Write ----

    /// Pushes the width of a write argument, 0 when there is none.
    fn width(&mut self, width: Option<Id>) {
        match width {
            Some(width) => self.expr(width),
            None => {
                self.code.i32_const(0);
            }
        }
    }

    /// One argument of `Write`: by the value's type, `value` or `value : width [: decimals]`.
    fn write_arg(&mut self, arg: Id) {
        let (value, width, decimals) = match self.ast.kind(arg) {
            Kind::Fmt {
                value,
                width,
                decimals,
            } => (value, Some(width), decimals),
            _ => (arg, None, None),
        };
        let node = self.ast.node(value);
        match (self.info.types.tag(node.ty), node.kind) {
            (None, _) if node.ty == Ty::REAL => self.write_real(value, width, decimals),
            (None, Kind::StrLit(s)) => self.write_literal(s, width),
            (None, _) if self.info.types.is_str(node.ty) => self.write_string_value(value, width),
            (None, _) if self.info.types.is_char_array(node.ty) => {
                self.write_char_array(value, node.ty, width)
            }
            (Some(Tag::Int), _) => {
                self.expr(value);
                self.width(width);
                self.code.call(self.layout.indexes.write_integer);
            }
            (Some(Tag::Char), _) => self.write_char(value, width),
            _ => self.write_boolean(value, width),
        }
    }

    /// `fmt_real` formats into a temporary (-1 for an absent width or decimals); `write`
    /// then prints what it produced.
    fn write_real(&mut self, value: Id, width: Option<Id>, decimals: Option<Id>) {
        let t = self.new_temp(REAL_BUF);
        self.code.i32_const(STDOUT);
        self.temp_addr(t);
        self.real(value);
        match width {
            Some(width) => self.expr(width),
            None => {
                self.code.i32_const(-1);
            }
        }
        self.decimals_or_absent(decimals);
        self.temp_addr(t);
        let fmt_real = self.helper_index(Helper::FmtReal);
        self.code
            .call(fmt_real)
            .call(self.layout.indexes.host_write);
    }

    /// A string literal is written from its text in static data, skipping the length byte.
    fn write_literal(&mut self, s: u32, width: Option<Id>) {
        let bytes = &self.ast.strs[s as usize];
        let addr = self.layout.literal(s, bytes);
        self.code
            .i32_const(addr as i32 + 1)
            .i32_const(bytes.len() as i32);
        self.width(width);
        self.code.call(self.layout.indexes.write_string);
    }

    /// Any other String is written from its address: text after the length byte.
    fn write_string_value(&mut self, value: Id, width: Option<Id>) {
        let address = self.new_local();
        self.str(value);
        self.code.local_set(address);
        self.code.local_get(address).i32_const(1).op(I32_ADD);
        self.code.local_get(address).mem(I32_LOAD8_U, 0, 0);
        self.width(width);
        self.code.call(self.layout.indexes.write_string);
    }

    /// An array of Char is written from its address: every element.
    fn write_char_array(&mut self, value: Id, ty: Ty, width: Option<Id>) {
        let (_, low, high, _) = self.info.types.array_of(ty).expect("an array");
        self.address_of(value);
        self.code.i32_const((high - low + 1) as i32);
        self.width(width);
        self.code.call(self.layout.indexes.write_string);
    }

    /// A Char is stored in the scratch buffer and written as a one-character string.
    fn write_char(&mut self, value: Id, width: Option<Id>) {
        self.code.i32_const(BUF);
        self.expr(value);
        self.code.mem(I32_STORE8, 0, 0).i32_const(BUF).i32_const(1);
        self.width(width);
        self.code.call(self.layout.indexes.write_string);
    }

    /// A Boolean is written as `TRUE` or `FALSE`: `select(a, b, c)` yields `a` when `c` is
    /// non-zero, so one select picks the text's address and another its length.
    fn write_boolean(&mut self, value: Id, width: Option<Id>) {
        let flag = self.new_local();
        self.expr(value);
        self.code.local_set(flag);
        self.code
            .i32_const(TRUE)
            .i32_const(FALSE)
            .local_get(flag)
            .op(SELECT);
        self.code
            .i32_const(4)
            .i32_const(5)
            .local_get(flag)
            .op(SELECT);
        self.width(width);
        self.code.call(self.layout.indexes.write_string);
    }

    // ---- Real expressions ----

    /// Evaluates the Real `e` into a new temporary and returns its frame offset.
    fn spill(&mut self, e: Id) -> u32 {
        let t = self.new_temp(8);
        self.temp_addr(t);
        self.real(e);
        self.code.mem(F64_STORE, 0, 0);
        t
    }

    /// Pushes the f64 stored in temporary `t`.
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
        match self.ast.kind(e) {
            Kind::Var(_) | Kind::Index { .. } | Kind::Field(..) | Kind::Deref(_) => {
                self.address_of(e)
            }
            _ => {
                let t = self.spill(e);
                self.temp_addr(t);
            }
        }
    }

    /// Pushes the f64 value of the Real expression `e`.
    fn real(&mut self, e: Id) {
        match self.ast.kind(e) {
            Kind::RealLit(v) => {
                self.code.f64_const(v);
            }
            Kind::IntToReal(a) => {
                self.expr(a);
                self.code.op(F64_CONVERT_I32_S);
            }
            Kind::Var(_) | Kind::Index { .. } | Kind::Field(..) | Kind::Deref(_) => {
                let offset = self.addr(e);
                self.code.mem(F64_LOAD, 0, offset);
            }
            Kind::Invoke(r, args) => {
                let t = self.invoke(r, args).expect("Real function");
                self.reload(t);
            }
            Kind::Binary {
                op: Op::Slash,
                left,
                right,
            } => {
                self.real(left);
                self.real(right);
                let real_div = self.layout.real_div_index();
                self.code.call(real_div);
            }
            Kind::Binary { op, left, right } => {
                self.real(left);
                self.real(right);
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
                    Op::Neg => F64_NEG,
                    Op::Abs => F64_ABS,
                    Op::Sqrt => F64_SQRT,
                    _ => F64_TRUNC,
                });
            }
            _ => unreachable!("checked Real expressions"),
        }
    }

    // ---- Integer and other expressions ----

    /// Pushes the value of the expression `e`: an i32 for integer-like types, an address for
    /// Strings and sets.
    fn expr(&mut self, e: Id) {
        let n = self.ast.node(e);
        match n.kind {
            Kind::IntLit(v) => {
                self.code.i32_const(v as i32);
            }
            Kind::Var(_) | Kind::Index { .. } | Kind::Field(..) | Kind::Deref(_) => self.get(e),
            Kind::AddrOf(a) => self.address_of(a),
            Kind::ProcRef(r) => {
                self.code
                    .i32_const(self.layout.table_slots[r as usize] as i32);
            }
            Kind::Invoke(r, args) => {
                self.invoke(r, args);
            }
            Kind::Indirect(callee, args) => self.indirect(callee, args),
            Kind::Binary {
                op: op @ (Op::And | Op::Or),
                left,
                right,
            } if n.ty == Ty::BOOLEAN => self.short_circuit(op, left, right),
            Kind::Unary(op @ (Op::Round | Op::Trunc), a) => self.round_or_trunc(op, a),
            Kind::Unary(op, a) => self.unary(op, a, n.ty),
            Kind::Binary { op, left, right } => self.binary(n.ty, op, left, right),
            _ => unreachable!("checked expressions"),
        }
    }

    /// `{$B-}`: the right operand of `and` / `or` is evaluated only when it can matter.
    fn short_circuit(&mut self, op: Op, left: Id, right: Id) {
        self.expr(left);
        self.code.if_(I32);
        if op == Op::And {
            self.expr(right);
            self.code.else_().i32_const(0);
        } else {
            self.code.i32_const(1).else_();
            self.expr(right);
        }
        self.code.end();
    }

    /// `Trunc(x)` and `Round(x)` of a Real into a LongInt, saturating at the ends of the
    /// range.
    fn round_or_trunc(&mut self, op: Op, a: Id) {
        if op == Op::Trunc {
            self.real(a);
        } else {
            // Half away from zero: trunc(x) + trunc(2 * frac(x)).
            let t = self.spill(a);
            self.reload(t);
            self.code.op(F64_TRUNC);
            self.frac(t);
            self.code
                .f64_const(2.0)
                .op(F64_MUL)
                .op(F64_TRUNC)
                .op(F64_ADD);
        }
        self.code.truncate_f64_saturating();
    }

    /// A binary operator by its operand type: Real, set membership, set comparison, string
    /// comparison, or integer-like.
    fn binary(&mut self, ty: Ty, op: Op, left: Id, right: Id) {
        let operand_ty = self.ast.ty(left);
        if operand_ty == Ty::REAL {
            self.real(left);
            self.real(right);
            self.code.op(real_op(op));
        } else if op == Op::In {
            self.address_of(right);
            self.expr(left);
            self.helper(Helper::SetIn);
        } else if self.info.types.is_set(operand_ty) {
            self.set_comparison(op, left, right);
        } else if self.info.types.is_str(operand_ty) {
            self.str(left);
            self.str(right);
            let compare = self.helper_index(Helper::StrCmp);
            self.code.call(compare).i32_const(0).op(binary_op(op));
        } else {
            self.integer_binary(ty, op, left, right);
        }
    }

    /// `=`, `<>`, `<=`, `>=` on sets: `SetCmp` reports which set has an element the other
    /// lacks, and a mask selects the bits that make the comparison false.
    fn set_comparison(&mut self, op: Op, left: Id, right: Id) {
        self.address_of(left);
        self.address_of(right);
        self.helper(Helper::SetCmp);
        let false_bits = match op {
            Op::Le => SET_FIRST_HAS_EXTRA,
            Op::Ge => SET_SECOND_HAS_EXTRA,
            _ => SET_FIRST_HAS_EXTRA | SET_SECOND_HAS_EXTRA,
        };
        // `<>` is true when any such bit is set; the others when none is.
        let test = binary_op(if op == Op::Ne { Op::Ne } else { Op::Eq });
        self.code
            .i32_const(false_bits)
            .op(I32_AND)
            .i32_const(0)
            .op(test);
    }

    /// Integer-like operands: `div` and `mod` go through `quot` (which checks for zero), the
    /// others are single instructions followed by narrowing to the result type.
    fn integer_binary(&mut self, ty: Ty, op: Op, left: Id, right: Id) {
        self.expr(left);
        if op == Op::Shr && ty == Ty::INTEGER {
            // A 16-bit value is sign-extended in the i32; clear the upper bits so the logical
            // shift brings in zeros as a 16-bit shift would.
            self.code.i32_const(0xFFFF).op(I32_AND);
        }
        self.expr(right);
        if matches!(op, Op::Div | Op::Mod) {
            let remainder = if op == Op::Mod { QUOT_REMAINDER } else { 0 };
            let sixteen_bit = if ty == Ty::INTEGER { QUOT_16_BIT } else { 0 };
            self.code
                .i32_const(remainder + sixteen_bit)
                .call(self.layout.indexes.quotient);
            return;
        }
        self.code.op(binary_op(op));
        if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Shl | Op::Shr) {
            self.narrow(ty);
        }
    }
}

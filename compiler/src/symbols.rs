//! Scopes, storage choice and frame layout: the symbol table of the checker.
//!
//! `Symbols` owns every `Symbol` created while checking, the stack of visible names, the
//! static-data cursor, and the bookkeeping of the routine being checked. The checker asks it
//! to declare, look up and place things; it never asks the checker anything. It reports
//! failures as a `SymbolError` that the checker turns into the TP diagnostic, and it never
//! reads or rewrites syntax-tree nodes (other than interning names).
//!
//! # Scope stack
//!
//! Visible names are a vector of `(name, Sym)` entries, innermost last. Looking a name up scans
//! from the end, so an inner declaration hides an outer one. Entering a routine remembers the
//! stack length and leaving it truncates back, which drops the routine's names. The entries
//! below `region_start` (the predefined names, System's, or the names around a routine) may be
//! hidden by a declaration but not duplicated; a name declared twice at or above `region_start`
//! is `AlreadyDeclared`. This is how a program or routine may redeclare `Integer` or a System
//! routine, yet cannot declare one of its own names twice.
//!
//! # Storage and frame layout
//!
//! A global is a fixed address in static data. A local starts as a WASM local; `capture` marks
//! it `needs_frame_slot` when its address is needed (a `var` argument, a use from a nested
//! routine) and `local_variable` does so at once for in-memory types. When a routine's body is
//! done, `place_frame` rewrites those symbols to `Frame` or `FrameRef` offsets in the routine's
//! frame in linear memory: declaration order, no padding, the static link first. Codegen reads
//! the final kinds, so no node of the tree is touched.
//!
//! # Units
//!
//! System and each unit own a region of the stack above `base`, the stack length after System.
//! A unit is checked in two phases (its `uses` may need a unit that is checked later):
//!
//! - Interface: `begin_unit` truncates the stack to `base`, `import` pushes the exports of each
//!   used unit as `imported` entries, the interface declarations follow, and `end_interface`
//!   records them as the unit's exports and saves the whole region.
//! - Implementation: `resume_unit` restores the saved region, the implementation `uses` and
//!   declarations follow.
//!
//! Imported entries do not count for the duplicate check, so a declaration may hide an imported
//! name; they are never exported, so a unit does not re-export what it uses. A later import
//! hides an earlier one: the unit named last in `uses` wins. The program is one more region.
//! A qualified name `U.X` (the parser interns it as one name, see `Ast::qualified`) is looked up
//! in U's exports only. System's exports are everything visible after it is declared.
use crate::ast::{Ast, Qualified};
use crate::types::{ConstValue, FrameCopy, Intrinsic, Routine, Sym, SymKind, Symbol, Ty, Types};

/// The System routines that the compiler calls on its own behalf, bound by name right after
/// System is loaded (docs/decisions.md).
#[derive(Clone, Copy)]
pub enum Helper {
    AbsLong,
    StrCat,
    StrCmp,
    StrAssign,
    StrDelete,
    StrInsert,
    HeapGet,
    HeapPut,
    HeapMark,
    HeapRelease,
    SetOp,
    SetAdd,
    SetIn,
    SetCmp,
    StrInt,
    ValLong,
    ValReal,
    RangeChk,
    HostWrite,
    HostRtError,
    HostHalt,
    FmtReal,
    TextOut,
    TextNewLine,
    TextFlush,
    CheckIo,
    Assign,
    Eof,
    Eoln,
    ReadChar,
    ReadLong,
    ReadReal,
    ReadStr,
    ReadLnEnd,
    Exp,
    Ln,
    Sin,
    Cos,
    ArcTan,
    RandomWord,
    RandomReal,
}

/// Each `Helper` with the name of its routine in `runtime/system.pas`, upper-cased.
pub const HELPERS: &[(Helper, &[u8])] = &[
    (Helper::AbsLong, b"ABSLONG"),
    (Helper::StrCat, b"STRCAT"),
    (Helper::StrCmp, b"STRCMP"),
    (Helper::StrAssign, b"STRASSIGN"),
    (Helper::StrDelete, b"STRDELETE"),
    (Helper::StrInsert, b"STRINSERT"),
    (Helper::HeapGet, b"HEAPGET"),
    (Helper::HeapPut, b"HEAPPUT"),
    (Helper::HeapMark, b"HEAPMARK"),
    (Helper::HeapRelease, b"HEAPRELEASE"),
    (Helper::SetOp, b"SETOP"),
    (Helper::SetAdd, b"SETADD"),
    (Helper::SetIn, b"SETIN"),
    (Helper::SetCmp, b"SETCMP"),
    (Helper::StrInt, b"STRINT"),
    (Helper::ValLong, b"VALLONG"),
    (Helper::ValReal, b"VALREAL"),
    (Helper::RangeChk, b"RANGECHK"),
    (Helper::HostWrite, b"HOSTWRITE"),
    (Helper::HostRtError, b"RTERROR"),
    (Helper::HostHalt, b"HOSTHALT"),
    (Helper::FmtReal, b"FMT_REAL"),
    (Helper::TextOut, b"TEXTOUT"),
    (Helper::TextNewLine, b"TEXTNEWLINE"),
    (Helper::TextFlush, b"TEXTFLUSH"),
    (Helper::CheckIo, b"CHECKIO"),
    (Helper::Assign, b"ASSIGN"),
    (Helper::Eof, b"EOF"),
    (Helper::Eoln, b"EOLN"),
    (Helper::ReadChar, b"READCHAR"),
    (Helper::ReadLong, b"READLONG"),
    (Helper::ReadReal, b"READREAL"),
    (Helper::ReadStr, b"READSTR"),
    (Helper::ReadLnEnd, b"READLNEND"),
    (Helper::Exp, b"EXPREAL"),
    (Helper::Ln, b"LNREAL"),
    (Helper::Sin, b"SINREAL"),
    (Helper::Cos, b"COSREAL"),
    (Helper::ArcTan, b"ARCTANREAL"),
    (Helper::RandomWord, b"RANDOMWORD"),
    (Helper::RandomReal, b"RANDOMREAL"),
];

/// The most bytes of one structure, of all globals, or of one routine's locals.
pub const MAX_DATA: u32 = 65535;

/// Static data: globals start here, after the code generator's fixed header (the characters
/// it writes and its digit buffer, which `codegen.rs` checks fit below this address), rounded
/// up to a multiple of 8.
pub const GLOBALS: u32 = 280;

/// The first 4 bytes of a frame hold the static link of a nested routine.
const STATIC_LINK_BYTES: u32 = 4;

/// Predefined names: types, constants, and the intrinsics the compiler expands itself.
const PREDEFINED: &[(&[u8], SymKind, Ty)] = &[
    (b"BYTE", SymKind::Type, Ty::BYTE),
    (b"SHORTINT", SymKind::Type, Ty::SHORTINT),
    (b"INTEGER", SymKind::Type, Ty::INTEGER),
    (b"WORD", SymKind::Type, Ty::WORD),
    (b"LONGINT", SymKind::Type, Ty::LONGINT),
    (b"BOOLEAN", SymKind::Type, Ty::BOOLEAN),
    (b"CHAR", SymKind::Type, Ty::CHAR),
    (b"REAL", SymKind::Type, Ty::REAL),
    (b"SINGLE", SymKind::Omitted, Ty::NONE),
    (b"DOUBLE", SymKind::Omitted, Ty::NONE),
    (b"EXTENDED", SymKind::Omitted, Ty::NONE),
    (b"COMP", SymKind::Omitted, Ty::NONE),
    (b"SEG", SymKind::Omitted, Ty::NONE),
    (b"OFS", SymKind::Omitted, Ty::NONE),
    (b"PTR", SymKind::Omitted, Ty::NONE),
    (b"POINTER", SymKind::Type, Ty::POINTER),
    (b"NIL", SymKind::Const(ConstValue::Int(0)), Ty::POINTER),
    (b"FALSE", SymKind::Const(ConstValue::Int(0)), Ty::BOOLEAN),
    (b"TRUE", SymKind::Const(ConstValue::Int(1)), Ty::BOOLEAN),
    (b"STRING", SymKind::Type, Ty::STRING),
    (b"WRITE", SymKind::Intrinsic(Intrinsic::Write), Ty::NONE),
    (b"WRITELN", SymKind::Intrinsic(Intrinsic::WriteLn), Ty::NONE),
    (b"READ", SymKind::Intrinsic(Intrinsic::Read), Ty::NONE),
    (b"READLN", SymKind::Intrinsic(Intrinsic::ReadLn), Ty::NONE),
    (b"ORD", SymKind::Intrinsic(Intrinsic::Ord), Ty::NONE),
    (b"CHR", SymKind::Intrinsic(Intrinsic::Chr), Ty::NONE),
    (b"SUCC", SymKind::Intrinsic(Intrinsic::Succ), Ty::NONE),
    (b"PRED", SymKind::Intrinsic(Intrinsic::Pred), Ty::NONE),
    (b"ODD", SymKind::Intrinsic(Intrinsic::Odd), Ty::NONE),
    (b"ABS", SymKind::Intrinsic(Intrinsic::Abs), Ty::NONE),
    (b"SQR", SymKind::Intrinsic(Intrinsic::Sqr), Ty::NONE),
    (b"EXIT", SymKind::Intrinsic(Intrinsic::Exit), Ty::NONE),
    (b"SIZEOF", SymKind::Intrinsic(Intrinsic::SizeOf), Ty::NONE),
    (b"ADDR", SymKind::Intrinsic(Intrinsic::Addr), Ty::NONE),
    (b"CONCAT", SymKind::Intrinsic(Intrinsic::Concat), Ty::NONE),
    (b"DELETE", SymKind::Intrinsic(Intrinsic::Delete), Ty::NONE),
    (b"INSERT", SymKind::Intrinsic(Intrinsic::Insert), Ty::NONE),
    (b"NEW", SymKind::Intrinsic(Intrinsic::New), Ty::NONE),
    (b"DISPOSE", SymKind::Intrinsic(Intrinsic::Dispose), Ty::NONE),
    (b"GETMEM", SymKind::Intrinsic(Intrinsic::GetMem), Ty::NONE),
    (b"FREEMEM", SymKind::Intrinsic(Intrinsic::FreeMem), Ty::NONE),
    (b"MARK", SymKind::Intrinsic(Intrinsic::Mark), Ty::NONE),
    (b"RELEASE", SymKind::Intrinsic(Intrinsic::Release), Ty::NONE),
    (
        b"RUNERROR",
        SymKind::Intrinsic(Intrinsic::RunError),
        Ty::NONE,
    ),
    (b"HALT", SymKind::Intrinsic(Intrinsic::Halt), Ty::NONE),
    (b"ROUND", SymKind::Intrinsic(Intrinsic::Round), Ty::NONE),
    (b"TRUNC", SymKind::Intrinsic(Intrinsic::Trunc), Ty::NONE),
    (b"SQRT", SymKind::Intrinsic(Intrinsic::Sqrt), Ty::NONE),
    (b"INT", SymKind::Intrinsic(Intrinsic::Int), Ty::NONE),
    (b"FRAC", SymKind::Intrinsic(Intrinsic::Frac), Ty::NONE),
    (b"INC", SymKind::Intrinsic(Intrinsic::Inc), Ty::NONE),
    (b"DEC", SymKind::Intrinsic(Intrinsic::Dec), Ty::NONE),
    (b"STR", SymKind::Intrinsic(Intrinsic::Str), Ty::NONE),
    (b"VAL", SymKind::Intrinsic(Intrinsic::Val), Ty::NONE),
    (
        b"FILLCHAR",
        SymKind::Intrinsic(Intrinsic::FillChar),
        Ty::NONE,
    ),
    (b"MOVE", SymKind::Intrinsic(Intrinsic::Move), Ty::NONE),
    (
        b"SETTEXTBUF",
        SymKind::Intrinsic(Intrinsic::SetTextBuf),
        Ty::NONE,
    ),
    (b"EXP", SymKind::Intrinsic(Intrinsic::Exp), Ty::NONE),
    (b"LN", SymKind::Intrinsic(Intrinsic::Ln), Ty::NONE),
    (b"SIN", SymKind::Intrinsic(Intrinsic::Sin), Ty::NONE),
    (b"COS", SymKind::Intrinsic(Intrinsic::Cos), Ty::NONE),
    (b"ARCTAN", SymKind::Intrinsic(Intrinsic::ArcTan), Ty::NONE),
    (b"RANDOM", SymKind::Intrinsic(Intrinsic::Random), Ty::NONE),
    (
        b"MAXINT",
        SymKind::Const(ConstValue::Int(32767)),
        Ty::INTEGER,
    ),
    (
        b"MAXLONGINT",
        SymKind::Const(ConstValue::Int(2147483647)),
        Ty::LONGINT,
    ),
];

/// Why a symbol operation failed; the checker maps each to its TP error.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SymbolError {
    /// The name is already declared in the current region.
    AlreadyDeclared,
    /// The globals, or one routine's locals, would exceed `MAX_DATA` bytes.
    TooManyVariables,
}

/// A name System must declare for the compiler to work is missing.
#[derive(Clone, Copy, Debug)]
pub struct MissingSystemName;

/// What `Symbols::bind_system_names` found in System.
pub struct SystemNames {
    /// The routine index of each `Helper`.
    pub helpers: [u32; HELPERS.len()],
    /// Address of `HeapOrg`; the next 4 bytes are `HeapPtr`.
    pub heap: u32,
    /// System's `Text` type and its console input file `Input`.
    pub text: Ty,
    pub input: Sym,
}

/// What a routine body saves on entry and restores on exit.
pub struct SavedScope {
    scope_len: usize,
    region_start: usize,
    next_local: u32,
    local_bytes: u32,
}

/// A visible name. `imported` marks a name that a `uses` clause made visible.
#[derive(Clone, Copy)]
struct Entry {
    name: u32,
    sym: Sym,
    imported: bool,
}

impl Entry {
    fn export(&self) -> Export {
        Export {
            name: self.name,
            sym: self.sym,
        }
    }
}

/// A name a unit makes visible to the units that use it.
#[derive(Clone, Copy)]
struct Export {
    name: u32,
    sym: Sym,
}

/// What the symbol table keeps of a unit between its two checking phases.
struct UnitScope {
    name: u32,
    /// The unit's interface declarations.
    exports: Vec<Export>,
    /// The unit's region after its interface part, restored for the implementation part.
    saved: Vec<Entry>,
}

pub struct Symbols {
    syms: Vec<Symbol>,
    /// Visible names, innermost last.
    scope: Vec<Entry>,
    /// The stack length after System: every unit and the program start from it.
    base: usize,
    units: Vec<UnitScope>,
    qualified: Vec<Qualified>,
    /// Scope entries below this index may be hidden but not duplicated by a declaration.
    region_start: usize,
    /// First address after the globals declared so far.
    data_end: u32,
    /// The routines being checked, outermost first; its length is the current depth.
    routine_stack: Vec<usize>,
    /// The next free WASM local index of the current routine.
    next_local: u32,
    /// Bytes of the current routine's local variables.
    local_bytes: u32,
}

impl Symbols {
    /// A table holding only the predefined names.
    pub fn new(ast: &mut Ast) -> Self {
        let mut symbols = Symbols {
            syms: Vec::new(),
            scope: Vec::new(),
            base: 0,
            units: Vec::new(),
            qualified: std::mem::take(&mut ast.qualified),
            region_start: 0,
            data_end: GLOBALS,
            routine_stack: Vec::new(),
            next_local: 0,
            local_bytes: 0,
        };
        for &(name, kind, ty) in PREDEFINED {
            let name = ast.intern(name);
            symbols.bind(name, Symbol::new(kind, ty));
        }
        symbols
    }

    /// Finds the System routines the compiler calls on its own (`Helper`), `HeapOrg`, `Text` and
    /// `Input`, before any program declaration can shadow them.
    pub fn bind_system_names(&self, ast: &mut Ast) -> Result<SystemNames, MissingSystemName> {
        let mut helpers = [0; HELPERS.len()];
        for &(helper, routine) in HELPERS {
            let name = ast.intern(routine);
            helpers[helper as usize] = self.routine_named(name).ok_or(MissingSystemName)?;
        }
        let heap = self.var_named(ast.intern(b"HEAPORG"));
        let text = self.type_named(ast.intern(b"TEXT"));
        let input = self.lookup(ast.intern(b"INPUT"));
        let (Some(heap), Some(text), Some(input)) = (heap, text, input) else {
            return Err(MissingSystemName);
        };
        Ok(SystemNames {
            helpers,
            heap,
            text,
            input,
        })
    }

    /// The routine index of the routine `name` stands for.
    fn routine_named(&self, name: u32) -> Option<u32> {
        match self.lookup_symbol(name)?.1.kind {
            SymKind::Routine(r) => Some(r),
            _ => None,
        }
    }

    /// The static address of the global variable `name` stands for.
    fn var_named(&self, name: u32) -> Option<u32> {
        match self.lookup_symbol(name)?.1.kind {
            SymKind::Var(address) => Some(address),
            _ => None,
        }
    }

    /// The type `name` stands for.
    fn type_named(&self, name: u32) -> Option<Ty> {
        let symbol = self.lookup_symbol(name)?.1;
        (symbol.kind == SymKind::Type).then_some(symbol.ty)
    }

    /// System is declared: its names, with the predefined ones, are the unit `name`'s exports,
    /// and may now be redeclared.
    pub fn end_system(&mut self, name: u32) {
        self.base = self.scope.len();
        self.region_start = self.base;
        let exports = self.scope.iter().map(Entry::export).collect();
        self.units.push(UnitScope {
            name,
            exports,
            saved: Vec::new(),
        });
    }

    /// Starts the region of the unit `name`, with only System's names visible.
    pub fn begin_unit(&mut self, name: u32) {
        self.reset_region();
        self.units.push(UnitScope {
            name,
            exports: Vec::new(),
            saved: Vec::new(),
        });
    }

    /// The interface part of the unit begun last is done: its own declarations are its exports.
    pub fn end_interface(&mut self) {
        let exports = self.own_entries().map(Entry::export).collect();
        let saved = self.scope[self.base..].to_vec();
        let unit = self.units.last_mut().expect("a unit was begun");
        unit.exports = exports;
        unit.saved = saved;
    }

    /// Returns to the region of the unit `name` as its interface part left it.
    pub fn resume_unit(&mut self, name: u32) {
        let saved = &unit_scope(&self.units, name).saved;
        self.scope.truncate(self.base);
        self.scope.extend_from_slice(saved);
        self.region_start = self.base;
    }

    /// Starts the program's region with only System's names visible.
    pub fn begin_program(&mut self) {
        self.reset_region();
    }

    /// Drops every name above System's and starts a new region there.
    fn reset_region(&mut self) {
        self.scope.truncate(self.base);
        self.region_start = self.base;
    }

    /// The entries declared in the current region, not imported.
    fn own_entries(&self) -> impl Iterator<Item = &Entry> {
        self.scope[self.region_start..]
            .iter()
            .filter(|e| !e.imported)
    }

    /// Makes the exports of the unit `name` visible, hiding earlier names of the same spelling.
    pub fn import(&mut self, name: u32) {
        let exports = &unit_scope(&self.units, name).exports;
        let entries = exports.iter().map(|e| Entry {
            name: e.name,
            sym: e.sym,
            imported: true,
        });
        self.scope.extend(entries);
    }

    /// The symbols declared in the current region, not imported.
    pub fn own_symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.own_entries().map(|e| self.symbol(e.sym))
    }

    /// True while System's own declarations (and the predefined names) are being declared.
    pub fn in_system_region(&self) -> bool {
        self.region_start == 0
    }

    /// Adds a symbol and makes `name` refer to it from here on. Depth is the current one.
    fn bind(&mut self, name: u32, symbol: Symbol) -> Sym {
        let sym = self.add_unnamed(symbol);
        self.scope.push(Entry {
            name,
            sym,
            imported: false,
        });
        sym
    }

    /// Adds a symbol that no name refers to (a function's result).
    fn add_unnamed(&mut self, symbol: Symbol) -> Sym {
        self.syms.push(Symbol {
            depth: self.depth(),
            ..symbol
        });
        Sym(self.syms.len() as u32 - 1)
    }

    /// Declares `name` in the current region; a second declaration in the same region is an
    /// error, but a declaration may hide a name from an outer region.
    pub fn declare(&mut self, name: u32, symbol: Symbol) -> Result<Sym, SymbolError> {
        if self.lookup_in_region(name).is_some() {
            return Err(SymbolError::AlreadyDeclared);
        }
        Ok(self.bind(name, symbol))
    }

    pub fn lookup(&self, name: u32) -> Option<Sym> {
        if let Some(q) = self.qualified.iter().find(|q| q.joined == name) {
            return self.exported(q.unit, q.member);
        }
        self.scope
            .iter()
            .rev()
            .find(|e| e.name == name)
            .map(|e| e.sym)
    }

    /// What the unit `unit` exports under `member`.
    fn exported(&self, unit: u32, member: u32) -> Option<Sym> {
        let unit = self.units.iter().find(|u| u.name == unit)?;
        unit.exports
            .iter()
            .rev()
            .find(|e| e.name == member)
            .map(|e| e.sym)
    }

    pub fn symbol(&self, sym: Sym) -> Symbol {
        self.syms[sym.0 as usize]
    }

    pub fn lookup_symbol(&self, name: u32) -> Option<(Sym, Symbol)> {
        let sym = self.lookup(name)?;
        Some((sym, self.symbol(sym)))
    }

    /// The symbol `name` has in the current region, ignoring outer regions.
    pub fn lookup_in_region(&self, name: u32) -> Option<Symbol> {
        let entry = self.own_entries().find(|e| e.name == name);
        entry.map(|e| self.symbol(e.sym))
    }

    /// The symbols declared by name since `saved` was taken, that is, in the routine entered.
    pub fn declared_since(&self, saved: &SavedScope) -> impl Iterator<Item = Symbol> + '_ {
        self.scope[saved.scope_len..]
            .iter()
            .map(|e| self.symbol(e.sym))
    }

    /// The number of symbols created so far; the next symbol's index.
    pub fn symbol_count(&self) -> usize {
        self.syms.len()
    }

    /// All symbols, for the code generator.
    pub fn into_symbols(self) -> Vec<Symbol> {
        self.syms
    }

    // ---- Routine nesting ----

    /// The lexical depth of the routine being checked: 0 for the program.
    pub fn depth(&self) -> u32 {
        self.routine_stack.len() as u32
    }

    pub fn is_nested(&self) -> bool {
        !self.routine_stack.is_empty()
    }

    /// The routine being checked, if any.
    pub fn current_routine(&self) -> Option<usize> {
        self.routine_stack.last().copied()
    }

    /// True if routine `r` is being checked now, either as the current routine or one around it.
    pub fn is_being_checked(&self, r: usize) -> bool {
        self.routine_stack.contains(&r)
    }

    /// Starts routine `r`'s region. Its first WASM local is `first_local`.
    pub fn enter_routine(&mut self, r: usize, first_local: u32) -> SavedScope {
        let saved = SavedScope {
            scope_len: self.scope.len(),
            region_start: self.region_start,
            next_local: self.next_local,
            local_bytes: std::mem::take(&mut self.local_bytes),
        };
        self.region_start = saved.scope_len;
        self.next_local = first_local;
        self.routine_stack.push(r);
        saved
    }

    /// Drops the routine's names and restores the state of the routine around it.
    pub fn leave_routine(&mut self, saved: SavedScope) {
        self.routine_stack.pop();
        self.scope.truncate(saved.scope_len);
        self.region_start = saved.region_start;
        self.next_local = saved.next_local;
        self.local_bytes = saved.local_bytes;
    }

    /// The next free WASM local index; after the body it is the routine's local count.
    pub fn next_local(&self) -> u32 {
        self.next_local
    }

    // ---- Storage ----

    /// The address the next global will get.
    pub fn data_end(&self) -> u32 {
        self.data_end
    }

    /// A global lives at the next free static address.
    pub fn global_variable(&mut self, ty: Ty, size: u32) -> Result<Symbol, SymbolError> {
        let address = self.data_end;
        self.data_end += size;
        if self.data_end - GLOBALS > MAX_DATA {
            return Err(SymbolError::TooManyVariables);
        }
        Ok(Symbol::new(SymKind::Var(address), ty))
    }

    /// A local is the next WASM local; aggregates and other in-memory types always live in
    /// the frame.
    pub fn local_variable(
        &mut self,
        types: &Types,
        ty: Ty,
        size: u32,
    ) -> Result<Symbol, SymbolError> {
        let index = self.take_local();
        self.local_bytes += size;
        if self.local_bytes > MAX_DATA {
            return Err(SymbolError::TooManyVariables);
        }
        Ok(Symbol {
            needs_frame_slot: types.in_memory(ty),
            ..Symbol::new(SymKind::Local(index), ty)
        })
    }

    /// A parameter is WASM local `index`: its value, or its address for a `var` parameter. An
    /// in-memory value parameter arrives as an address and is copied to the frame.
    pub fn parameter(types: &Types, index: u32, ty: Ty, by_ref: bool) -> Symbol {
        let kind = if by_ref {
            SymKind::Ref(index)
        } else {
            SymKind::Local(index)
        };
        Symbol {
            needs_frame_slot: !by_ref && types.in_memory(ty),
            ..Symbol::new(kind, ty)
        }
    }

    /// The function result is a hidden parameter holding its address when it lives in
    /// memory, otherwise a local after the parameters. It has no name: assignments to the
    /// function's own name are redirected to it.
    pub fn declare_result(&mut self, routine: &Routine) -> Sym {
        let kind = if routine.result_ptr {
            SymKind::Ref(routine.has_static_link() as u32)
        } else {
            SymKind::Local(self.take_local())
        };
        self.add_unnamed(Symbol::new(kind, routine.result))
    }

    fn take_local(&mut self) -> u32 {
        let index = self.next_local;
        self.next_local += 1;
        index
    }

    /// A reference to `sym` from the current routine: a `Local` or `Ref` of an enclosing
    /// routine is captured, a `Local` passed as a `var` argument is address-taken; both move
    /// it to the frame.
    pub fn capture(&mut self, sym: Sym, by_ref: bool) {
        let depth = self.depth();
        let symbol = &mut self.syms[sym.0 as usize];
        let is_local_or_ref = matches!(symbol.kind, SymKind::Local(_) | SymKind::Ref(_));
        let used_by_nested_routine = is_local_or_ref && symbol.depth < depth;
        let passed_by_reference = by_ref && matches!(symbol.kind, SymKind::Local(_));
        symbol.needs_frame_slot |= used_by_nested_routine || passed_by_reference;
    }

    /// Moves the locals that need a frame slot (symbols of the current `routine` from
    /// `first_symbol` on) to the frame, in declaration order with no padding. A routine with a
    /// static link keeps it in the first 4 bytes. A `var` parameter keeps its address in the
    /// frame. A parameter moved this way is copied there by the prologue (`FrameCopy`).
    pub fn place_frame(&mut self, routine: &mut Routine, first_symbol: usize, types: &Types) {
        let depth = self.depth();
        let parameter_locals = routine.params.len() as u32 + routine.hidden_params();
        let mut size = if routine.has_static_link() {
            STATIC_LINK_BYTES
        } else {
            0
        };
        let first_slot = size;
        let mut copies = Vec::new();
        for s in &mut self.syms[first_symbol..] {
            if s.depth != depth || !s.needs_frame_slot {
                continue;
            }
            let (is_ref, local) = match s.kind {
                SymKind::Local(local) => (false, local),
                SymKind::Ref(local) => (true, local),
                _ => continue,
            };
            // A `var` parameter's frame slot holds its 4-byte address.
            let ty = if is_ref { Ty::LONGINT } else { s.ty };
            if local < parameter_locals {
                copies.push(FrameCopy {
                    local,
                    offset: size,
                    ty,
                });
            }
            s.kind = if is_ref {
                SymKind::FrameRef(size)
            } else {
                SymKind::Frame(size)
            };
            size += types.size(ty);
        }
        // A routine with nested routines needs a frame even without captured locals: the
        // nested ones receive its frame pointer as their static link.
        routine.framed = size > first_slot || routine.has_children;
        routine.frame = size;
        routine.copies = copies;
    }
}

/// The saved state of the unit `name`, which the loader found and the checker began. A free
/// function so that the caller can borrow `units` while it changes `scope`.
fn unit_scope(units: &[UnitScope], name: u32) -> &UnitScope {
    let unit = units.iter().find(|u| u.name == name);
    unit.expect("every used unit was loaded and begun")
}

//! Whole-program checker: declarations, name resolution, typing and constant folding.
//!
//! The checker walks the parser's tree once and rewrites it in place: a `Name` becomes a
//! `Var`, a constant, or a call; every expression node gets its type; constant expressions
//! are folded.
//!
//! Where each variable lives is decided by `symbols.rs`, which the checker asks: a variable is
//! first a WASM local, or a global at a fixed address; `capture` marks a local that needs an
//! address, and `place_frame` moves those into the routine's frame in linear memory once its
//! body is checked. A routine nested in another receives the enclosing frame pointer as a
//! hidden first parameter, the static link; a function returning a String receives the address
//! of its result as a further hidden parameter (see `Routine::hidden_params`).
//!
//! Units are checked in two phases: System, then every unit's interface in `Plan` order, then
//! every implementation, then the program. A unit's initialisation part becomes an `InitPart`,
//! and the program body is rewritten to run those first.
//!
//! A typed constant is a static variable. Its initialiser is checked here (`initial_value`);
//! codegen lays out its bytes (`static_image`). A routine declared `external` (runtime
//! sources only) is a host import: it has no body. `Write` and `Read` get an explicit file
//! argument (the console when none is given) and `Read` becomes one helper call per item
//! (`ReadChar`, `ReadLong`, ...).
//!
//! Several intrinsics are rewritten into an assignment of a System helper's result:
//! `Inc(x, n)` is `x := Ord(x) + n`, `New(p)` is `p := HeapGet(size)`, `Delete(s, i, n)` is
//! `s := StrDelete(s, i, n)`, `Str`, `Val`, `Insert`, `GetMem`, `Mark` likewise; `Concat` is
//! a chain of `+`, and `Exp`, `Sin` and the like are calls of System routines. Codegen then
//! needs no knowledge of them. `FillChar` and `Move` become `FillMem` and `CopyMem` nodes
//! whose addresses are `@x` nodes, because an untyped `var` parameter has no Pascal type.
use crate::ast::{Ast, Id, Kind, List, Op, Plan, NO_NAME};
use crate::diag::{code, Diag};
use crate::symbols::{Helper, SymbolError, Symbols, HELPERS, MAX_DATA};
use crate::types::{
    ConstValue, Field, Intrinsic, Origin, ParamType, Routine, Signature, Sym, SymKind, Symbol, Tag,
    Ty, TypedConst, Types,
};

pub struct Info {
    pub types: Types,
    pub syms: Vec<Symbol>,
    pub routines: Vec<Routine>,
    /// First address after the globals.
    pub data_end: u32,
    /// The typed constants in address order.
    pub typed_consts: Vec<TypedConst>,
    /// The routine index of each `Helper`.
    helpers: [u32; HELPERS.len()],
    /// Address of System's `HeapOrg`; the next 4 bytes are `HeapPtr`.
    pub heap: u32,
}

impl Info {
    /// The routine index of a System helper.
    pub fn helper(&self, helper: Helper) -> u32 {
        self.helpers[helper as usize]
    }

    /// Whether a call of routine `r` is followed by a check of `InOutRes` under `{$I+}`: the
    /// routines whose first parameter is a text file, except `Assign`, which never checks.
    pub fn checks_io(&self, r: u32) -> bool {
        let first = self.routines[r as usize].params.first();
        first.is_some_and(|p| self.types.is_file(p.ty)) && r != self.helper(Helper::Assign)
    }
}

type R<T> = Result<T, Diag>;

/// The file of a `Read` or `Write` and where its items begin.
struct FileArg {
    file: Id,
    /// Index of the first item that is not the file itself.
    first_item: usize,
    /// The item at index 0 has already been checked as an expression (it was a variable that
    /// turned out not to be a file).
    first_item_checked: bool,
}

/// A record field reached by its bare name inside a `with`: the record's designator, and the
/// field's type and byte offset.
struct WithField {
    record: Id,
    ty: Ty,
    offset: u32,
}

/// A parameter as declared, with the node that declares it.
struct ParamDecl {
    node: Id,
    name: u32,
    type_name: u32,
    by_ref: bool,
}

struct Cx<'a> {
    ast: &'a mut Ast,
    types: Types,
    symbols: Symbols,
    typed_consts: Vec<TypedConst>,
    routines: Vec<Routine>,
    /// The record designators of the enclosing `with` statements, innermost last.
    withs: Vec<(Id, Ty)>,
    /// How many calls enclose the code being checked, the call itself included.
    call_depth: u32,
    helpers: [u32; HELPERS.len()],
    /// System's console input file, the file of `Read` and `Eof` without one.
    input: Sym,
    /// Pointer types whose target is declared later in the same type section.
    forward: Vec<(Ty, u32)>,
    /// The `InitPart` of each unit checked so far, in initialisation order.
    init_parts: Vec<Id>,
}

/// Checks System, the interface of every unit, then every implementation, then the program.
/// Two phases, because a unit's implementation may use a unit whose interface completes
/// later (`Plan` has the orders).
pub fn check(ast: &mut Ast, plan: &Plan) -> R<Info> {
    // Nodes the checker makes are not range checked unless they copy a checked node.
    ast.range_checks = false;
    ast.io_checks = false;
    let symbols = Symbols::new(ast);
    let mut cx = Cx {
        ast,
        types: Types::new(),
        symbols,
        typed_consts: Vec::new(),
        routines: Vec::new(),
        withs: Vec::new(),
        call_depth: 0,
        helpers: [0; HELPERS.len()],
        input: Sym(0),
        forward: Vec::new(),
        init_parts: Vec::new(),
    };
    let heap = cx.system(plan.system)?;
    for &unit in &plan.interface_order {
        cx.unit_interface(unit)?;
    }
    for &unit in &plan.init_order {
        cx.unit_implementation(unit)?;
    }
    cx.program(plan.program)?;
    Ok(Info {
        types: cx.types,
        data_end: cx.symbols.data_end(),
        syms: cx.symbols.into_symbols(),
        routines: cx.routines,
        typed_consts: cx.typed_consts,
        helpers: cx.helpers,
        heap,
    })
}

/// Value of a constant operator, computed at LongInt precision. The unary operators use only
/// `a`; the operators that are not constant (`Slash`, `Round`, ...) fold to 0 and are never
/// called with constants.
fn fold(op: Op, a: i64, b: i64) -> i64 {
    let (x, y) = (a as i32, b as i32);
    let v = match op {
        Op::Add => x.wrapping_add(y),
        Op::Sub => x.wrapping_sub(y),
        Op::Mul => x.wrapping_mul(y),
        Op::Div => x.wrapping_div(y),
        Op::Mod => x.wrapping_rem(y),
        Op::And => x & y,
        Op::Or => x | y,
        Op::Xor => x ^ y,
        Op::Shl => x.wrapping_shl(y as u32),
        Op::Shr => (x as u32).wrapping_shr(y as u32) as i32,
        Op::Eq => (x == y) as i32,
        Op::Ne => (x != y) as i32,
        Op::Lt => (x < y) as i32,
        Op::Gt => (x > y) as i32,
        Op::Le => (x <= y) as i32,
        Op::Ge => (x >= y) as i32,
        Op::In => 0,
        Op::Neg => x.wrapping_neg(),
        Op::Not => !x,
        Op::Ord => x,
        Op::Chr => x & 255,
        Op::Succ => x.wrapping_add(1),
        Op::Pred => x.wrapping_sub(1),
        Op::Odd => x & 1,
        Op::Abs => x.wrapping_abs(),
        Op::Sqr => x.wrapping_mul(x),
        Op::Slash | Op::Round | Op::Trunc | Op::Sqrt | Op::Int | Op::Frac => 0,
    };
    v as i64
}

impl Cx<'_> {
    /// Finds the System names the compiler relies on; a gap is an error in System itself, so
    /// it is reported at line 0.
    fn bind_system_names(&mut self) -> R<u32> {
        let broken_system = Diag {
            code: code::UNKNOWN_IDENTIFIER,
            line: 0,
            col: 0,
            file: 0,
        };
        let names = self
            .symbols
            .bind_system_names(self.ast)
            .map_err(|_| broken_system)?;
        if self.routines.iter().any(|r| !r.defined) {
            return Err(broken_system.with_code(code::UNDEFINED_FORWARD));
        }
        self.helpers = names.helpers;
        self.types.text = names.text;
        self.input = names.input;
        Ok(names.heap)
    }

    fn err(&self, id: Id, code: u16) -> Diag {
        let node = self.ast.node(id);
        Diag {
            code,
            line: node.line,
            col: node.col,
            file: node.file,
        }
    }

    // ---- Units and the program ----

    /// System: its declarations, then the names the compiler relies on. Returns `HeapOrg`.
    fn system(&mut self, unit: Id) -> R<u32> {
        let Kind::Unit {
            name, interface, ..
        } = self.ast.kind(unit)
        else {
            unreachable!("System is a unit")
        };
        for d in self.ast.items_vec(interface) {
            self.decl(d)?;
        }
        let heap = self.bind_system_names()?;
        self.symbols.end_system(name);
        Ok(heap)
    }

    /// Makes the exports of each unit in a `uses` list visible.
    fn import_uses(&mut self, uses: List) {
        for &entry in self.ast.items(uses) {
            self.symbols.import(self.ast.name_of(entry));
        }
    }

    /// A unit's interface part: its declarations become the unit's exports. Interface routine
    /// headings have no body here, so they are forwards that the implementation completes.
    fn unit_interface(&mut self, unit: Id) -> R<()> {
        let Kind::Unit {
            name,
            interface_uses,
            interface,
            ..
        } = self.ast.kind(unit)
        else {
            unreachable!("a unit")
        };
        // Nodes made while checking this file carry its number.
        self.ast.file = self.ast.node(unit).file;
        self.symbols.begin_unit(name);
        self.import_uses(interface_uses);
        for d in self.ast.items_vec(interface) {
            self.decl(d)?;
        }
        self.resolve_forward(unit)?;
        self.symbols.end_interface();
        Ok(())
    }

    /// A unit's implementation part and its initialisation, which becomes an `InitPart`. A
    /// routine of the unit that is still undefined is error 59 at the unit's `end`.
    fn unit_implementation(&mut self, unit: Id) -> R<()> {
        let Kind::Unit {
            name,
            implementation_uses,
            implementation,
            body,
            ..
        } = self.ast.kind(unit)
        else {
            unreachable!("a unit")
        };
        self.ast.file = self.ast.node(unit).file;
        self.symbols.resume_unit(name);
        self.import_uses(implementation_uses);
        for d in self.ast.items_vec(implementation) {
            self.decl(d)?;
        }
        self.resolve_forward(unit)?;
        if self.symbols.own_symbols().any(|s| self.is_undefined(s)) {
            return Err(self.err(unit, code::UNDEFINED_FORWARD));
        }
        self.stmt(body)?;
        let (line, col) = self.ast.position(body);
        let part = self.ast.add(Kind::InitPart(body), line, col);
        self.init_parts.push(part);
        Ok(())
    }

    /// The program. Its body becomes the unit initialisation parts followed by its own
    /// statements, so code generation sees one body.
    fn program(&mut self, root: Id) -> R<()> {
        let Kind::Program {
            name,
            uses,
            declarations,
            body,
        } = self.ast.kind(root)
        else {
            unreachable!("the root is a program")
        };
        self.ast.file = self.ast.node(root).file;
        self.symbols.begin_program();
        self.import_uses(uses);
        if name != NO_NAME {
            self.declare(root, name, Symbol::new(SymKind::Prog, Ty::NONE))?;
        }
        for d in self.ast.items_vec(declarations) {
            self.decl(d)?;
        }
        self.resolve_forward(body)?;
        if self.routines.iter().any(|r| !r.defined) {
            return Err(self.err(root, code::UNDEFINED_FORWARD));
        }
        self.stmt(body)?;
        let mut statements = std::mem::take(&mut self.init_parts);
        statements.push(body);
        let (line, col) = self.ast.position(body);
        let statements = self.ast.list(&statements);
        let whole = self.ast.add(Kind::Compound(statements), line, col);
        let program = Kind::Program {
            name,
            uses,
            declarations,
            body: whole,
        };
        self.ast.set_kind(root, program);
        Ok(())
    }

    /// A routine declared `forward` (or in a unit's interface) that has no body yet.
    fn is_undefined(&self, symbol: Symbol) -> bool {
        matches!(symbol.kind, SymKind::Routine(r) if !self.routines[r as usize].defined)
    }

    /// Declares `name` in the current region; a second declaration in the same region is an
    /// error, but a user declaration may hide a predefined or System one.
    fn declare(&mut self, at: Id, name: u32, symbol: Symbol) -> R<Sym> {
        let declared = self.symbols.declare(name, symbol);
        declared.map_err(|error| self.symbol_error(at, error))
    }

    /// The TP error for a failed symbol operation at node `at`.
    fn symbol_error(&self, at: Id, error: SymbolError) -> Diag {
        match error {
            SymbolError::AlreadyDeclared => self.err(at, code::DUPLICATE_IDENTIFIER),
            SymbolError::TooManyVariables => self.err(at, code::TOO_MANY_VARIABLES),
        }
    }

    /// Resolves the pointer types declared with a target not yet declared. Called where a type
    /// section ends; TP reports an undefined target at `at`, the next declaration or the body.
    fn resolve_forward(&mut self, at: Id) -> R<()> {
        for (pointer, name) in std::mem::take(&mut self.forward) {
            let to = self.type_of(at, name).map_err(|d| match d.code {
                code::NOT_SUPPORTED => d,
                _ => d.with_code(code::UNDEFINED_POINTER_TARGET),
            })?;
            self.types.set_pointer_target(pointer, to);
        }
        Ok(())
    }

    // ---- Declarations ----

    fn decl(&mut self, d: Id) -> R<()> {
        // A type section ends at the first declaration that is not a type.
        if !matches!(self.ast.kind(d), Kind::TypeDecl(..)) {
            self.resolve_forward(d)?;
        }
        match self.ast.kind(d) {
            Kind::ConstDecl(name, value) => self.const_decl(d, name, value),
            Kind::TypedConst {
                name,
                type_name,
                init,
            } => self.typed_const_decl(d, name, type_name, init),
            Kind::VarDecl(name, type_name) => self.var_decl(d, name, type_name),
            Kind::TypeDecl(name, spec) => {
                let ty = self.spec(spec, name)?;
                self.declare(d, name, Symbol::new(SymKind::Type, ty))?;
                Ok(())
            }
            Kind::Routine { .. } => self.routine(d),
            _ => unreachable!("declarations only"),
        }
    }

    /// A constant must fold to an integer, Real, string or set of constants.
    fn const_decl(&mut self, d: Id, name: u32, value: Id) -> R<()> {
        self.expr(value)?;
        let node = self.ast.node(value);
        let (constant, ty) = match node.kind {
            Kind::IntLit(v) => (ConstValue::Int(v), node.ty),
            Kind::RealLit(v) => (ConstValue::Real(v), Ty::REAL),
            Kind::StrLit(i) => (ConstValue::Str(i), Ty::STRING),
            Kind::SetLit(_) if self.is_constant_literal(value) => (ConstValue::Set(value), node.ty),
            _ => return Err(self.err(value, code::CANNOT_EVALUATE)),
        };
        self.declare(d, name, Symbol::new(SymKind::Const(constant), ty))?;
        Ok(())
    }

    /// `name: type = init`: a variable in static storage, even inside a routine, so its value
    /// outlives each call. The initialiser is checked here; codegen lays out its bytes
    /// (`static_image`) when a live body names the constant.
    fn typed_const_decl(&mut self, d: Id, name: u32, type_name: u32, init: Id) -> R<()> {
        let ty = self.type_of(d, type_name)?;
        if self.types.sig(ty).is_some() {
            return Err(self.err(d, code::PROCEDURE_TYPE_NOT_ALLOWED));
        }
        self.initial_value(ty, init)?;
        let address = self.symbols.data_end();
        let size = self.types.size(ty);
        let symbol = self.symbols.global_variable(ty, size);
        let symbol = symbol.map_err(|error| self.symbol_error(d, error))?;
        self.typed_consts.push(TypedConst { address, ty, init });
        self.declare(d, name, symbol)?;
        Ok(())
    }

    /// Checks that `init` is a constant of type `ty`, following the shape of the type.
    fn initial_value(&mut self, ty: Ty, init: Id) -> R<()> {
        // A record initialiser for another type is rejected here: the array and scalar paths
        // below would read it as an expression.
        if matches!(self.ast.kind(init), Kind::RecordInit(_)) && !self.types.is_record(ty) {
            return Err(self.err(init, code::TYPE_MISMATCH));
        }
        if self.types.is_record(ty) {
            return self.record_initialiser(ty, init);
        }
        match self.types.array_of(ty) {
            // `array_of` also describes a String; a String initialiser is a scalar one.
            Some(array) if !self.types.is_str(ty) => {
                self.array_initialiser(init, (array.hi - array.lo + 1) as usize, array.elem)
            }
            _ => self.scalar_initialiser(ty, init),
        }
    }

    /// `(a, b, c)` with exactly `count` elements, or a string for an array of Char.
    fn array_initialiser(&mut self, init: Id, count: usize, elem: Ty) -> R<()> {
        let Kind::ArrayInit(items) = self.ast.kind(init) else {
            return self.char_array_string(init, count, elem);
        };
        let items = self.ast.items_vec(items);
        for (n, &item) in items.iter().enumerate() {
            if n == count {
                return Err(self.err(item, code::RIGHT_PAREN_EXPECTED));
            }
            self.initial_value(elem, item)?;
        }
        if items.len() < count {
            return Err(self.err(init, code::COMMA_EXPECTED));
        }
        Ok(())
    }

    /// `array[1..4] of Char = 'ABCD'`: the string must have exactly as many characters as the
    /// array has elements.
    fn char_array_string(&mut self, init: Id, count: usize, elem: Ty) -> R<()> {
        self.expr(init)?;
        self.coerce_str(init);
        match self.ast.kind(init) {
            Kind::StrLit(s) if self.types.tag(elem) == Some(Tag::Char) => {
                if self.ast.strs[s as usize].len() == count {
                    return Ok(());
                }
                Err(self.err(init, code::STRING_LENGTH_MISMATCH))
            }
            _ => Err(self.err(init, code::TYPE_MISMATCH)),
        }
    }

    /// `(f: a; g: b)`: the entries name the first fields of the record, in declaration order.
    fn record_initialiser(&mut self, ty: Ty, init: Id) -> R<()> {
        let Kind::RecordInit(entries) = self.ast.kind(init) else {
            return Err(self.err(init, code::FIELD_IDENTIFIER_EXPECTED));
        };
        let fields = self.types.record_fields(ty);
        for (n, entry) in self.ast.items_vec(entries).into_iter().enumerate() {
            let Kind::FieldInit(name, value) = self.ast.kind(entry) else {
                unreachable!("a record initialiser holds field entries")
            };
            let Some(field) = fields.get(n).filter(|field| field.name == name) else {
                let is_a_field = self.types.field(ty, name).is_some();
                let code = if is_a_field {
                    code::INVALID_FIELD_ORDER
                } else {
                    code::FIELD_IDENTIFIER_EXPECTED
                };
                return Err(self.err(entry, code));
            };
            self.initial_value(field.ty, value)?;
        }
        Ok(())
    }

    /// A scalar, string, set or pointer: a constant expression assignable to `ty`. `(value)`
    /// becomes the value itself.
    fn scalar_initialiser(&mut self, ty: Ty, init: Id) -> R<()> {
        if let Kind::ArrayInit(items) = self.ast.kind(init) {
            let &[inner] = self.ast.items(items) else {
                return Err(self.err(init, code::RIGHT_PAREN_EXPECTED));
            };
            self.scalar_initialiser(ty, inner)?;
            let value = self.ast.node(inner);
            self.ast.resolve(init, value.kind, value.ty);
            return Ok(());
        }
        self.expr(init)?;
        self.assignable(init, ty, init)?;
        if !self.is_constant_literal(init) {
            return Err(self.err(init, code::CANNOT_EVALUATE));
        }
        Ok(())
    }

    /// An integer, Real or string literal, or a set literal whose elements are all constant.
    fn is_constant_literal(&self, value: Id) -> bool {
        match self.ast.kind(value) {
            Kind::IntLit(_) | Kind::RealLit(_) | Kind::StrLit(_) => true,
            Kind::SetLit(items) => self
                .ast
                .items(items)
                .iter()
                .all(|&i| self.is_constant_element(i)),
            _ => false,
        }
    }

    /// A set element or range whose ends are integer constants.
    fn is_constant_element(&self, item: Id) -> bool {
        let is_literal = |i: Id| matches!(self.ast.kind(i), Kind::IntLit(_));
        match self.ast.kind(item) {
            Kind::Range(lo, hi) => is_literal(lo) && is_literal(hi),
            _ => is_literal(item),
        }
    }

    fn var_decl(&mut self, d: Id, name: u32, type_name: u32) -> R<()> {
        let ty = self.type_of(d, type_name)?;
        let size = self.types.size(ty);
        let symbol = if self.symbols.is_nested() {
            self.symbols.local_variable(&self.types, ty, size)
        } else {
            self.symbols.global_variable(ty, size)
        };
        let symbol = symbol.map_err(|error| self.symbol_error(d, error))?;
        self.declare(d, name, symbol)?;
        Ok(())
    }

    fn type_of(&self, at: Id, name: u32) -> R<Ty> {
        match self.symbols.lookup_symbol(name) {
            None => Err(self.err(at, code::UNKNOWN_IDENTIFIER)),
            Some((_, symbol)) => match symbol.kind {
                SymKind::Type => Ok(symbol.ty),
                SymKind::Omitted => Err(self.err(at, code::NOT_SUPPORTED)),
                _ => Err(self.err(at, code::TYPE_IDENTIFIER_EXPECTED)),
            },
        }
    }

    // ---- Type specifications ----

    /// The type a declaration `name = spec` defines; `type T = T` is error 21.
    fn spec(&mut self, spec: Id, name: u32) -> R<Ty> {
        match self.ast.kind(spec) {
            Kind::Name(n) if n == name => Err(self.err(spec, code::ERROR_IN_TYPE)),
            Kind::Name(n) => self.type_of(spec, n),
            Kind::Enum(names) => self.enum_type(names),
            Kind::PtrType(target) => Ok(self.pointer_type(spec, target)),
            Kind::StrType(size) => self.string_type(spec, size),
            Kind::Array(indexes, element) => self.array_type(spec, indexes, element),
            Kind::Record(items) => self.record_type(spec, items),
            Kind::SetType(of) => {
                let base = self.spec(of, NO_NAME)?;
                self.types.set_of(base).map_err(|code| self.err(of, code))
            }
            Kind::Range(lo, hi) => self.subrange_type(spec, lo, hi),
            Kind::ProcType { params, result, .. } => self.proc_type(spec, params, result),
            _ => unreachable!("the parser builds type specs"),
        }
    }

    /// Each member of an enumeration becomes a constant of the new type.
    fn enum_type(&mut self, names: List) -> R<Ty> {
        let members = self.ast.items_vec(names);
        let ty = self.types.enumeration(members.len() as i64);
        for (ordinal, id) in members.into_iter().enumerate() {
            let name = self.ast.name_of(id);
            let value = SymKind::Const(ConstValue::Int(ordinal as i64));
            self.declare(id, name, Symbol::new(value, ty))?;
        }
        Ok(ty)
    }

    /// `^name`; a target not declared yet is resolved where the type section ends.
    fn pointer_type(&mut self, spec: Id, target: u32) -> Ty {
        let to = self.type_of(spec, target).unwrap_or(Ty::NONE);
        let pointer = self.types.pointer(to);
        if to == Ty::NONE {
            self.forward.push((pointer, target));
        }
        pointer
    }

    fn string_type(&mut self, spec: Id, size: Id) -> R<Ty> {
        self.expr(size)?;
        let Kind::IntLit(n) = self.ast.kind(size) else {
            return Err(self.err(spec, code::CANNOT_EVALUATE));
        };
        if !(1..=255).contains(&n) {
            return Err(self.err(spec, code::INVALID_STRING_LENGTH));
        }
        Ok(self.types.string(n as u8))
    }

    /// `array[i, j] of e` is an array of arrays: the innermost dimension is built first.
    fn array_type(&mut self, spec: Id, indexes: List, element: Id) -> R<Ty> {
        let mut index_types = Vec::new();
        for index in self.ast.items_vec(indexes) {
            let ty = self.spec(index, NO_NAME)?;
            if self.types.tag(ty).is_none() {
                return Err(self.err(index, code::ORDINAL_TYPE_EXPECTED));
            }
            index_types.push(ty);
        }
        let mut ty = self.spec(element, NO_NAME)?;
        for &index in index_types.iter().rev() {
            ty = self.types.array(index, ty);
            if self.types.size(ty) > MAX_DATA {
                return Err(self.err(spec, code::STRUCTURE_TOO_LARGE));
            }
        }
        Ok(ty)
    }

    fn record_type(&mut self, spec: Id, items: List) -> R<Ty> {
        let mut fields = Vec::new();
        let size = self.fields(items, 0, &mut fields)?;
        if size > MAX_DATA {
            return Err(self.err(spec, code::STRUCTURE_TOO_LARGE));
        }
        Ok(self.types.record(fields, size))
    }

    /// Lays out the fields `items` from offset `at`, appending them to `out`; returns the end
    /// offset. The arms of a variant part all start at the end of the fixed part.
    fn fields(&mut self, items: List, at: u32, out: &mut Vec<Field>) -> R<u32> {
        let mut end = at;
        for item in self.ast.items_vec(items) {
            match self.ast.kind(item) {
                Kind::FieldDecl(name, spec) => {
                    let ty = self.spec(spec, NO_NAME)?;
                    // An untagged variant's selector type has no name and takes no space.
                    if name != NO_NAME {
                        if out.iter().any(|f| f.name == name) {
                            return Err(self.err(item, code::DUPLICATE_IDENTIFIER));
                        }
                        out.push(Field {
                            name,
                            ty,
                            offset: end,
                        });
                        end += self.types.size(ty);
                    }
                }
                Kind::Variant(arms) => {
                    let start = end;
                    for arm in self.ast.items_vec(arms) {
                        let Kind::Record(list) = self.ast.kind(arm) else {
                            unreachable!("the parser lists record arms")
                        };
                        end = end.max(self.fields(list, start, out)?);
                    }
                }
                _ => unreachable!("the parser lists fields"),
            }
        }
        Ok(end)
    }

    /// `lo..hi` of integer, Char or Boolean constants.
    fn subrange_type(&mut self, spec: Id, lo: Id, hi: Id) -> R<Ty> {
        let (low, low_ty) = self.range_end(spec, lo)?;
        let (high, high_ty) = self.range_end(spec, hi)?;
        if !self.types.compat(low_ty, high_ty) {
            return Err(self.err(spec, code::TYPE_MISMATCH));
        }
        if low > high {
            return Err(self.err(spec, code::LOWER_BOUND_GREATER));
        }
        Ok(self.types.subrange(low, high, low_ty))
    }

    fn range_end(&mut self, spec: Id, end: Id) -> R<(i64, Ty)> {
        self.expr(end)?;
        let node = self.ast.node(end);
        match node.kind {
            Kind::IntLit(value) => Ok((value, node.ty)),
            _ => Err(self.err(spec, code::CANNOT_EVALUATE)),
        }
    }

    /// `procedure (params)` or `function (params): result`.
    fn proc_type(&mut self, spec: Id, params: Option<List>, result: u32) -> R<Ty> {
        let mut signature = Vec::new();
        for param in self.param_decls(params) {
            let ty = self.type_of(param.node, param.type_name)?;
            signature.push(ParamType {
                ty,
                by_ref: param.by_ref,
            });
        }
        let result = if result == NO_NAME {
            Ty::NONE
        } else {
            self.type_of(spec, result)?
        };
        if self.types.is_str(result) || result == Ty::REAL {
            return Err(self.err(spec, code::NOT_SUPPORTED));
        }
        Ok(self.types.proc(signature, result))
    }

    fn param_decls(&self, params: Option<List>) -> Vec<ParamDecl> {
        let nodes = params.map_or(Vec::new(), |list| self.ast.items_vec(list));
        let decl = |node: Id| {
            let Kind::Param {
                name,
                type_name,
                by_ref,
            } = self.ast.kind(node)
            else {
                unreachable!("the parser lists parameters")
            };
            ParamDecl {
                node,
                name,
                type_name,
                by_ref,
            }
        };
        nodes.into_iter().map(decl).collect()
    }

    // ---- Routines ----

    /// A procedure or function declaration, with its body when it is not `forward`.
    fn routine(&mut self, d: Id) -> R<()> {
        let Kind::Routine {
            name,
            params,
            result,
            is_function,
            block,
            external,
        } = self.ast.kind(d)
        else {
            unreachable!("routine declaration")
        };
        let signature = self.signature_of(d, params, result, is_function)?;
        if external.is_some() {
            self.check_import_signature(d, &signature)?;
        }
        let index = self.declare_routine(d, name, params.is_some(), signature, block)?;
        if external.is_some() {
            // The host provides the body.
            self.routines[index].import = external;
            self.routines[index].defined = true;
        }
        match block {
            Some(block) => self.routine_body(index, block),
            None => Ok(()),
        }
    }

    /// A host import takes only plain scalars by value (a Real is an f64, the rest i32) and
    /// returns one i32 or nothing; it must be a top-level routine, which has no static link.
    fn check_import_signature(&self, d: Id, signature: &Signature) -> R<()> {
        let passed_by_value =
            |p: &ParamType| !p.by_ref && !self.types.is_agg(p.ty) && !self.types.is_str(p.ty);
        let nested = self.symbols.is_nested();
        let result_is_i32 = !self.types.in_memory(signature.result);
        if nested || !result_is_i32 || !signature.params.iter().all(passed_by_value) {
            return Err(self.err(d, code::NOT_SUPPORTED));
        }
        Ok(())
    }

    /// The parameters and result a declaration states, with their types resolved.
    fn signature_of(
        &mut self,
        d: Id,
        params: Option<List>,
        result: u32,
        is_function: bool,
    ) -> R<Signature> {
        let mut types = Vec::new();
        let mut names = Vec::new();
        for param in self.param_decls(params) {
            let ty = self.type_of(param.node, param.type_name)?;
            if self.types.is_file(ty) && !param.by_ref {
                return Err(self.err(param.node, code::FILE_MUST_BE_VAR));
            }
            types.push(ParamType {
                ty,
                by_ref: param.by_ref,
            });
            names.push(param.name);
        }
        let mut result_ty = Ty::NONE;
        if result != NO_NAME {
            result_ty = self.type_of(d, result)?;
            if self.types.is_agg(result_ty) {
                return Err(self.err(d, code::INVALID_RESULT_TYPE));
            }
        }
        Ok(Signature {
            params: types,
            names,
            result: result_ty,
            is_function,
        })
    }

    /// Enters the routine in the routine table, or matches a definition with its earlier
    /// `forward` declaration. Returns the routine's index.
    fn declare_routine(
        &mut self,
        d: Id,
        name: u32,
        has_param_list: bool,
        signature: Signature,
        block: Option<Id>,
    ) -> R<usize> {
        match self.symbols.lookup_in_region(name).map(|s| s.kind) {
            None => {
                if signature.is_function && signature.result == Ty::NONE {
                    return Err(self.err(d, code::COLON_EXPECTED));
                }
                let depth = self.symbols.depth() + 1;
                let result_ptr = self.types.in_memory(signature.result);
                // The runtime sources (System, Crt, Dos) have every token at line 0.
                let origin = if self.symbols.in_system_region() {
                    Origin::System
                } else if self.ast.node(d).line == 0 {
                    Origin::Unit
                } else {
                    Origin::User
                };
                self.routines
                    .push(Routine::new(signature, depth, result_ptr, origin));
                if let Some(parent) = self.symbols.current_routine() {
                    self.routines[parent].has_children = true;
                }
                let index = self.routines.len() - 1;
                let kind = SymKind::Routine(index as u32);
                self.declare(d, name, Symbol::new(kind, Ty::NONE))?;
                Ok(index)
            }
            Some(SymKind::Routine(index)) => {
                let index = index as usize;
                self.match_forward(d, index, has_param_list, signature, block)?;
                Ok(index)
            }
            Some(_) => Err(self.err(d, code::DUPLICATE_IDENTIFIER)),
        }
    }

    /// A definition of a routine declared `forward` must come once, with a body, and repeat
    /// the parameters and result exactly (or leave them out).
    fn match_forward(
        &mut self,
        d: Id,
        index: usize,
        has_param_list: bool,
        signature: Signature,
        block: Option<Id>,
    ) -> R<()> {
        let routine = &self.routines[index];
        if routine.defined || routine.is_function != signature.is_function {
            return Err(self.err(d, code::DUPLICATE_IDENTIFIER));
        }
        if block.is_none() {
            return Err(self.err(d, code::BEGIN_EXPECTED));
        }
        let params_differ = has_param_list && signature.params != routine.params;
        let result_differs = signature.result != Ty::NONE && signature.result != routine.result;
        if params_differ || result_differs {
            return Err(self.err(d, code::HEADER_MISMATCH));
        }
        if has_param_list {
            self.routines[index].names = signature.names;
        }
        Ok(())
    }

    /// Checks the body of routine `r`: its parameters and result become locals (hidden
    /// parameters first), then its declarations and statements are checked, and finally
    /// `place_frame` decides which locals move to the frame.
    fn routine_body(&mut self, r: usize, block: Id) -> R<()> {
        let Kind::Block(declarations, body) = self.ast.kind(block) else {
            unreachable!("the parser makes a block")
        };
        self.routines[r].defined = true;
        let first_symbol = self.symbols.symbol_count();
        let first_param = self.routines[r].hidden_params();
        let declared_params = first_param + self.routines[r].params.len() as u32;
        let saved = self.symbols.enter_routine(r, declared_params);
        if self.routines[r].is_function {
            self.routines[r].result_sym = self.symbols.declare_result(&self.routines[r]);
        }
        self.declare_params(r, block, first_param)?;
        for decl in self.ast.items_vec(declarations) {
            self.decl(decl)?;
        }
        self.resolve_forward(body)?;
        if self
            .symbols
            .declared_since(&saved)
            .any(|s| self.is_undefined(s))
        {
            return Err(self.err(body, code::UNDEFINED_FORWARD));
        }
        self.stmt(body)?;
        self.routines[r].body = body;
        self.routines[r].nlocals = self.symbols.next_local() - declared_params;
        self.symbols
            .place_frame(&mut self.routines[r], first_symbol, &self.types);
        self.symbols.leave_routine(saved);
        Ok(())
    }

    /// Declares the parameters as locals, the first one being local `first_param`.
    fn declare_params(&mut self, r: usize, block: Id, first_param: u32) -> R<()> {
        let params = self.routines[r].params.clone();
        let names = self.routines[r].names.clone();
        for (i, (&name, param)) in names.iter().zip(&params).enumerate() {
            let index = first_param + i as u32;
            let symbol = Symbols::parameter(&self.types, index, param.ty, param.by_ref);
            self.declare(block, name, symbol)?;
        }
        Ok(())
    }

    // ---- Variable and procedural arguments ----

    /// TP's error for an argument of a `var` parameter that is not a variable, decided by its
    /// leftmost token; None for a variable.
    fn var_arg_error(&self, e: Id) -> Option<u16> {
        match self.ast.kind(e) {
            Kind::Binary { left, .. } => Some(
                self.var_arg_error(left)
                    .unwrap_or(code::RIGHT_PAREN_EXPECTED),
            ),
            Kind::Call(callee, args) => match self.ast.kind(callee) {
                // A type cast of a variable is a variable.
                Kind::Name(n) if self.type_of(callee, n).is_ok() => {
                    match self.ast.items(args).first() {
                        Some(&arg) => self.var_arg_error(arg),
                        None => Some(code::VARIABLE_EXPECTED),
                    }
                }
                Kind::Name(name) => Some(self.function_reference_error(name, true)),
                _ => Some(code::VARIABLE_EXPECTED),
            },
            Kind::Deref(_) | Kind::Var(_) => None,
            Kind::Index { base, .. } | Kind::Field(base, _) => self.var_arg_error(base),
            Kind::Name(name) if self.with_field(name).is_some() => None,
            Kind::Name(name) => match self.symbols.lookup_symbol(name) {
                None => Some(code::UNKNOWN_IDENTIFIER),
                Some((_, s)) if s.kind == SymKind::Omitted => Some(code::NOT_SUPPORTED),
                Some((_, s)) if s.kind.is_variable() => None,
                Some(_) => Some(self.function_reference_error(name, false)),
            },
            _ => Some(code::VARIABLE_EXPECTED),
        }
    }

    /// The error for a function name used where a variable is wanted.
    fn function_reference_error(&self, name: u32, is_call: bool) -> u16 {
        match self.symbols.lookup_symbol(name).map(|(_, s)| s.kind) {
            Some(SymKind::Routine(r)) if self.routines[r as usize].is_function => {
                let takes_arguments = !self.routines[r as usize].params.is_empty();
                if is_call || !takes_arguments {
                    code::INVALID_VARIABLE_REFERENCE
                } else {
                    code::LEFT_PAREN_EXPECTED
                }
            }
            Some(SymKind::Intrinsic(i)) if i.is_function() => code::INVALID_VARIABLE_REFERENCE,
            _ => code::VARIABLE_EXPECTED,
        }
    }

    /// An argument of a `var` parameter of type `ty`: a variable of that very type. `at` is
    /// the call.
    fn var_arg(&mut self, at: Id, arg: Id, ty: Ty) -> R<()> {
        if let Some(error) = self.var_arg_error(arg) {
            return Err(self.err(arg, error));
        }
        match self.ast.kind(arg) {
            Kind::Name(name) if self.with_field(name).is_none() => {
                let (sym, symbol) = self.resolve(arg, name)?;
                if symbol.ty != ty {
                    return Err(self.var_mismatch(at, ty));
                }
                self.symbols.capture(sym, true);
                self.ast.resolve(arg, Kind::Var(sym), ty);
            }
            kind => {
                // A field, element or dereference is checked as an expression. An argument the
                // checker made itself (`Input`) is already resolved.
                if !matches!(kind, Kind::Var(_)) {
                    self.expr(arg)?;
                }
                if self.ast.ty(arg) != ty {
                    return Err(self.var_mismatch(at, ty));
                }
            }
        }
        Ok(())
    }

    /// The routine's signature as a procedural type.
    fn proc_type_of(&mut self, r: usize) -> Ty {
        let params = self.routines[r].params.clone();
        let result = self.routines[r].result;
        self.types.proc(params, result)
    }

    /// `e` where a value of procedural type `want` is expected (assignment source, value
    /// argument): a global routine of that signature or a variable of that type. `at` is the
    /// statement or call.
    fn proc_value(&mut self, at: Id, want: Ty, e: Id) -> R<()> {
        let Kind::Name(name) = self.ast.kind(e) else {
            return Err(self.err(e, code::VARIABLE_EXPECTED));
        };
        let (sym, symbol) = self.resolve(e, name)?;
        let kind = match symbol.kind {
            SymKind::Routine(r) => {
                let r = r as usize;
                if self.routines[r].depth > 1 {
                    return Err(self.err(at, code::INVALID_PROCEDURE_REFERENCE));
                }
                if self.proc_type_of(r) != want {
                    return Err(self.err(at, code::TYPE_MISMATCH));
                }
                self.routines[r].addressed = true;
                Kind::ProcRef(r as u32)
            }
            kind if kind.is_variable() => {
                if symbol.ty != want {
                    return Err(self.err(at, code::TYPE_MISMATCH));
                }
                self.symbols.capture(sym, false);
                Kind::Var(sym)
            }
            _ => return Err(self.err(e, code::VARIABLE_EXPECTED)),
        };
        self.ast.resolve(e, kind, want);
        Ok(())
    }

    /// The field `name` of the innermost enclosing `with` record that has one.
    fn with_field(&self, name: u32) -> Option<WithField> {
        let found = |&(record, record_ty): &(Id, Ty)| {
            let (ty, offset) = self.types.field(record_ty, name)?;
            Some(WithField { record, ty, offset })
        };
        self.withs.iter().rev().find_map(found)
    }

    /// The symbol a name refers to; a predefined type this compiler omits is error 200.
    fn resolve(&self, id: Id, name: u32) -> R<(Sym, Symbol)> {
        let (sym, symbol) = self
            .symbols
            .lookup_symbol(name)
            .ok_or(self.err(id, code::UNKNOWN_IDENTIFIER))?;
        match symbol.kind {
            SymKind::Omitted => Err(self.err(id, code::NOT_SUPPORTED)),
            _ => Ok((sym, symbol)),
        }
    }

    // ---- Statements ----

    fn stmt(&mut self, id: Id) -> R<()> {
        match self.ast.kind(id) {
            Kind::Compound(statements) => self.statements(statements)?,
            Kind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.condition(id, condition)?;
                self.stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.stmt(else_branch)?;
                }
            }
            Kind::While(condition, body) => {
                self.condition(id, condition)?;
                self.stmt(body)?;
            }
            Kind::Repeat(body, condition) => {
                self.statements(body)?;
                self.condition(id, condition)?;
            }
            Kind::Case {
                selector,
                arms,
                else_body,
            } => self.case_statement(id, selector, arms, else_body)?,
            Kind::For {
                variable,
                first,
                last,
                body,
                ..
            } => self.for_statement(id, variable, first, last, body)?,
            Kind::Assign(target, value) => self.assign_statement(id, target, value)?,
            Kind::With(records, body) => self.with_statement(records, body)?,
            Kind::Call(callee, args) => {
                self.within_call(|cx| cx.call_statement(id, callee, args))?
            }
            _ => unreachable!("statements only"),
        }
        Ok(())
    }

    fn statements(&mut self, list: List) -> R<()> {
        for statement in self.ast.items_vec(list) {
            self.stmt(statement)?;
        }
        Ok(())
    }

    /// `at` is the statement owning the Boolean condition.
    fn condition(&mut self, at: Id, condition: Id) -> R<()> {
        self.expr(condition)?;
        match self.ast.ty(condition) {
            Ty::BOOLEAN => Ok(()),
            _ => Err(self.err(at, code::BOOLEAN_EXPRESSION_EXPECTED)),
        }
    }

    fn case_statement(&mut self, id: Id, selector: Id, arms: List, else_body: List) -> R<()> {
        self.expr(selector)?;
        let selector_ty = self.ast.ty(selector);
        if self.types.tag(selector_ty).is_none() {
            return Err(self.err(id, code::ORDINAL_EXPRESSION_EXPECTED));
        }
        for arm in self.ast.items_vec(arms) {
            let Kind::Arm(labels, body) = self.ast.kind(arm) else {
                unreachable!("the parser lists arms")
            };
            for label in self.ast.items_vec(labels) {
                self.label(label, selector_ty)?;
            }
            self.stmt(body)?;
        }
        self.statements(else_body)
    }

    /// A case label range: constants of the selector's family, within Integer.
    fn label(&mut self, label: Id, selector_ty: Ty) -> R<()> {
        let Kind::Range(lo, hi) = self.ast.kind(label) else {
            unreachable!("the parser lists ranges")
        };
        // A single label has both ends the same node.
        let ends = if lo == hi { vec![lo] } else { vec![lo, hi] };
        for end in ends {
            self.expr(end)?;
            let node = self.ast.node(end);
            let Kind::IntLit(value) = node.kind else {
                return Err(self.err(label, code::CANNOT_EVALUATE));
            };
            if !self.types.compat(selector_ty, node.ty) {
                return Err(self.err(label, code::CASE_TYPES_MISMATCH));
            }
            let is_integer = self.types.tag(selector_ty) == Some(Tag::Int);
            if is_integer && !(-32768..=32767).contains(&value) {
                return Err(self.err(label, code::CASE_CONSTANT_OUT_OF_RANGE));
            }
        }
        Ok(())
    }

    /// The control variable must be a local of this routine or a global of ordinal type.
    fn for_statement(&mut self, id: Id, variable: Id, first: Id, last: Id, body: Id) -> R<()> {
        let (sym, symbol) = self.resolve(variable, self.ast.name_of(variable))?;
        let is_outer = symbol.depth < self.symbols.depth();
        let is_reference = matches!(symbol.kind, SymKind::Ref(_));
        let is_outer_local = matches!(symbol.kind, SymKind::Local(_)) && is_outer;
        if is_reference
            || is_outer_local
            || self.types.sig(symbol.ty).is_some()
            || self.types.tag(symbol.ty).is_none()
        {
            return Err(self.err(variable, code::INVALID_FOR_VARIABLE));
        }
        if !matches!(symbol.kind, SymKind::Var(_) | SymKind::Local(_)) {
            return Err(self.err(variable, code::VARIABLE_EXPECTED));
        }
        self.ast.resolve(variable, Kind::Var(sym), symbol.ty);
        for bound in [first, last] {
            self.expr(bound)?;
            self.assignable(id, symbol.ty, bound)?;
        }
        self.stmt(body)
    }

    fn assign_statement(&mut self, id: Id, target: Id, value: Id) -> R<()> {
        let name = match self.ast.kind(target) {
            Kind::Name(name) if self.with_field(name).is_none() => name,
            _ => {
                // A selector chain, or a field named through `with`.
                self.resolve_result_indexed(target);
                self.expr(target)?;
                let ty = self.ast.ty(target);
                return self.assign_value(id, ty, value);
            }
        };
        let (sym, symbol) = self.resolve(target, name)?;
        let (kind, ty) = match symbol.kind {
            kind if kind.is_variable() => {
                self.symbols.capture(sym, false);
                (Kind::Var(sym), symbol.ty)
            }
            // Assigning to the name of the function being defined sets its result.
            SymKind::Routine(r) if self.is_enclosing_function(r) => self.result_variable(r),
            SymKind::Routine(r) if self.routines[r as usize].is_function => {
                return Err(self.err(target, code::INVALID_VARIABLE_REFERENCE));
            }
            SymKind::Routine(_) => return Err(self.err(target, code::SEMICOLON_EXPECTED)),
            // `Random` is a function without parameters, like a user's.
            SymKind::Intrinsic(Intrinsic::Random) => {
                return Err(self.err(target, code::INVALID_VARIABLE_REFERENCE));
            }
            SymKind::Intrinsic(i) if i.is_function() => {
                return Err(self.err(target, code::LEFT_PAREN_EXPECTED));
            }
            _ => return Err(self.err(target, code::ERROR_IN_STATEMENT)),
        };
        self.ast.resolve(target, kind, ty);
        self.assign_value(id, ty, value)
    }

    /// The result variable of the function `r` being defined, with its type.
    fn result_variable(&mut self, r: u32) -> (Kind, Ty) {
        let result = self.routines[r as usize].result_sym;
        self.symbols.capture(result, false);
        (Kind::Var(result), self.routines[r as usize].result)
    }

    /// In `Name[i] := v` with `Name` the function being defined, `Name` is the result variable
    /// (a string whose elements the body may set), not a recursive call. The base is resolved
    /// here; reads of `Name[i]` stay calls, as in Turbo Pascal.
    fn resolve_result_indexed(&mut self, target: Id) {
        let Kind::Index { base, .. } = self.ast.kind(target) else {
            return;
        };
        let Kind::Name(name) = self.ast.kind(base) else {
            return;
        };
        if self.with_field(name).is_some() {
            return;
        }
        if let Ok((_, symbol)) = self.resolve(base, name) {
            if let SymKind::Routine(r) = symbol.kind {
                if self.is_enclosing_function(r) {
                    let (kind, ty) = self.result_variable(r);
                    self.ast.resolve(base, kind, ty);
                }
            }
        }
    }

    /// True if routine `r` is a function being defined now (this one or one around it).
    fn is_enclosing_function(&self, r: u32) -> bool {
        self.symbols.is_being_checked(r as usize) && self.routines[r as usize].is_function
    }

    /// `at` is the assignment of `value` to a target of type `ty`.
    fn assign_value(&mut self, at: Id, ty: Ty, value: Id) -> R<()> {
        if self.types.sig(ty).is_some() {
            return self.proc_value(at, ty, value);
        }
        self.expr(value)?;
        self.assignable(at, ty, value)
    }

    fn with_statement(&mut self, records: List, body: Id) -> R<()> {
        let outer = self.withs.len();
        for designator in self.ast.items_vec(records) {
            self.expr(designator)?;
            let ty = self.ast.ty(designator);
            if !self.types.is_record(ty) {
                return Err(self.err(designator, code::RECORD_VARIABLE_EXPECTED));
            }
            self.withs.push((designator, ty));
        }
        self.stmt(body)?;
        self.withs.truncate(outer);
        Ok(())
    }

    /// A procedure call statement, or an intrinsic that is a statement. The call node is
    /// rewritten to what codegen expects: `Invoke`, `Indirect`, `Write`, `Exit`, or the
    /// assignment of a helper's result (see the module comment).
    fn call_statement(&mut self, id: Id, callee: Id, args: List) -> R<()> {
        if self.is_designator(callee) {
            return self.designator_call_statement(id, callee, args);
        }
        let (_, symbol) = self.resolve(callee, self.ast.name_of(callee))?;
        let kind = match symbol.kind {
            SymKind::Intrinsic(i @ (Intrinsic::Write | Intrinsic::WriteLn)) => {
                self.write_call(id, args, i == Intrinsic::WriteLn)?
            }
            SymKind::Intrinsic(i @ (Intrinsic::Read | Intrinsic::ReadLn)) => {
                self.read_call(id, args, i == Intrinsic::ReadLn)?
            }
            SymKind::Intrinsic(i @ (Intrinsic::Delete | Intrinsic::Insert)) => {
                self.edit_string(id, i, args)?
            }
            SymKind::Intrinsic(
                i @ (Intrinsic::New
                | Intrinsic::Dispose
                | Intrinsic::GetMem
                | Intrinsic::FreeMem
                | Intrinsic::Mark
                | Intrinsic::Release
                | Intrinsic::RunError),
            ) => self.heap_call(id, i, args)?,
            SymKind::Intrinsic(i @ (Intrinsic::Inc | Intrinsic::Dec)) => {
                self.inc_dec(id, i, args)?
            }
            SymKind::Intrinsic(Intrinsic::Halt) => self.halt_call(id, args)?,
            SymKind::Intrinsic(Intrinsic::FillChar) => self.fill_char(id, args)?,
            SymKind::Intrinsic(Intrinsic::Move) => self.move_bytes(id, args)?,
            SymKind::Intrinsic(Intrinsic::Str) => self.str_call(id, args)?,
            SymKind::Intrinsic(Intrinsic::Val) => self.val_call(id, args)?,
            SymKind::Intrinsic(Intrinsic::Exit) => {
                if !self.ast.items(args).is_empty() {
                    return Err(self.err(id, code::SEMICOLON_EXPECTED));
                }
                Kind::Exit
            }
            SymKind::Routine(r) if !self.routines[r as usize].is_function => {
                let params = self.routines[r as usize].params.clone();
                let origin = self.routines[r as usize].origin;
                self.arguments(id, params, args, false, origin)?;
                Kind::Invoke(r, args)
            }
            SymKind::Routine(_) => return Err(self.err(id, code::INVALID_VARIABLE_REFERENCE)),
            SymKind::Intrinsic(i) if i.is_function() => {
                return Err(self.err(id, code::INVALID_VARIABLE_REFERENCE));
            }
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                Some((params, Ty::NONE)) => self.indirect(id, callee, params, args, false)?,
                _ => return Err(self.err(id, code::ASSIGN_EXPECTED)),
            },
            _ => return Err(self.err(callee, code::ERROR_IN_STATEMENT)),
        };
        self.ast.set_kind(id, kind);
        Ok(())
    }

    /// Whether a callee is a value to evaluate rather than a routine to look up: anything but
    /// a plain name, and a plain name that is a field of an enclosing `with` record, which
    /// hides any routine of that name.
    fn is_designator(&self, callee: Id) -> bool {
        match self.ast.kind(callee) {
            Kind::Name(name) => self.with_field(name).is_some(),
            _ => true,
        }
    }

    /// `r.f(args);`, `a[i];`, `p^.f;`: a call through a procedure value held in a variable.
    /// Any other designator is no statement: TP wants `:=` after it.
    fn designator_call_statement(&mut self, id: Id, callee: Id, args: List) -> R<()> {
        self.expr(callee)?;
        let Some((params, Ty::NONE)) = self.types.sig(self.ast.ty(callee)) else {
            return Err(self.err(id, code::ASSIGN_EXPECTED));
        };
        self.arguments(id, params, args, false, Origin::User)?;
        self.ast.set_kind(id, Kind::Indirect(callee, args));
        Ok(())
    }

    // ---- Arguments ----

    /// Checks the arguments of a call against the parameters; `at` is the call. A routine
    /// without parameters takes no parentheses.
    fn arguments(
        &mut self,
        at: Id,
        params: Vec<ParamType>,
        args: List,
        in_expr: bool,
        origin: Origin,
    ) -> R<()> {
        let items = self.ast.items_vec(args);
        // The call is itself an argument of an enclosing call.
        let nested = in_expr && self.call_depth > 1;
        // TP checks the arguments that have a parameter before it counts them.
        for (&arg, param) in items.iter().zip(&params) {
            if matches!(self.ast.kind(arg), Kind::Fmt { .. }) {
                return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
            }
            if param.by_ref {
                self.var_arg(at, arg, param.ty)?;
            } else if self.types.sig(param.ty).is_some() {
                self.proc_value(at, param.ty, arg)?;
            } else {
                self.expr(arg)?;
                self.argument_assignable(at, param.ty, arg, origin)?;
            }
        }
        match argument_count_error(params.len(), items.len(), nested) {
            Some(code::LEFT_PAREN_EXPECTED) => Err(self.missing_argument_list(at)),
            Some(error) => Err(self.err(at, error)),
            None => Ok(()),
        }
    }

    /// Runs `check` on a call: its arguments are checked one call deeper.
    fn within_call<T>(&mut self, check: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        self.call_depth += 1;
        let checked = check(self);
        self.call_depth -= 1;
        checked
    }

    /// TP reports a missing argument list at the line of the token after the name.
    fn missing_argument_list(&self, at: Id) -> Diag {
        let follow = self.ast.follow.iter().find(|f| f.node == at);
        let line = follow.map_or(self.ast.node(at).line, |f| f.line);
        Diag {
            line,
            ..self.err(at, code::LEFT_PAREN_EXPECTED)
        }
    }

    /// `assignable` for an argument; TP names the kind of value that a System routine wants.
    fn argument_assignable(&mut self, at: Id, ty: Ty, arg: Id, origin: Origin) -> R<()> {
        self.assignable(at, ty, arg).map_err(|d| {
            let code = if d.code != code::TYPE_MISMATCH || origin != Origin::System {
                d.code
            } else if self.types.is_str(ty) {
                code::STRING_EXPRESSION_EXPECTED
            } else if self.types.tag(ty) == Some(Tag::Char) {
                code::CHAR_EXPRESSION_EXPECTED
            } else if self.types.tag(ty) == Some(Tag::Int) {
                code::INTEGER_EXPRESSION_EXPECTED
            } else {
                code::TYPE_MISMATCH
            };
            d.with_code(code)
        })
    }

    /// A call through the variable named by `callee` with the signature `params`: the
    /// variable becomes the callee value of the call node `at`.
    fn indirect(
        &mut self,
        at: Id,
        callee: Id,
        params: Vec<ParamType>,
        args: List,
        in_expr: bool,
    ) -> R<Kind> {
        let (sym, symbol) = self.resolve(callee, self.ast.name_of(callee))?;
        self.arguments(at, params, args, in_expr, Origin::User)?;
        self.symbols.capture(sym, false);
        self.ast.resolve(callee, Kind::Var(sym), symbol.ty);
        Ok(Kind::Indirect(callee, args))
    }

    /// `at` is the assignment; a constant must also fit the destination's range.
    fn assignable(&mut self, at: Id, dest: Ty, value: Id) -> R<()> {
        let node = self.ast.node(value);
        if self.types.is_str(dest) {
            if !self.types.is_str(node.ty) && self.types.tag(node.ty) != Some(Tag::Char) {
                return Err(self.err(at, code::TYPE_MISMATCH));
            }
            self.coerce_str(value);
            return Ok(());
        }
        if dest == Ty::REAL {
            self.coerce_real(value);
            if self.ast.ty(value) != Ty::REAL {
                return Err(self.err(at, code::TYPE_MISMATCH));
            }
            return Ok(());
        }
        if self.types.is_agg(dest) {
            if self.types.is_file(dest) {
                return Err(self.err(at, code::ILLEGAL_ASSIGNMENT));
            }
            // Name equivalence: only the very same type; sets need compatible bases.
            let compatible_sets = self.types.is_set(dest) && self.types.compat(dest, node.ty);
            if node.ty == dest || compatible_sets {
                return Ok(());
            }
            return Err(self.err(at, code::TYPE_MISMATCH));
        }
        if !self.types.compat(dest, node.ty) {
            return Err(self.err(at, code::TYPE_MISMATCH));
        }
        if let Kind::IntLit(constant) = node.kind {
            let range = self.types.ord(dest);
            if constant < range.lo || constant > range.hi {
                return Err(self.err(at, code::CONSTANT_OUT_OF_RANGE));
            }
        }
        Ok(())
    }

    /// An Integer-family operand `e` used as a Real: a literal becomes a real literal, else an
    /// `IntToReal` node over a copy of `e`. The copy takes a new slot so that `e`, which the
    /// parent refers to, can be overwritten by the wrapper.
    fn coerce_real(&mut self, e: Id) {
        let node = self.ast.node(e);
        if self.types.tag(node.ty) != Some(Tag::Int) {
            return;
        }
        let kind = if let Kind::IntLit(v) = node.kind {
            Kind::RealLit(v as f64)
        } else {
            self.ast.nodes.push(node);
            Kind::IntToReal(self.ast.nodes.len() as Id - 1)
        };
        self.ast.resolve(e, kind, Ty::REAL);
    }

    /// A Char operand `e` used as a String: a one-character literal, else a `CharToStr` node
    /// over a copy of `e` (see `coerce_real`).
    fn coerce_str(&mut self, e: Id) {
        let node = self.ast.node(e);
        if self.types.tag(node.ty) != Some(Tag::Char) {
            return;
        }
        let kind = if let Kind::IntLit(c) = node.kind {
            self.ast.strs.push(vec![c as u8]);
            Kind::StrLit(self.ast.strs.len() as u32 - 1)
        } else {
            self.ast.nodes.push(node);
            Kind::CharToStr(self.ast.nodes.len() as Id - 1)
        };
        self.ast.resolve(e, kind, Ty::STRING);
    }

    /// The types `Write` accepts: integers, Char, Boolean, Real, String and array of Char; not
    /// enumerations, sets, records or pointers.
    fn is_writable(&self, ty: Ty) -> bool {
        let is_text_or_real =
            self.types.is_str(ty) || self.types.is_char_array(ty) || ty == Ty::REAL;
        let is_enum = self.types.tag(ty) == Some(Tag::Enum);
        !is_enum && (is_text_or_real || self.types.tag(ty).is_some())
    }

    /// One argument of `Write`/`WriteLn`: `value`, or `value : width [: decimals]`. `checked`
    /// says that `console_or_file` already checked the expression.
    fn write_arg(&mut self, arg: Id, checked: bool) -> R<()> {
        let mut value = arg;
        let mut decimals = None;
        if let Kind::Fmt {
            value: v,
            width,
            decimals: d,
        } = self.ast.kind(arg)
        {
            value = v;
            decimals = d;
            for field in [Some(width), d].into_iter().flatten() {
                self.expr(field)?;
                if self.types.tag(self.ast.ty(field)) != Some(Tag::Int) {
                    return Err(self.err(arg, code::INTEGER_EXPRESSION_EXPECTED));
                }
            }
        }
        if !checked {
            self.expr(value)?;
        }
        let ty = self.ast.ty(value);
        if !self.is_writable(ty) {
            return Err(self.err(value, code::CANNOT_READ_OR_WRITE));
        }
        if decimals.is_some() && ty != Ty::REAL {
            return Err(self.err(arg, code::RIGHT_PAREN_EXPECTED));
        }
        Ok(())
    }

    // ---- Expressions ----

    /// Checks the expression `e` and rewrites its node to the resolved kind and type.
    fn expr(&mut self, e: Id) -> R<()> {
        let (kind, ty) = self.expr_kind(e)?;
        self.ast.resolve(e, kind, ty);
        Ok(())
    }

    /// The resolved kind and the type of expression `e`.
    fn expr_kind(&mut self, e: Id) -> R<(Kind, Ty)> {
        let node = self.ast.node(e);
        Ok(match node.kind {
            Kind::IntLit(v) => (node.kind, self.types.const_ty(v)),
            Kind::RealLit(_) => (node.kind, Ty::REAL),
            Kind::StrLit(i) => match self.ast.strs[i as usize][..] {
                // A one-character string is a Char constant until used as a String.
                [c] => (Kind::IntLit(c as i64), Ty::CHAR),
                _ => (node.kind, Ty::STRING),
            },
            Kind::Name(name) => self.name_expr(e, name)?,
            // Already resolved: the result variable of the function being defined.
            Kind::Var(_) => (node.kind, node.ty),
            Kind::Index {
                base,
                index,
                continues,
            } => self.index_expr(e, base, index, continues)?,
            Kind::Field(base, name) => self.field_expr(e, base, name)?,
            Kind::Call(callee, args) => self.within_call(|cx| cx.call_expr(e, callee, args))?,
            Kind::AddrOf(operand) => {
                self.expr(operand)?;
                match self.ast.kind(operand) {
                    Kind::Var(sym) => self.symbols.capture(sym, true),
                    Kind::Index { .. } | Kind::Field(..) | Kind::Deref(_) => {}
                    _ => return Err(self.err(e, code::VARIABLE_EXPECTED)),
                }
                (node.kind, Ty::POINTER)
            }
            Kind::Deref(pointer) => {
                self.expr(pointer)?;
                match self.types.target(self.ast.ty(pointer)) {
                    Some(to) => (node.kind, to),
                    None => return Err(self.err(e, code::INVALID_QUALIFIER)),
                }
            }
            Kind::Paren(inner) => {
                self.expr(inner)?;
                let inner = self.ast.node(inner);
                (inner.kind, inner.ty)
            }
            Kind::Unary(op, operand) => self.unary(e, op, operand)?,
            Kind::SetLit(items) => self.set_literal(node.kind, items)?,
            Kind::Range(lo, hi) => {
                self.expr(lo)?;
                self.expr(hi)?;
                let (lo_ty, hi_ty) = (self.ast.ty(lo), self.ast.ty(hi));
                if !self.types.compat(lo_ty, hi_ty) {
                    return Err(self.err(hi, code::TYPE_MISMATCH));
                }
                (node.kind, lo_ty)
            }
            Kind::Binary { op, left, right } => self.binary_expr(e, op, left, right)?,
            _ => unreachable!("expressions only"),
        })
    }

    /// `base[index]`: the base is an array or a string, the index of the index type.
    fn index_expr(&mut self, e: Id, base: Id, index: Id, continues: bool) -> R<(Kind, Ty)> {
        self.expr(base)?;
        // Only a variable has elements to address; a constant string or a call result does not.
        match self.ast.kind(base) {
            Kind::Var(_) | Kind::Field(..) | Kind::Index { .. } | Kind::Deref(_) => {}
            Kind::StrLit(_) => return Err(self.err(e, code::RIGHT_PAREN_EXPECTED)),
            _ => return Err(self.err(e, code::INVALID_QUALIFIER)),
        }
        let Some(array) = self.types.array_of(self.ast.ty(base)) else {
            let error = if continues {
                code::RIGHT_BRACKET_EXPECTED
            } else {
                code::INVALID_QUALIFIER
            };
            return Err(self.err(e, error));
        };
        self.expr(index)?;
        let index_node = self.ast.node(index);
        if !self.types.compat(array.index, index_node.ty) {
            return Err(self.err(e, code::TYPE_MISMATCH));
        }
        if matches!(index_node.kind, Kind::IntLit(c) if c < array.lo || c > array.hi) {
            return Err(self.err(e, code::CONSTANT_OUT_OF_RANGE));
        }
        Ok((self.ast.kind(e), array.elem))
    }

    /// `base.name` becomes the field at its byte offset.
    fn field_expr(&mut self, e: Id, base: Id, name: u32) -> R<(Kind, Ty)> {
        self.expr(base)?;
        let base_ty = self.ast.ty(base);
        if !self.types.is_record(base_ty) || self.types.is_file(base_ty) {
            return Err(self.err(e, code::INVALID_QUALIFIER));
        }
        let Some((ty, offset)) = self.types.field(base_ty, name) else {
            return Err(self.err(e, code::FIELD_IDENTIFIER_EXPECTED));
        };
        Ok((Kind::Field(base, offset), ty))
    }

    /// A name used as an operand: a field of a `with` record, a constant, a variable (called
    /// when it is a function-valued procedural variable), or a function called without
    /// arguments.
    fn name_expr(&mut self, e: Id, name: u32) -> R<(Kind, Ty)> {
        if let Some(field) = self.with_field(name) {
            return Ok((Kind::Field(field.record, field.offset), field.ty));
        }
        let (sym, symbol) = self.resolve(e, name)?;
        Ok(match symbol.kind {
            SymKind::Const(ConstValue::Int(v)) => (Kind::IntLit(v), symbol.ty),
            SymKind::Const(ConstValue::Real(v)) => (Kind::RealLit(v), symbol.ty),
            SymKind::Const(ConstValue::Str(i)) => (Kind::StrLit(i), symbol.ty),
            SymKind::Const(ConstValue::Set(literal)) => (self.ast.kind(literal), symbol.ty),
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                // A procedural variable in an expression is called.
                Some((_, Ty::NONE)) => return Err(self.err(e, code::INVALID_PROCEDURE_REFERENCE)),
                Some((params, result)) => {
                    let (line, col) = self.ast.position(e);
                    let callee = self.ast.add(Kind::Name(name), line, col);
                    let no_args = self.ast.list(&[]);
                    (self.indirect(e, callee, params, no_args, true)?, result)
                }
                None => {
                    self.symbols.capture(sym, false);
                    (Kind::Var(sym), symbol.ty)
                }
            },
            SymKind::Routine(r) => {
                let no_args = self.ast.list(&[]);
                self.function_call(e, e, r, no_args)?
            }
            SymKind::Intrinsic(Intrinsic::Random) => {
                let no_args = self.ast.list(&[]);
                self.random_call(e, no_args)?
            }
            SymKind::Intrinsic(i) if i.is_function() => {
                return Err(self.err(e, code::LEFT_PAREN_EXPECTED));
            }
            _ => return Err(self.err(e, code::ERROR_IN_EXPRESSION)),
        })
    }

    /// A call of routine `r` in an expression: it must be a function. `not_function_at` is
    /// the node TP reports a procedure used as a value at.
    fn function_call(&mut self, e: Id, not_function_at: Id, r: u32, args: List) -> R<(Kind, Ty)> {
        let routine = &self.routines[r as usize];
        let (is_function, result, origin) = (routine.is_function, routine.result, routine.origin);
        if !is_function {
            // TP's own procedures are no part of an expression at all (error 42).
            let error = if origin == Origin::System {
                code::ERROR_IN_EXPRESSION
            } else {
                code::INVALID_PROCEDURE_REFERENCE
            };
            return Err(self.err(not_function_at, error));
        }
        let params = routine.params.clone();
        let args = self.default_input(e, r, args);
        self.arguments(e, params, args, true, origin)?;
        Ok((Kind::Invoke(r, args), result))
    }

    /// `name(args)` in an expression: a function, a procedural variable, an intrinsic, or a
    /// type cast.
    fn call_expr(&mut self, e: Id, callee: Id, args: List) -> R<(Kind, Ty)> {
        if self.is_designator(callee) {
            return self.designator_call(e, callee, args);
        }
        let (_, symbol) = self.resolve(callee, self.ast.name_of(callee))?;
        Ok(match symbol.kind {
            SymKind::Routine(r) => {
                let call = self.function_call(e, callee, r, args)?;
                self.folded_length(callee, r, args).unwrap_or(call)
            }
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                Some((_, Ty::NONE)) => {
                    return Err(self.err(callee, code::INVALID_PROCEDURE_REFERENCE))
                }
                Some((params, result)) => (self.indirect(e, callee, params, args, true)?, result),
                None => return Err(self.err(e, code::RIGHT_PAREN_EXPECTED)),
            },
            SymKind::Intrinsic(Intrinsic::SizeOf) => self.size_of(e, args)?,
            SymKind::Intrinsic(i) if i.is_real_function() => self.real_function(e, i, args)?,
            SymKind::Intrinsic(Intrinsic::Concat) => self.concat(e, args)?,
            SymKind::Intrinsic(Intrinsic::Random) => self.random_call(e, args)?,
            SymKind::Intrinsic(i) => match i.unary_op() {
                Some(op) => {
                    let items = self.ast.items_vec(args);
                    if items.len() > 1 {
                        return Err(self.err(e, code::RIGHT_PAREN_EXPECTED));
                    }
                    self.unary(e, op, items[0])?
                }
                None => return Err(self.err(callee, code::ERROR_IN_EXPRESSION)),
            },
            SymKind::Type => self.cast(e, symbol.ty, args)?,
            _ => return Err(self.err(e, code::RIGHT_PAREN_EXPECTED)),
        })
    }

    /// `r.f(args)`, `a[i](args)`: a call through a function value held in a variable.
    fn designator_call(&mut self, e: Id, callee: Id, args: List) -> R<(Kind, Ty)> {
        self.expr(callee)?;
        let Some((params, result)) = self.types.sig(self.ast.ty(callee)) else {
            return Err(self.err(e, code::RIGHT_PAREN_EXPECTED));
        };
        if result == Ty::NONE {
            return Err(self.err(callee, code::INVALID_PROCEDURE_REFERENCE));
        }
        self.arguments(e, params, args, true, Origin::User)?;
        Ok((Kind::Indirect(callee, args), result))
    }

    /// System's `Length` of a string constant is a constant. It is the only function that is
    /// folded, so it is recognised by its name.
    fn folded_length(&self, callee: Id, r: u32, args: List) -> Option<(Kind, Ty)> {
        let name = &self.ast.names[self.ast.name_of(callee) as usize];
        if self.routines[r as usize].origin != Origin::System || name != b"LENGTH" {
            return None;
        }
        let Kind::StrLit(s) = self.ast.kind(*self.ast.items(args).first()?) else {
            return None;
        };
        let length = self.ast.strs[s as usize].len() as i64;
        Some((Kind::IntLit(length), self.types.const_ty(length)))
    }

    /// `[a, b..c]`: the base type is the type of the elements, which must agree.
    fn set_literal(&mut self, kind: Kind, items: List) -> R<(Kind, Ty)> {
        let mut base = Ty::NONE;
        for item in self.ast.items_vec(items) {
            self.expr(item)?;
            let ty = self.ast.ty(item);
            base = self
                .types
                .element(base, ty)
                .map_err(|code| self.err(item, code))?;
        }
        Ok((kind, self.types.set(base)))
    }

    /// A binary operator: typed by `Types::binary`, operands converted to String or Real as
    /// the operator needs, and folded when both operands are constants.
    fn binary_expr(&mut self, e: Id, op: Op, left: Id, right: Id) -> R<(Kind, Ty)> {
        self.expr(left)?;
        self.expr(right)?;
        let (left_ty, right_ty) = (self.ast.ty(left), self.ast.ty(right));
        let ty = self
            .types
            .binary(op, left_ty, right_ty)
            .map_err(|code| self.err(e, code))?;
        if ty == Ty::STRING || self.types.is_str(left_ty) || self.types.is_str(right_ty) {
            self.coerce_str(left);
            self.coerce_str(right);
        }
        if op == Op::Slash || left_ty == Ty::REAL || right_ty == Ty::REAL {
            self.coerce_real(left);
            self.coerce_real(right);
        }
        match (self.ast.kind(left), self.ast.kind(right)) {
            (Kind::IntLit(_), Kind::IntLit(0)) if matches!(op, Op::Div | Op::Mod) => {
                Err(self.err(e, code::DIVISION_BY_ZERO))
            }
            (Kind::RealLit(x), Kind::RealLit(y))
                if ty == Ty::REAL && (op != Op::Slash || y != 0.0) =>
            {
                let value = match op {
                    Op::Add => x + y,
                    Op::Sub => x - y,
                    Op::Mul => x * y,
                    _ => x / y,
                };
                Ok((Kind::RealLit(value), ty))
            }
            (Kind::IntLit(x), Kind::IntLit(y)) if ty != Ty::STRING => {
                Ok(self.constant(fold(op, x, y), ty))
            }
            (Kind::StrLit(x), Kind::StrLit(y)) if op == Op::Add => {
                let mut joined = self.ast.strs[x as usize].clone();
                joined.extend_from_slice(&self.ast.strs[y as usize]);
                joined.truncate(255);
                self.ast.strs.push(joined);
                Ok((Kind::StrLit(self.ast.strs.len() as u32 - 1), ty))
            }
            _ => Ok((self.ast.kind(e), ty)),
        }
    }

    /// A unary operator or intrinsic applied to `a`, folded when `a` is constant (`Sqr` is
    /// not constant in TP).
    fn unary(&mut self, e: Id, op: Op, a: Id) -> R<(Kind, Ty)> {
        self.expr(a)?;
        let arg_ty = self.ast.ty(a);
        let mut ty = self
            .types
            .unary(op, arg_ty)
            .map_err(|code| self.err(e, code))?;
        if matches!(op, Op::Round | Op::Trunc | Op::Sqrt | Op::Int | Op::Frac) {
            self.coerce_real(a);
        }
        let operand = self.ast.node(a);
        if op == Op::Ord && self.types.tag(operand.ty) == Some(Tag::Enum) {
            // TP types Ord(enum) by the enumeration's range (`Ord(c) + w` is a Word sum).
            let range = self.types.ord(operand.ty);
            ty = self.types.subrange(range.lo, range.hi, Ty::INTEGER);
        }
        Ok(match operand.kind {
            Kind::RealLit(v) if op == Op::Neg => (Kind::RealLit(-v), ty),
            Kind::IntLit(v) if op == Op::Not && ty == Ty::BOOLEAN => self.constant(v ^ 1, ty),
            Kind::IntLit(v) if op != Op::Sqr => self.constant(fold(op, v, 0), ty),
            _ => (Kind::Unary(op, a), ty),
        })
    }

    /// A folded constant: an integer takes the smallest type, other ordinals keep theirs.
    fn constant(&self, v: i64, ty: Ty) -> (Kind, Ty) {
        let ty = if self.types.tag(ty) == Some(Tag::Int) {
            self.types.const_ty(v)
        } else {
            ty
        };
        (Kind::IntLit(v), ty)
    }

    /// `T(x)`: the ordinal value of `x` as type `ty`.
    fn cast(&mut self, at: Id, ty: Ty, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        };
        self.expr(arg)?;
        let arg_node = self.ast.node(arg);
        let is_scalar = |t: Ty| self.types.tag(t).is_some() || self.types.is_ptr(t);
        if !is_scalar(ty) || !is_scalar(arg_node.ty) {
            return Err(self.err(at, code::NOT_SUPPORTED));
        }
        Ok(match arg_node.kind {
            Kind::IntLit(v) => self.constant(v, ty),
            _ => (Kind::Unary(Op::Ord, arg), ty),
        })
    }

    /// `Concat(a, b, ...)`: the same as `a + b + ...`.
    fn concat(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let items = self.ast.items_vec(args);
        if items.is_empty() {
            return Err(self.err(at, code::ERROR_IN_EXPRESSION));
        }
        let (line, col) = self.ast.position(at);
        let mut sum = items[0];
        for (i, &item) in items.iter().enumerate() {
            self.expr(item)?;
            let ty = self.ast.ty(item);
            if !self.types.is_str(ty) && self.types.tag(ty) != Some(Tag::Char) {
                return Err(self.err(at, code::STRING_EXPRESSION_EXPECTED));
            }
            self.coerce_str(item);
            if i > 0 {
                let add = Kind::Binary {
                    op: Op::Add,
                    left: sum,
                    right: item,
                };
                sum = self.ast.add(add, line, col);
                self.ast.set_ty(sum, Ty::STRING);
            }
        }
        let node = self.ast.node(sum);
        Ok((node.kind, node.ty))
    }

    /// `SizeOf(type name or variable)`, a constant.
    fn size_of(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        };
        let ty = match self.ast.kind(arg) {
            Kind::Name(name) if self.with_field(name).is_none() => {
                let (_, symbol) = self.resolve(arg, name)?;
                if matches!(symbol.kind, SymKind::Const(_) | SymKind::Routine(_)) {
                    return Err(self.err(arg, code::VARIABLE_EXPECTED));
                }
                symbol.ty
            }
            _ => {
                self.expr(arg)?;
                self.ast.ty(arg)
            }
        };
        Ok(self.constant(self.types.size(ty) as i64, Ty::INTEGER))
    }

    // ---- Intrinsics rewritten into assignments of helper results ----

    /// Error 87 (too few) or 89 (too many) at `at` unless `min..=max` arguments were given.
    fn expect_arg_count(&self, at: Id, count: usize, min: usize, max: usize) -> R<()> {
        if count == 0 && min > 0 {
            return Err(self.err(at, code::LEFT_PAREN_EXPECTED));
        }
        if count < min {
            return Err(self.err(at, code::COMMA_EXPECTED));
        }
        if count > max {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        }
        Ok(())
    }

    /// A variable argument (TP's error otherwise) and its type.
    fn variable(&mut self, e: Id) -> R<Ty> {
        if let Some(error) = self.var_arg_error(e) {
            return Err(self.err(e, error));
        }
        self.expr(e)?;
        Ok(self.ast.ty(e))
    }

    /// The file of `Read` and `Write`: a leading variable of type Text, else the console. A
    /// write to the console passes the file address 0, a read the console file `Input`.
    fn console_or_file(&mut self, at: Id, items: &[Id], reading: bool) -> R<FileArg> {
        let first = items.first().copied();
        let variable = first.filter(|&e| self.var_arg_error(e).is_none());
        if let Some(first) = variable {
            self.expr(first)?;
            if self.types.is_file(self.ast.ty(first)) {
                return Ok(FileArg {
                    file: first,
                    first_item: 1,
                    first_item_checked: false,
                });
            }
        }
        let console = if reading {
            self.input_file(at)
        } else {
            self.ast.typed(Kind::IntLit(0), Ty::LONGINT, at)
        };
        Ok(FileArg {
            file: console,
            first_item: 0,
            first_item_checked: variable.is_some(),
        })
    }

    /// A reference to System's `Input`.
    fn input_file(&mut self, at: Id) -> Id {
        self.ast.typed(Kind::Var(self.input), self.types.text, at)
    }

    /// `Write(f, a, b)`: the file node followed by the checked items.
    fn write_call(&mut self, at: Id, args: List, newline: bool) -> R<Kind> {
        let items = self.ast.items_vec(args);
        let FileArg {
            file,
            first_item,
            first_item_checked,
        } = self.console_or_file(at, &items, false)?;
        for (n, &arg) in items.iter().enumerate().skip(first_item) {
            self.write_arg(arg, first_item_checked && n == 0)?;
        }
        let mut parts = vec![file];
        parts.extend(&items[first_item..]);
        Ok(Kind::Write(self.ast.list(&parts), newline))
    }

    /// `Read(f, a, b)` is `a := ReadChar(f)` (or `ReadLong`, `ReadReal`, `ReadStr`, by the
    /// type of `a`) for each variable, so narrowing and truncation are the assignment's;
    /// `ReadLn` ends with `ReadLnEnd(f)`.
    fn read_call(&mut self, at: Id, args: List, line_end: bool) -> R<Kind> {
        let items = self.ast.items_vec(args);
        let FileArg {
            file,
            first_item,
            first_item_checked,
        } = self.console_or_file(at, &items, true)?;
        let mut steps = Vec::new();
        for (n, &target) in items.iter().enumerate().skip(first_item) {
            steps.push(self.read_item(at, file, target, first_item_checked && n == 0)?);
        }
        if line_end {
            steps.push(self.helper_call(at, Helper::ReadLnEnd, &[file], Ty::NONE));
        }
        Ok(Kind::Compound(self.ast.list(&steps)))
    }

    /// `target := ReadX(file)` for one variable of `Read`.
    fn read_item(&mut self, at: Id, file: Id, target: Id, checked: bool) -> R<Id> {
        if matches!(self.ast.kind(target), Kind::Fmt { .. }) {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        }
        let ty = if checked {
            self.ast.ty(target)
        } else {
            self.variable(target)?
        };
        let mut args = vec![file];
        let (helper, result) = if self.types.tag(ty) == Some(Tag::Char) {
            (Helper::ReadChar, Ty::CHAR)
        } else if let Ok(result) = self.types.val_type(ty) {
            let helper = if result == Ty::REAL {
                Helper::ReadReal
            } else {
                Helper::ReadLong
            };
            (helper, result)
        } else if self.types.is_str(ty) {
            let capacity = Kind::IntLit(self.types.capacity(ty) as i64);
            args.push(self.ast.typed(capacity, Ty::INTEGER, at));
            (Helper::ReadStr, Ty::STRING)
        } else {
            return Err(self.err(target, code::CANNOT_READ_OR_WRITE));
        };
        let call = self.helper_call(at, helper, &args, result);
        Ok(self.ast.typed(Kind::Assign(target, call), Ty::NONE, at))
    }

    /// `Eof` and `Eoln` without a file read the console: their argument is `Input`.
    fn default_input(&mut self, at: Id, r: u32, args: List) -> List {
        let console_routines = [Helper::Eof, Helper::Eoln].map(|h| self.helpers[h as usize]);
        if !console_routines.contains(&r) || !self.ast.items(args).is_empty() {
            return args;
        }
        let input = self.input_file(at);
        self.ast.list(&[input])
    }

    /// The error for a `var` argument of type other than `ty`: a file is asked for by name.
    fn var_mismatch(&self, at: Id, ty: Ty) -> Diag {
        let wrong_file = self.types.is_file(ty);
        let error = if wrong_file {
            code::FILE_VARIABLE_EXPECTED
        } else {
            code::TYPE_MISMATCH
        };
        self.err(at, error)
    }

    /// A call of System helper `helper` with `args`, as a node of type `ty` at `at`.
    fn helper_call(&mut self, at: Id, helper: Helper, args: &[Id], ty: Ty) -> Id {
        let list = self.ast.list(args);
        let call = Kind::Invoke(self.helpers[helper as usize], list);
        self.ast.typed(call, ty, at)
    }

    /// `Delete(s, i, n)` is `s := StrDelete(s, i, n)`, `Insert(src, s, i)` is
    /// `s := StrInsert(src, s, i)`: the call becomes that assignment.
    fn edit_string(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let is_insert = which == Intrinsic::Insert;
        let target = if is_insert { items[1] } else { items[0] };
        let ty = self.variable(target)?;
        if !self.types.is_str(ty) {
            return Err(self.err(target, code::STRING_VARIABLE_EXPECTED));
        }
        // The other two arguments: the string to insert and a position, or two counts.
        let others = if is_insert {
            [(items[0], Ty::STRING), (items[2], Ty::INTEGER)]
        } else {
            [(items[1], Ty::INTEGER), (items[2], Ty::INTEGER)]
        };
        for (arg, wanted) in others {
            self.expr(arg)?;
            self.argument_assignable(at, wanted, arg, Origin::System)?;
        }
        let helper = if is_insert {
            Helper::StrInsert
        } else {
            Helper::StrDelete
        };
        let call = self.helper_call(at, helper, &items, Ty::STRING);
        Ok(Kind::Assign(target, call))
    }

    /// `Inc(x [, n])` is `x := Ord(x) + n`, `Dec` subtracts; `n` defaults to 1.
    fn inc_dec(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 1, 2)?;
        let target = items[0];
        let ty = self.variable(target)?;
        let result = self
            .types
            .step_type(ty)
            .map_err(|code| self.err(target, code))?;
        let step = match items.get(1) {
            Some(&n) => {
                self.expr(n)?;
                self.argument_assignable(at, Ty::LONGINT, n, Origin::System)?;
                n
            }
            None => self.ast.typed(Kind::IntLit(1), Ty::INTEGER, at),
        };
        let current = self.ast.typed(Kind::Unary(Op::Ord, target), result, at);
        let op = if which == Intrinsic::Inc {
            Op::Add
        } else {
            Op::Sub
        };
        let sum = Kind::Binary {
            op,
            left: current,
            right: step,
        };
        let sum = self.ast.typed(sum, result, at);
        // `Inc` and `Dec` are never range checked.
        self.ast.set_range_checks(sum, false);
        Ok(Kind::Assign(target, sum))
    }

    /// `Str(v [: w [: d]], s)` is `s := StrInt(v, w, max)` for an integer and a formatted
    /// Real otherwise, `max` being the capacity of `s`.
    fn str_call(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 2, 2)?;
        let target = items[1];
        let ty = self.variable(target)?;
        self.write_arg(items[0], false)?;
        let (value, width, decimals) = match self.ast.kind(items[0]) {
            Kind::Fmt {
                value,
                width,
                decimals,
            } => (value, Some(width), decimals),
            _ => (items[0], None, None),
        };
        let value_ty = self.ast.ty(value);
        let result = self
            .types
            .str_type(ty, value_ty)
            .map_err(|code| self.err(target, code))?;
        let width = match width {
            Some(width) => width,
            None => self.ast.typed(Kind::IntLit(-1), Ty::INTEGER, at),
        };
        let converted = if value_ty == Ty::REAL {
            let format = Kind::Fmt {
                value,
                width,
                decimals,
            };
            self.ast.typed(format, result, at)
        } else {
            let max = self.types.size(ty) as i64 - 1;
            let max = self.ast.typed(Kind::IntLit(max), Ty::INTEGER, at);
            self.helper_call(at, Helper::StrInt, &[value, width, max], result)
        };
        Ok(Kind::Assign(target, converted))
    }

    /// `Val(s, v, code)` is `v := ValLong(s, code)`, or `ValReal` for a Real `v`.
    fn val_call(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let (source, target, error_code) = (items[0], items[1], items[2]);
        let ty = self.variable(target)?;
        let result = self
            .types
            .val_type(ty)
            .map_err(|code| self.err(target, code))?;
        self.expr(source)?;
        self.argument_assignable(at, Ty::STRING, source, Origin::System)?;
        self.var_arg(at, error_code, Ty::INTEGER)
            .map_err(|d| Diag {
                code: if d.code == code::TYPE_MISMATCH {
                    code::INTEGER_VARIABLE_EXPECTED
                } else {
                    d.code
                },
                ..d
            })?;
        let helper = if result == Ty::REAL {
            Helper::ValReal
        } else {
            Helper::ValLong
        };
        let call = self.helper_call(at, helper, &[source, error_code], result);
        Ok(Kind::Assign(target, call))
    }

    /// `Exp`, `Ln`, `Sin`, `Cos` and `ArcTan` of an integer or Real: a call of the System
    /// routine that computes it, with the argument converted to Real.
    fn real_function(&mut self, at: Id, which: Intrinsic, args: List) -> R<(Kind, Ty)> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 1, 1)?;
        let operand = items[0];
        self.expr(operand)?;
        // Like `Sqrt`, the operand must be an integer or a Real (error 79).
        let operand_ty = self.ast.ty(operand);
        self.types
            .unary(Op::Sqrt, operand_ty)
            .map_err(|code| self.err(at, code))?;
        self.coerce_real(operand);
        let helper = match which {
            Intrinsic::Exp => Helper::Exp,
            Intrinsic::Ln => Helper::Ln,
            Intrinsic::Sin => Helper::Sin,
            Intrinsic::Cos => Helper::Cos,
            _ => Helper::ArcTan,
        };
        let call = self.helper_call(at, helper, &[operand], Ty::REAL);
        Ok((self.ast.kind(call), Ty::REAL))
    }

    /// `Random` is a Real in [0, 1); `Random(n)` is a Word below `n`, for an integer `n`.
    /// Each is a call of the System routine that draws it.
    fn random_call(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 0, 1)?;
        let call = match items.first() {
            None => self.helper_call(at, Helper::RandomReal, &[], Ty::REAL),
            Some(&range) => {
                self.expr(range)?;
                self.argument_assignable(at, Ty::WORD, range, Origin::System)?;
                self.helper_call(at, Helper::RandomWord, &[range], Ty::WORD)
            }
        };
        Ok((self.ast.kind(call), self.ast.ty(call)))
    }

    /// The address of the variable `arg` of an untyped `var` parameter: any variable is
    /// accepted, whatever its type, and it must live in memory like any address-taken one.
    fn address_of_variable(&mut self, at: Id, arg: Id) -> R<Id> {
        self.variable(arg)?;
        if let Kind::Var(sym) = self.ast.kind(arg) {
            self.symbols.capture(sym, true);
        }
        Ok(self.ast.typed(Kind::AddrOf(arg), Ty::POINTER, at))
    }

    /// A byte count of type Word: any integer expression, a constant within 0 to 65535.
    fn byte_count(&mut self, at: Id, count: Id) -> R<()> {
        self.expr(count)?;
        self.argument_assignable(at, Ty::WORD, count, Origin::System)
    }

    /// `FillChar(x, count, value)`: stores `value`, an ordinal, in `count` bytes from `x`.
    fn fill_char(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let (target, count, value) = (items[0], items[1], items[2]);
        let first_byte = self.address_of_variable(at, target)?;
        self.byte_count(at, count)?;
        self.expr(value)?;
        match self.types.tag(self.ast.ty(value)) {
            None => return Err(self.err(value, code::ORDINAL_EXPRESSION_EXPECTED)),
            Some(Tag::Int) => self.argument_assignable(at, Ty::BYTE, value, Origin::System)?,
            Some(_) => {}
        }
        Ok(Kind::FillMem(first_byte, count, value))
    }

    /// `Move(source, dest, count)`: copies `count` bytes, correctly when they overlap.
    fn move_bytes(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let source = self.address_of_variable(at, items[0])?;
        let dest = self.address_of_variable(at, items[1])?;
        self.byte_count(at, items[2])?;
        Ok(Kind::CopyMem(dest, source, items[2]))
    }

    /// `Halt` and `Halt(code)`: a call of System's `HostHalt` with the exit code, 0 without
    /// an argument. TP types the code as a Word.
    fn halt_call(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 0, 1)?;
        let exit_code = match items.first() {
            Some(&arg) => {
                self.expr(arg)?;
                self.argument_assignable(at, Ty::WORD, arg, Origin::System)?;
                arg
            }
            None => self.ast.typed(Kind::IntLit(0), Ty::INTEGER, at),
        };
        let list = self.ast.list(&[exit_code]);
        Ok(Kind::Invoke(self.helpers[Helper::HostHalt as usize], list))
    }

    /// `New`, `Dispose`, `GetMem`, `FreeMem`, `Mark`, `Release` and `RunError`: calls of the
    /// System heap helpers, with `SizeOf(p^)` as the size of `New` and `Dispose`. `New`,
    /// `GetMem` and `Mark` assign the helper's result to the variable.
    fn heap_call(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        let count = if matches!(which, Intrinsic::GetMem | Intrinsic::FreeMem) {
            2
        } else {
            1
        };
        self.expect_arg_count(at, items.len(), count, count)?;
        let p = items[0];
        if which == Intrinsic::RunError {
            self.expr(p)?;
            self.argument_assignable(at, Ty::INTEGER, p, Origin::System)?;
            return Ok(Kind::RunErr(p));
        }
        let ty = self.variable(p)?;
        let to = self
            .types
            .target(ty)
            .ok_or(self.err(p, code::POINTER_VARIABLE_EXPECTED))?;
        let size = match items.get(1) {
            Some(&size) => {
                self.expr(size)?;
                self.argument_assignable(at, Ty::LONGINT, size, Origin::System)?;
                size
            }
            None => {
                let size_of_target = Kind::IntLit(self.types.size(to) as i64);
                self.ast.typed(size_of_target, Ty::LONGINT, at)
            }
        };
        let (helper, helper_args) = match which {
            Intrinsic::New | Intrinsic::GetMem => (Helper::HeapGet, vec![size]),
            Intrinsic::Dispose | Intrinsic::FreeMem => (Helper::HeapPut, vec![p, size]),
            Intrinsic::Mark => (Helper::HeapMark, vec![]),
            _ => (Helper::HeapRelease, vec![p]),
        };
        let list = self.ast.list(&helper_args);
        let call = Kind::Invoke(self.helpers[helper as usize], list);
        let returns_nothing = matches!(
            which,
            Intrinsic::Dispose | Intrinsic::FreeMem | Intrinsic::Release
        );
        if returns_nothing {
            return Ok(call);
        }
        let call = self.ast.typed(call, Ty::POINTER, at);
        Ok(Kind::Assign(p, call))
    }
}

/// TP's error for the number of arguments in a call, or None if it matches. A routine with
/// no parameters takes no parentheses: TP reads the call as complete and fails on the `(`,
/// with error 85 where a statement could end, or 89 where an argument list is open.
fn argument_count_error(params: usize, args: usize, nested: bool) -> Option<u16> {
    match (params, args) {
        (0, 0) => None,
        (0, _) if nested => Some(code::RIGHT_PAREN_EXPECTED),
        (0, _) => Some(code::SEMICOLON_EXPECTED),
        (_, 0) => Some(code::LEFT_PAREN_EXPECTED),
        (p, a) if a < p => Some(code::COMMA_EXPECTED),
        (p, a) if a > p => Some(code::RIGHT_PAREN_EXPECTED),
        _ => None,
    }
}

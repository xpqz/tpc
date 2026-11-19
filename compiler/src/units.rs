//! Unit loading: finds the source of every unit a program uses, parses it, and orders the units.
//!
//! The host registers every source file by name before compiling (decision 27), so a unit is
//! the file `NAME.PAS` of that table, any case; of two files with the same name the later
//! registration wins. Only the units a program reaches are lexed and parsed.
//!
//! One depth-first walk of the `uses` clauses, in the order they are written, produces both
//! orders the checker needs. A unit is visited when it is first named:
//!
//! 1. its interface `uses` are visited, then its interface is complete (`interface_order`);
//! 2. its implementation `uses` are visited, then the unit is done (`init_order`).
//!
//! A unit named while its interface is still open is a cycle through an interface: error 68.
//! A unit named after its interface is complete is fine even if the unit using it is still open,
//! which is how implementation parts may use each other (Turbo Pascal's rule, measured).
use crate::ast::{Ast, Id, Kind, List, Plan};
use crate::diag::{code, Diag};
use crate::lex::{lex, Tok};
use crate::parse::{parse_program, parse_system, parse_unit};
use crate::symbols::name_of;

/// A registered source file.
pub struct SourceFile {
    pub name: String,
    pub text: Vec<u8>,
}

#[derive(PartialEq)]
enum State {
    InterfaceOpen,
    InterfaceComplete,
    Done,
}

struct LoadedUnit {
    name: u32,
    state: State,
}

struct Loader<'a> {
    ast: Ast,
    files: &'a [SourceFile],
    units: Vec<LoadedUnit>,
    interface_order: Vec<Id>,
    init_order: Vec<Id>,
}

/// TP reports end of file on the last line of the file, not after its final newline.
fn last_line(text: &[u8]) -> u32 {
    let newlines = text.iter().filter(|&&c| c == b'\n').count() as u32;
    let unterminated = !text.ends_with(b"\n") as u32;
    (newlines + unterminated).max(1)
}

/// A diagnostic of file number `file` whose text is `text`. System's own diagnostics stay at
/// line 0, as its tokens are.
fn in_file(d: Diag, file: u32, text: &[u8]) -> Diag {
    let end_of_file = d.code == code::UNEXPECTED_END_OF_FILE && d.line != 0;
    let line = if end_of_file { last_line(text) } else { d.line };
    Diag { line, file, ..d }
}

/// Lexes and parses `text` as file number `file` with `parse`.
fn parse_file(
    ast: &mut Ast,
    file: u32,
    text: &[u8],
    parse: fn(&mut Ast, Vec<Tok>) -> Result<Id, Diag>,
) -> Result<Id, Diag> {
    ast.file = file;
    let tokens = lex(text).map_err(|d| in_file(d, file, text))?;
    parse(ast, tokens).map_err(|d| in_file(d, file, text))
}

/// Parses System, the program and every unit the program uses.
pub fn load(system: &[u8], program: &[u8], files: &[SourceFile]) -> Result<(Ast, Plan), Diag> {
    let mut ast = Ast::default();
    // Every token of System is at line 0, so its diagnostics never carry a user line.
    let mut system_tokens = lex(system).map_err(|d| Diag { line: 0, ..d })?;
    system_tokens.iter_mut().for_each(|t| t.line = 0);
    let system_node = parse_system(&mut ast, system_tokens).map_err(|d| in_file(d, 0, system))?;
    let program_node = parse_file(&mut ast, 0, program, parse_program)?;
    let Kind::Program { uses, .. } = ast.kind(program_node) else {
        unreachable!("parse_program makes a program")
    };
    let mut loader = Loader {
        ast,
        files,
        units: Vec::new(),
        interface_order: Vec::new(),
        init_order: Vec::new(),
    };
    loader.visit_uses(0, uses)?;
    let plan = Plan {
        system: system_node,
        interface_order: loader.interface_order,
        init_order: loader.init_order,
        program: program_node,
    };
    Ok((loader.ast, plan))
}

impl Loader<'_> {
    /// An error at the `uses` entry `entry` of file `file`.
    fn at_entry(&self, entry: Id, code: u16, file: u32) -> Diag {
        let (line, col) = self.ast.position(entry);
        Diag {
            code,
            line,
            col,
            file,
        }
    }

    fn visit_uses(&mut self, file: u32, uses: List) -> Result<(), Diag> {
        for entry in self.ast.items_vec(uses) {
            self.visit(file, entry)?;
        }
        Ok(())
    }

    /// The unit named by the `uses` entry `entry` of file `file`: loaded and walked if new.
    fn visit(&mut self, file: u32, entry: Id) -> Result<(), Diag> {
        let name = name_of(&self.ast, entry);
        if let Some(unit) = self.units.iter().find(|u| u.name == name) {
            return match unit.state {
                State::InterfaceOpen => {
                    Err(self.at_entry(entry, code::CIRCULAR_UNIT_REFERENCE, file))
                }
                _ => Ok(()),
            };
        }
        let (node, unit_file) = self.load_unit(file, entry, name)?;
        let Kind::Unit {
            interface_uses,
            implementation_uses,
            ..
        } = self.ast.kind(node)
        else {
            unreachable!("parse_unit makes a unit")
        };
        let index = self.units.len();
        self.units.push(LoadedUnit {
            name,
            state: State::InterfaceOpen,
        });
        self.visit_uses(unit_file, interface_uses)?;
        self.units[index].state = State::InterfaceComplete;
        self.interface_order.push(node);
        self.visit_uses(unit_file, implementation_uses)?;
        self.units[index].state = State::Done;
        self.init_order.push(node);
        Ok(())
    }

    /// Finds the file of the unit `name` and parses it. Returns the unit's node and the number
    /// of its file. Not found is error 15 and a different unit name is error 69, both at the
    /// `uses` entry.
    fn load_unit(&mut self, file: u32, entry: Id, name: u32) -> Result<(Id, u32), Diag> {
        let mut wanted = self.ast.names[name as usize].clone();
        wanted.extend_from_slice(b".PAS");
        let found = self
            .files
            .iter()
            .rposition(|f| f.name.as_bytes().eq_ignore_ascii_case(&wanted));
        let Some(position) = found else {
            return Err(self.at_entry(entry, code::FILE_NOT_FOUND, file));
        };
        let unit_file = position as u32 + 1;
        let text = &self.files[position].text;
        let node = parse_file(&mut self.ast, unit_file, text, parse_unit)?;
        let Kind::Unit { name: declared, .. } = self.ast.kind(node) else {
            unreachable!("parse_unit makes a unit")
        };
        if declared != name {
            return Err(self.at_entry(entry, code::UNIT_NAME_MISMATCH, file));
        }
        Ok((node, unit_file))
    }
}

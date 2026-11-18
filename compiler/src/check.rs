//! Whole-program checker: resolves `Write`/`WriteLn` and types string literals.
use crate::ast::{Ast, Id, Kind};
use crate::driver::Diag;
use crate::types::{Ty, Types};

pub fn check(ast: &mut Ast, root: Id) -> Result<Types, Diag> {
    stmt(ast, root)?;
    Ok(Types::new())
}

fn unknown(ast: &Ast, id: Id) -> Diag {
    let n = ast.nodes[id as usize];
    Diag {
        code: 3,
        line: n.line,
        col: n.col,
    }
}

fn stmt(ast: &mut Ast, id: Id) -> Result<(), Diag> {
    match ast.nodes[id as usize].kind {
        Kind::Compound(l) => {
            for s in ast.items(l).to_vec() {
                stmt(ast, s)?;
            }
        }
        Kind::Call(name, args) => {
            let ln = match &ast.names[name as usize][..] {
                b"WRITE" => false,
                b"WRITELN" => true,
                _ => return Err(unknown(ast, id)),
            };
            for a in ast.items(args).to_vec() {
                expr(ast, a)?;
            }
            ast.nodes[id as usize].kind = Kind::Write(args, ln);
        }
        _ => unreachable!("parser produces only statements here"),
    }
    Ok(())
}

fn expr(ast: &mut Ast, id: Id) -> Result<(), Diag> {
    match ast.nodes[id as usize].kind {
        Kind::StrLit(_) => ast.nodes[id as usize].ty = Ty::STRING,
        _ => return Err(unknown(ast, id)),
    }
    Ok(())
}

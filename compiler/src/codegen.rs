//! Walks the checked AST and emits the module (docs/abi.md).
use crate::ast::{Ast, Id, Kind};
use crate::wasm::{Code, Module, FUNC, I32, MEM};

const CALL: u8 = 0x10;
const PAGES: u32 = 16;
const DATA: u32 = 256;
const STDOUT: i32 = 1;

struct Gen<'a> {
    ast: &'a Ast,
    code: Code,
    /// Address of each string literal; address `DATA` holds the line feed.
    addr: Vec<u32>,
}

pub fn gen(ast: &Ast, root: Id) -> Vec<u8> {
    let mut data = vec![b'\n'];
    let mut addr = Vec::new();
    for s in &ast.strs {
        addr.push(DATA + data.len() as u32);
        data.extend_from_slice(s);
    }
    let mut g = Gen {
        ast,
        code: Code::new(),
        addr,
    };
    g.stmt(root);
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let main = m.ty(&[], &[]);
    m.import_func("host", "write", write);
    m.memory(PAGES);
    m.global_i32(true, (PAGES * 65536) as i32);
    let f = m.func(main, &[], g.code);
    m.export("main", FUNC, f);
    m.export("memory", MEM, 0);
    m.data(DATA, &data);
    m.finish()
}

impl Gen<'_> {
    fn write(&mut self, at: u32, len: usize) {
        self.code
            .i32_const(STDOUT)
            .i32_const(at as i32)
            .i32_const(len as i32)
            .idx(CALL, 0);
    }

    fn stmt(&mut self, id: Id) {
        match self.ast.nodes[id as usize].kind {
            Kind::Compound(l) => self.ast.items(l).iter().for_each(|&s| self.stmt(s)),
            Kind::Write(args, ln) => {
                for &a in self.ast.items(args) {
                    let Kind::StrLit(s) = self.ast.nodes[a as usize].kind else {
                        unreachable!()
                    };
                    self.write(self.addr[s as usize], self.ast.strs[s as usize].len());
                }
                if ln {
                    self.write(DATA, 1);
                }
            }
            _ => unreachable!("checker leaves only statements"),
        }
    }
}

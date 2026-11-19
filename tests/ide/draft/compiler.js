// Draft of host/compiler.js: the JS side of the compiler's wasm exports (plan section 3,
// decisions 27 and 28). It keeps the compiler instance, registers the unit and include files
// before each compile and turns the result into a plain object.

const SOURCE_FILE = /\.(PAS|INC)$/i;

export class Compiler {
  // `instance` is the instantiated compiler.wasm (it imports nothing).
  constructor(instance) {
    this.exports = instance.exports;
    this.registered = []; // upper-case file names in registration order; err_file counts from 1
  }

  // One alloc holds all `parts`; returns the address of each (add_file and compile only accept
  // ranges inside the buffer of the last alloc).
  put(...parts) {
    const { alloc, memory } = this.exports;
    const total = parts.reduce((sum, part) => sum + part.length, 0);
    let at = alloc(total);
    return parts.map((part) => {
      new Uint8Array(memory.buffer, at, part.length).set(part);
      at += part.length;
      return at - part.length;
    });
  }

  addFile(name, text) {
    const label = Uint8Array.from(name, (c) => c.charCodeAt(0));
    const [labelAt, textAt] = this.put(label, text);
    this.exports.add_file(labelAt, label.length, textAt, text.length);
    if (!this.registered.includes(name.toUpperCase())) this.registered.push(name.toUpperCase());
  }

  // Compiles `main` (the editor's buffer, as the file `mainName`) with every other source file
  // of `files` (a Map of name -> bytes) available to `uses` and `{$I}`. Returns
  // { module } or { code, line, col, file }, the file being that of the error.
  compile(mainName, main, files) {
    for (const [name, text] of files) {
      if (SOURCE_FILE.test(name) && name.toUpperCase() !== mainName.toUpperCase()) {
        this.addFile(name, text);
      }
    }
    const [at] = this.put(main);
    const x = this.exports;
    if (x.compile(at, main.length, 0) === 0) {
      return { module: new Uint8Array(x.memory.buffer, x.out_ptr(), x.out_len()).slice() };
    }
    const file = x.err_file();
    return {
      code: x.err_code(),
      line: x.err_line(),
      col: x.err_col(),
      file: file === 0 ? mainName : this.registered[file - 1],
    };
  }
}

// A hand-assembled compiler module with the exports of the real one. Its `compile` traps
// (`unreachable`) for an empty source and otherwise fails with error 5 at line 3, column 7,
// message "boom". It lets tests drive the trap path, which the real compiler does not reach.
const leb = (n) => {
  const bytes = [];
  do {
    let byte = n & 0x7f;
    n >>= 7;
    if (n !== 0) byte |= 0x80;
    bytes.push(byte);
  } while (n !== 0);
  return bytes;
};
const signedLeb = (n) => {
  const bytes = [];
  for (;;) {
    const byte = n & 0x7f;
    n >>= 7;
    if ((n === 0 && !(byte & 0x40)) || (n === -1 && byte & 0x40)) return [...bytes, byte];
    bytes.push(byte | 0x80);
  }
};
const vec = (items) => [...leb(items.length), ...items.flat()];
const section = (id, body) => [id, ...leb(body.length), ...body];
const name = (text) => [...leb(text.length), ...Buffer.from(text)];

export function fakeCompilerModule() {
  const types = [
    [0x60, 1, 0x7f, 1, 0x7f], // 0: (i32) -> i32
    [0x60, 3, 0x7f, 0x7f, 0x7f, 1, 0x7f], // 1: (i32 i32 i32) -> i32
    [0x60, 0, 1, 0x7f], // 2: () -> i32
    [0x60, 4, 0x7f, 0x7f, 0x7f, 0x7f, 1, 0x7f], // 3: (i32 i32 i32 i32) -> i32
  ];
  const functions = [
    ["alloc", 0, [0x41, ...signedLeb(1024), 0x0b]],
    ["add_file", 3, [0x41, 0, 0x0b]],
    // if (len == 0) unreachable; return 1
    ["compile", 1, [0x20, 1, 0x45, 0x04, 0x40, 0x00, 0x0b, 0x41, 1, 0x0b]],
    ["out_ptr", 2, [0x41, 0, 0x0b]],
    ["out_len", 2, [0x41, 0, 0x0b]],
    ["err_code", 2, [0x41, 5, 0x0b]],
    ["err_line", 2, [0x41, 3, 0x0b]],
    ["err_col", 2, [0x41, 7, 0x0b]],
    ["err_file", 2, [0x41, 0, 0x0b]],
    ["err_message_ptr", 2, [0x41, ...signedLeb(100), 0x0b]],
    ["err_message_len", 2, [0x41, 4, 0x0b]],
  ];
  const exports = functions.map(([fn], i) => [...name(fn), 0, i]);
  exports.push([...name("memory"), 2, 0]);
  const bytes = [
    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...section(1, vec(types)),
    ...section(3, vec(functions.map(([, type]) => [type]))),
    ...section(5, vec([[0, 1]])), // one memory, at least 1 page
    ...section(7, vec(exports)),
    ...section(10, vec(functions.map(([, , body]) => [...leb(body.length + 1), 0, ...body]))),
    ...section(11, vec([[0, 0x41, ...signedLeb(100), 0x0b, ...leb(4), ...Buffer.from("boom")]])),
  ];
  return new WebAssembly.Module(Uint8Array.from(bytes));
}

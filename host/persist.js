// Persistence of the IDE's files: the page keeps each file in a localStorage-like store under
// the key `tp5:NAME`, as a Latin-1 string (one character per byte), because the store holds
// strings only. `storage` needs length, key(i), getItem and setItem.
const PREFIX = "tp5:";

// All saved files as a Map of name -> bytes.
export function loadFiles(storage) {
  const files = new Map();
  for (let i = 0; i < storage.length; i++) {
    const key = storage.key(i);
    if (!key.startsWith(PREFIX)) continue;
    const text = storage.getItem(key);
    files.set(key.slice(PREFIX.length), Uint8Array.from(text, (c) => c.charCodeAt(0)));
  }
  return files;
}

export function saveFile(storage, name, bytes) {
  storage.setItem(PREFIX + name, Array.from(bytes, (b) => String.fromCharCode(b)).join(""));
}

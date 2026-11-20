// The extension entry. VS Code loads it as CommonJS; the code proper is ES modules (shared in
// style with the host), loaded with import(). The vscode API is handed to them, so they never
// import it.
const vscode = require("vscode");

let extension;

async function activate(context) {
  extension = await import("./src/extension.mjs");
  return extension.activate(vscode, context);
}

function deactivate() {
  extension?.deactivate();
}

exports.activate = activate;
exports.deactivate = deactivate;

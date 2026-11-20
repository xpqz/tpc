// The one place the extension reaches the host modules (host/*.js), which it shares with the
// browser IDE. In the repository they are two directories up; `make vscode` replaces this file
// in the package with one that points at the copy of host/ inside the extension.
export * from "../../host/compiler.js";
export * from "../../host/keys.js";
export * from "../../host/runtime.js";
export * from "../../host/screen.js";
export * from "../../host/terminal.js";

// A minimal fake of the vscode API, enough to drive activate(): diagnostics, documents,
// commands, the terminal and the window's messages. `state` records what the extension did.
export class FakeEventEmitter {
  constructor() {
    this.listeners = [];
    this.event = (listener) => {
      this.listeners.push(listener);
      return { dispose: () => (this.listeners = this.listeners.filter((l) => l !== listener)) };
    };
  }
  fire(value) {
    for (const listener of [...this.listeners]) listener(value);
  }
  dispose() {}
}

export function fakeVscode(documents, compilerPath, { settings = {} } = {}) {
  const shown = new Map();
  const handlers = {};
  const commands = new Map();
  const terminals = [];
  const messages = { error: [], status: [] };
  const state = { shown, handlers, commands, terminals, messages, documents, activeEditor: undefined };
  const event = (name) => (handler) => ((handlers[name] = handler), { dispose() {} });
  const values = { compilerPath, "diagnostics.onType": true, "run.writeFiles": true, ...settings };
  state.api = {
    languages: {
      createDiagnosticCollection: () => ({
        set: (uri, list) => shown.set(uri.fsPath, list),
        delete: (uri) => shown.delete(uri.fsPath),
        dispose() {},
      }),
    },
    commands: {
      registerCommand: (name, handler) => (commands.set(name, handler), { dispose() {} }),
    },
    window: {
      get activeTextEditor() {
        return state.activeEditor;
      },
      createTerminal: (options) => {
        const terminal = { options, shown: false, disposed: false };
        terminal.show = () => (terminal.shown = true);
        terminal.dispose = () => {
          terminal.disposed = true;
          options.pty.close?.();
        };
        terminals.push(terminal);
        return terminal;
      },
      showErrorMessage: (text) => (messages.error.push(text), Promise.resolve(undefined)),
      setStatusBarMessage: (text) => (messages.status.push(text), { dispose() {} }),
    },
    workspace: {
      textDocuments: documents,
      getConfiguration: () => ({ get: (key) => values[key] }),
      onDidOpenTextDocument: event("open"),
      onDidSaveTextDocument: event("save"),
      onDidChangeTextDocument: event("change"),
      onDidCloseTextDocument: event("close"),
      onDidChangeConfiguration: event("config"),
    },
    EventEmitter: FakeEventEmitter,
    Uri: { file: (fsPath) => ({ fsPath }) },
    Range: class {
      constructor(line, start, endLine, end) {
        Object.assign(this, { line, start, endLine, end });
      }
    },
    Diagnostic: class {
      constructor(range, message, severity) {
        Object.assign(this, { range, message, severity });
      }
    },
    DiagnosticSeverity: { Error: 0 },
  };
  return state;
}

export const fakeDocument = (fileName, text) => ({
  fileName,
  languageId: "tp5pascal",
  uri: { scheme: "file" },
  getText: () => text,
});

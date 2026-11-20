// The Pseudoterminal of one run. VS Code gives it the terminal's typing (handleInput) and
// shows what it writes. `EventEmitter` is vscode's, handed in so this module never imports it.
import { startRun } from "./run_session.mjs";
import { inputBytes } from "./terminal_text.mjs";

const CTRL_C = "\x03";
const PRESS_ANY_KEY = "\r\nPress any key to close the terminal.\r\n";

export function createRunPty(EventEmitter, { wasm, directory, writeFiles }) {
  const written = new EventEmitter();
  const closed = new EventEmitter();
  let session;
  let userClosed = false; // the terminal is gone: nothing more is written
  let closeFired = false;

  const write = (text) => {
    if (!userClosed) written.fire(text);
  };

  return {
    onDidWrite: written.event,
    onDidClose: closed.event,

    get running() {
      return session?.running ?? false;
    },

    open() {
      session = startRun({ wasm, directory, writeFiles, onOutput: write });
      session.ended.then(() => write(PRESS_ANY_KEY));
    },

    // The terminal was closed by the user: end the program without a word.
    close() {
      userClosed = true;
      session?.stop();
    },

    stop() {
      session?.stop();
    },

    // While the program runs, typing is its keyboard and Ctrl+C stops it. Once it has ended
    // the terminal waits for any key and then closes itself.
    handleInput(data) {
      if (session === undefined) return;
      if (!session.running) {
        if (!closeFired) closed.fire();
        closeFired = true;
      } else if (data.includes(CTRL_C)) {
        session.stop();
      } else {
        session.sendKeys(inputBytes(data));
      }
    },
  };
}

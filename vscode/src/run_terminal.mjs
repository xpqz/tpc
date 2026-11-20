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
  let dimensions; // the terminal's size, known before the program starts
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

    open(initialDimensions) {
      dimensions = initialDimensions ?? dimensions;
      session = startRun({ wasm, directory, writeFiles, onOutput: write });
      if (dimensions) session.resize(dimensions.columns, dimensions.rows);
      // A Crt program's last picture stays until a key is pressed; the prompt follows it.
      session.ended.then(() => {
        if (!session.screenHeld) write(PRESS_ANY_KEY);
      });
    },

    setDimensions(newDimensions) {
      dimensions = newDimensions;
      session?.resize(newDimensions.columns, newDimensions.rows);
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
    // the terminal waits for any key (the first one leaves a Crt picture and shows the end
    // text) and then closes itself.
    handleInput(data) {
      if (session === undefined) return;
      if (!session.running && session.screenHeld) {
        session.showEnd();
        write(PRESS_ANY_KEY);
      } else if (!session.running) {
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

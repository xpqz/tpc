import test, { mock } from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/scheduler.mjs");

test("calls within the delay run once, with the last key, after the delay", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  const { debouncer } = await load();
  const ran = [];
  const d = debouncer(300, (key) => ran.push(key));
  d.schedule("a");
  t.mock.timers.tick(200);
  d.schedule("a");
  t.mock.timers.tick(200);
  assert.deepEqual(ran, [], "the second call restarted the wait");
  t.mock.timers.tick(100);
  assert.deepEqual(ran, ["a"]);
});

test("different keys are debounced independently", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  const { debouncer } = await load();
  const ran = [];
  const d = debouncer(300, (key) => ran.push(key));
  d.schedule("a");
  d.schedule("b");
  t.mock.timers.tick(300);
  assert.deepEqual(ran.sort(), ["a", "b"]);
});

test("cancel drops a pending run", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  const { debouncer } = await load();
  const ran = [];
  const d = debouncer(300, (key) => ran.push(key));
  d.schedule("a");
  d.cancel("a");
  t.mock.timers.tick(1000);
  assert.deepEqual(ran, []);
});

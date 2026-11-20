// A debouncer: `schedule(key)` runs `run(key)` once, `delayMs` after the last call for that key.
export function debouncer(delayMs, run) {
  const pending = new Map();
  return {
    schedule(key) {
      clearTimeout(pending.get(key));
      pending.set(
        key,
        setTimeout(() => {
          pending.delete(key);
          run(key);
        }, delayMs),
      );
    },
    cancel(key) {
      clearTimeout(pending.get(key));
      pending.delete(key);
    },
  };
}

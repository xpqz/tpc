// Node stand-in for the browser's worker entry: runs host/worker.js in a worker_threads Worker.
import { parentPort, workerData } from "node:worker_threads";
import { runProgram } from "../../host/worker.js";

await runProgram({ ...workerData, post: (message) => parentPort.postMessage(message) });

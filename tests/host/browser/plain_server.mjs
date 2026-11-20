// A static file server that sends no COOP/COEP headers, like GitHub Pages. `serveDirectory`
// serves `root` under `prefix` (such as "/tpc/"); resolves with { port, stop }.
import http from "node:http";
import fs from "node:fs";
import path from "node:path";

const TYPES = {
  ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm", ".pas": "text/plain",
};

export function serveDirectory(root, prefix, port) {
  const server = http.createServer((req, res) => {
    const { pathname } = new URL(req.url, "http://x");
    let file = pathname.startsWith(prefix)
      ? path.join(root, decodeURIComponent(pathname.slice(prefix.length)))
      : null;
    if (file && pathname.endsWith("/")) file = path.join(file, "index.html");
    if (!file || !file.startsWith(root) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      res.writeHead(404).end("not found");
      return;
    }
    res.writeHead(200, { "Content-Type": TYPES[path.extname(file)] ?? "application/octet-stream" });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve) =>
    server.listen(port, () => resolve({ port, stop: () => server.close() })));
}

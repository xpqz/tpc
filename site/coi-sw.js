// Service worker that makes the site cross-origin isolated. GitHub Pages cannot send the
// COOP/COEP headers that SharedArrayBuffer needs, so every response passes through here and
// leaves with both headers added.
self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));

function withIsolationHeaders(response) {
  // An opaque or opaque-redirect response (status 0) has unreadable headers and cannot be
  // rebuilt; pass it through. The site makes no cross-origin requests, so none is expected.
  if (response.status === 0) return response;
  const headers = new Headers(response.headers);
  headers.set("Cross-Origin-Opener-Policy", "same-origin");
  headers.set("Cross-Origin-Embedder-Policy", "require-corp");
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

self.addEventListener("fetch", (event) => {
  const request = event.request;
  // Chrome rejects this combination when refetched; leave it to the browser.
  if (request.cache === "only-if-cached" && request.mode !== "same-origin") return;
  event.respondWith(fetch(request).then(withIsolationHeaders));
});

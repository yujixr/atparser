addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const res = await fetch("https://atcoder.jp/contests/");
  const html = await res.text();

  const { parser } = wasm_bindgen;
  await wasm_bindgen(wasm);

  return new Response(parser(html), {
    status: 200,
    headers: { "Content-Type": 'application/xml; charset="UTF-8"' }
  });
}

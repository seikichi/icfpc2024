import * as wasm from "wasm";

self.addEventListener("message", (event: MessageEvent<string>) => {
  (async () => {
    try {
      await wasm.default();
      self.postMessage(wasm.transpile(event.data));
    } catch (e) {
      console.error(e);
    }
  })();
});

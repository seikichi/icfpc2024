import * as wasm from "wasm";

self.addEventListener("message", (event: MessageEvent<string>) => {
  (async () => {
    try {
      await wasm.default();
      self.postMessage(wasm.eval_str(event.data));
    } catch (e) {
      console.error(e);
    }
  })();
});

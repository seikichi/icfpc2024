import * as wasm from "wasm";

self.addEventListener("message", (event: MessageEvent<[number, number]>) => {
  console.log("Worker received:", event.data);

  (async () => {
    try {
      await wasm.default();
      const [lhs, rhs] = event.data;
      self.postMessage(Number(wasm.add(lhs, rhs)));
    } catch (e) {
      console.error(e);
    }
  })();
});

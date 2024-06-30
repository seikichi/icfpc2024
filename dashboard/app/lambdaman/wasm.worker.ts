import * as wasm from "wasm";
import { WorkerRequest, WorkerResponse } from "./utils";
import { ok, err } from "@/lib/result";

(async () => {
  await wasm.default();

  self.addEventListener("message", (event: MessageEvent<WorkerRequest>) => {
    try {
      if (event.data.type === "TRANSPILE") {
        const experssion = wasm.transpile(event.data.source);
        const response: WorkerResponse = {
          id: event.data.id,
          result: ok(experssion),
        };
        self.postMessage(response);
      } else if (event.data.type === "EVAL") {
        const value = wasm.eval_str(event.data.expression);
        const response: WorkerResponse = {
          id: event.data.id,
          result: ok(value),
        };
        self.postMessage(response);
      } else {
        console.error("Unknown event", event);
      }
    } catch (e) {
      console.error("wasm error", e);
      const response: WorkerResponse = {
        id: event.data.id,
        result: err(e instanceof Error ? e.message : String(e)),
      };
      self.postMessage(response);
    }
  });
})();

"use client";

import { SubmitButton } from "@/components/submit";
import { useCallback, useEffect, useRef, useState } from "react";
import { WorkerRequest, WorkerResponse } from "./utils";
import { Result } from "@/lib/result";
import { z } from "zod";

const DEFAULT_CODE = `\
"solve lambdaman9 " . (
  -- sを3回繰り返す
  let f = \\s -> s . s . s in
  -- sを81回繰り返す
  let g = \\s -> f (f (f (f s))) in
  g (g "R" . g "L" . "D")
)
`;

function useWASM() {
  const workerRef = useRef<Worker>();

  const makeUniqueId = (() => {
    let counter = 0;
    return () => ++counter;
  })();

  useEffect(() => {
    const worker = new Worker(new URL("./wasm.worker.ts", import.meta.url));
    workerRef.current = worker;
    return () => {
      workerRef.current = undefined;
      worker.terminate();
    };
  }, []);

  return {
    transpile(code: string): Promise<string> {
      const id = makeUniqueId();
      const request: WorkerRequest = { id, type: "TRANSPILE", source: code };

      return new Promise((resolve, reject) => {
        if (!workerRef.current) {
          return reject("worker not found");
        }

        const handler = (event: MessageEvent<WorkerResponse>) => {
          if (event.data.id !== id) {
            return;
          }
          workerRef.current?.removeEventListener("message", handler);

          if (!event.data.result.ok) {
            reject(event.data.result.error);
            return;
          }
          resolve(event.data.result.value);
        };

        workerRef.current.addEventListener("message", handler);
        workerRef.current.postMessage(request);
      });
    },
    async eval(expression: string): Promise<string> {
      const id = makeUniqueId();
      const request: WorkerRequest = { id, type: "EVAL", expression };

      return new Promise((resolve, reject) => {
        if (!workerRef.current) {
          return reject("worker not found");
        }

        const handler = (event: MessageEvent<WorkerResponse>) => {
          if (event.data.id !== id) {
            return;
          }

          workerRef.current?.removeEventListener("message", handler);
          if (!event.data.result.ok) {
            reject(event.data.result.error);
            return;
          }
          resolve(event.data.result.value);
        };

        workerRef.current.addEventListener("message", handler);
        workerRef.current.postMessage(request);
      });
    },
  };
}

export default function Page() {
  const wasm = useWASM();

  const [expression, setExpression] = useState<string>("");
  const [value, setValue] = useState<string>("");

  const formAction = useCallback(
    async (formData: FormData) => {
      setExpression("");
      setValue("");

      try {
        const code = z.string().parse(formData.get("code"));
        console.log({ code });
        const expression = await wasm.transpile(code);
        console.log({ expression });
        setExpression(expression);
        const value = await wasm.eval(expression);
        setValue(value);
      } catch (e) {
        console.error(e);
      }
    },
    [wasm]
  );

  return (
    <>
      <h1>Lambdaman</h1>

      <section>
        <h2>DSL</h2>
        <form action={formAction}>
          <div>
            <label htmlFor="code">code</label>
            <textarea
              id="code"
              name="code"
              defaultValue={DEFAULT_CODE}
              style={{ width: "100%", boxSizing: "border-box" }}
            ></textarea>
          </div>

          <div>
            <SubmitButton style={{ width: "100%" }}>Execute</SubmitButton>
          </div>
        </form>

        <div>
          <code style={{ backgroundColor: "lightgray" }}>{expression}</code>
        </div>

        <div>
          <code style={{ backgroundColor: "lightgray" }}>{value}</code>
        </div>

        <div>
          <canvas />
        </div>
      </section>
    </>
  );
}

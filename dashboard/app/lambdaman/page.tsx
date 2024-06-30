"use client";

const NUM_LEVELS = 21;

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
  const [error, setError] = useState<string | null>(null);

  const formAction = useCallback(
    async (formData: FormData) => {
      setExpression("");
      setValue("");
      setError(null);

      try {
        const code = z.string().parse(formData.get("code"));
        const expression = await wasm.transpile(code);
        setExpression(expression);
        const value = await wasm.eval(expression);
        setValue(value);
      } catch (e) {
        console.error(e);
        setError(e instanceof Error ? e.message : String(e));
      }
    },
    [wasm]
  );

  const [map, setMap] = useState<string | null>(null);
  const onLevelChange = useCallback(
    (event: React.ChangeEvent<HTMLSelectElement>) => {
      const level = event.target.value;
      if (level === "") {
        return;
      }

      fetch(`/courses/lambdaman/lambdaman${level}`)
        .then((response) => response.text())
        .then(setMap)
        .catch(console.error);
    },
    []
  );

  return (
    <>
      <h1>Lambdaman</h1>

      <section>
        <select
          name="level"
          id="level"
          style={{ width: "100%" }}
          onChange={onLevelChange}
        >
          <option value="">--Please choose an level--</option>
          {Array(NUM_LEVELS)
            .fill(0)
            .map((_, key) => (
              <option key={key} value={`${key + 1}`}>
                lambdaman{key + 1}
              </option>
            ))}
        </select>
      </section>

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

        {error && (
          <div>
            <code style={{ backgroundColor: "pink" }}>{error}</code>
          </div>
        )}

        <div>
          <p>{map}</p>
        </div>
      </section>
    </>
  );
}

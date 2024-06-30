"use client";

import { SubmitButton } from "@/components/submit";
import { useCallback, useEffect, useRef, useState } from "react";
import { z } from "zod";

export default function Page() {
  const workerRef = useRef<Worker>();
  const [result, setResult] = useState<string | null>(null);

  useEffect(() => {
    const worker = new Worker(new URL("./wasm.worker.ts", import.meta.url));
    worker.onmessage = (ev: MessageEvent<string>) => {
      setResult(ev.data);
    };
    workerRef.current = worker;
    return () => {
      workerRef.current = undefined;
      worker.terminate();
    };
  }, []);

  const formAction = useCallback(async (formData: FormData) => {
    if (!workerRef.current) {
      return;
    }

    try {
      const code = z.string().parse(formData.get("code"));
      workerRef.current.postMessage(code);
    } catch (e) {
      console.error(e);
    }
  }, []);

  return (
    <>
      <h1>DSL</h1>

      <section>
        <form action={formAction}>
          <div>
            <label htmlFor="code">code</label>
            <textarea
              id="code"
              name="code"
              style={{ width: "100%", boxSizing: "border-box" }}
            ></textarea>
          </div>

          <div>
            <SubmitButton style={{ width: "100%" }}>Transpile</SubmitButton>
          </div>
        </form>
      </section>

      {result !== null && (
        <section>
          <code style={{ whiteSpace: "pre" }}>{result}</code>
        </section>
      )}
    </>
  );
}

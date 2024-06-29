"use client";

import { SubmitButton } from "@/components/submit";
import { useCallback, useEffect, useRef, useState } from "react";
import { z } from "zod";

const SAMPLE = `? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I" B% U- I$ I# ? B= I" B% I( I$ ? B= U- I" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4`;

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
      <h1>ICFP Language</h1>

      <section>
        <form action={formAction}>
          <div>
            <label htmlFor="code">code</label>
            <textarea
              id="code"
              name="code"
              style={{ width: "100%", boxSizing: "border-box" }}
              defaultValue={SAMPLE}
            ></textarea>
          </div>

          <div>
            <SubmitButton style={{ width: "100%" }}>Evaluate</SubmitButton>
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

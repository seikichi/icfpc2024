"use client";

import { useCallback, useRef, useState } from "react";
import { SubmitButton } from "@/components/submit";
import { z } from "zod";
import { experiment } from "./actions";
import { Result, err } from "@/lib/result";

export default function Page() {
  const [result, setResult] = useState<Result<{}, string> | null>(null);

  const formRef = useRef<HTMLFormElement>(null);

  const formAction = useCallback(async (formData: FormData) => {
    try {
      const course = z.string().parse(formData.get("course"));
      const levels = z.string().parse(formData.get("levels"));
      const tag = z.string().parse(formData.get("tag"));
      const args = z.string().parse(formData.get("args"));

      setResult(await experiment({ course, levels, tag, args }));
    } catch (e) {
      console.error(e);
      setResult(err(e instanceof Error ? e.message : JSON.stringify(e)));
    } finally {
      formRef.current?.reset();
    }
  }, []);

  return (
    <>
      <h1>Experiments</h1>
      <section>
        <h2>Submit</h2>

        <form ref={formRef} action={formAction}>
          <div>
            <label htmlFor="course">Course:</label>
            <input id="course" name="course" type="text" />
          </div>

          <div>
            <label htmlFor="levels">Levels:</label>
            <input id="levels" name="levels" type="text" />
          </div>

          <div>
            <label htmlFor="tag">Tag:</label>
            <input id="tag" name="tag" type="text" />
          </div>

          <div>
            <label htmlFor="args">Args:</label>
            <input id="args" name="args" type="args" />
          </div>

          <div>
            <SubmitButton>submit</SubmitButton>
          </div>
        </form>
      </section>
    </>
  );
}

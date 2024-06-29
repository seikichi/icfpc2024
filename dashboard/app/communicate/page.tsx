"use client";

import { Result, err } from "@/lib/result";
import { useState, useCallback, useRef } from "react";
import { communicate } from "./actions";
import { z } from "zod";
import { SubmitButton } from "@/components/submit";

export default function Page() {
  const formRef = useRef<HTMLFormElement>(null);

  const [response, setResponse] = useState<Result<
    { raw: string; evaluated: string | null },
    string
  > | null>(null);

  const formAction = useCallback(async (formData: FormData) => {
    try {
      const message = z.string().parse(formData.get("message"));
      setResponse(await communicate(message));
    } catch (e) {
      console.error(e);
      setResponse(err(JSON.stringify(e)));
    } finally {
      formRef.current?.reset();
    }
  }, []);

  return (
    <>
      <h1>Communicate</h1>
      <form ref={formRef} action={formAction}>
        <label htmlFor="message"></label>
        <textarea
          id="message"
          name="message"
          defaultValue="get index"
        ></textarea>
        <SubmitButton>send</SubmitButton>
      </form>
      <div>
        {response && response.ok && (
          <div>
            <code style={{ whiteSpace: "pre" }}>{response.value.raw}</code>
          </div>
        )}
        {response && response.ok && response.value.evaluated && (
          <div>
            <hr />
            <code style={{ whiteSpace: "pre-wrap" }}>
              {response.value.evaluated}
            </code>
          </div>
        )}
        {response && !response.ok && (
          <code style={{ whiteSpace: "pre" }}>Error: {response.error}</code>
        )}
      </div>
    </>
  );
}

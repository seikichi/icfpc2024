"use client";

import { Result, err } from "@/lib/result";
import { useState, useCallback } from "react";
import { communicate } from "./actions";
import { z } from "zod";

export default function Page() {
  const [response, setResponse] = useState<Result<
    { raw: string; evaluated: string | null },
    string
  > | null>(null);

  const handleSubmit = useCallback(
    async (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      const fd = new FormData(e.currentTarget);
      try {
        const message = z.string().parse(fd.get("message"));
        console.log({ message });
        setResponse(await communicate(message));
      } catch (e) {
        console.error(e);
        setResponse(err(JSON.stringify(e)));
      }
    },
    []
  );

  return (
    <>
      <h1>Communicate</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="message"></label>
        <input id="message" name="message" defaultValue="get index" />
        <button>send</button>
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
            <code style={{ whiteSpace: "pre" }}>
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

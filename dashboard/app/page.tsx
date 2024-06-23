"use client";

import { useEffect, useState } from "react";
import * as wasm from "wasm";

export default function Home() {
  const [answer, setAnswer] = useState("");
  useEffect(() => {
    (async () => {
      try {
        await wasm.default();
        setAnswer(`${wasm.add(40, 2)}`);
      } catch (e) {
        console.error(e);
      }
    })();
  }, []);

  return (
    <>
      Answer to the Ultimate Question of Life, the Universe, and Everything:{" "}
      {answer}
    </>
  );
}

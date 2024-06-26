"use client";

import { useEffect, useRef, useState } from "react";

export default function Page() {
  const workerRef = useRef<Worker>();
  const [answer, setAnswer] = useState("");

  useEffect(() => {
    const worker = new Worker(new URL("./wasm.worker.ts", import.meta.url));
    worker.onmessage = (ev: MessageEvent<number>) => {
      console.log("onmessage", ev);
      setAnswer(`${ev.data}`);
    };
    worker.postMessage([40, 2]);
    workerRef.current = worker;
    return () => {
      workerRef.current = undefined;
      worker.terminate();
    };
  }, []);

  return <>Answer: {answer}</>;
}

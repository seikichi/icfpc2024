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

  return (
    <>
      <h1 className="text-lg font-semibold text-gray-900 sm:text-xl dark:text-gray-50">
        WebAssembly Demo
      </h1>
      <div className="mt-4 sm:mt-6 lg:mt-10">
        <div className="my-40 flex w-full flex-col items-center justify-center">
          <h2 className="mt-6 text-lg font-semibold sm:text-xl">
            wasm.add(40 + 2) = {answer}
          </h2>
          <p className="mt-3 max-w-md text-center text-gray-500">
            Answer to the Ultimate Question of Life, the Universe, and
            Everything.
          </p>
        </div>
      </div>
    </>
  );
}

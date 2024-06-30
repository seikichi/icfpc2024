import { Result } from "@/lib/result";

export type WorkerRequest =
  | { id: number; type: "TRANSPILE"; source: string }
  | { id: number; type: "EVAL"; expression: string };

export type WorkerResponse = { id: number; result: Result<string, string> };

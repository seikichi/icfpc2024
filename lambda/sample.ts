import { Handler } from "aws-lambda";
import { z } from "zod";
import { setTimeout } from "timers/promises";

const SampleEvent = z.object({
  x: z.number(),
});
type SampleEvent = z.infer<typeof SampleEvent>;

type SampleOutput = { result: number };

// NOTE: optuna を使ってみる例
export const handler: Handler<unknown, SampleOutput> = async (event, _c) => {
  const { x } = SampleEvent.parse(event);
  await setTimeout(1000);

  return { result: (x - 2) ** 2 };
};

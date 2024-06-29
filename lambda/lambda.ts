import { Handler } from "aws-lambda";
import { solve } from "./solver";
import * as path from "path";

import { z } from "zod";

const Env = z.object({
  COMMIT_ID: z.string().min(1),
});

const SolverEvent = z.object({
  course: z.string(),
  level: z.number(),
  args: z.string(),
});

export const solver: Handler = async (rawEvent, _context) => {
  const env = Env.parse(process.env);
  const event = SolverEvent.parse(rawEvent);

  await solve({
    ...event,
    commitId: env.COMMIT_ID,
    tmpDir: "/tmp",
    courseDir: "./cources",
    solverDir: path.join("target", "release"),
  });
};

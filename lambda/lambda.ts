import { Handler } from "aws-lambda";
import { solve } from "./solver";
import * as path from "path";

import { z } from "zod";

import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient();

import {
  LambdaClient,
  InvokeCommand,
  InvocationType,
} from "@aws-sdk/client-lambda";

const SolverEnv = z.object({
  COMMIT_ID: z.string().min(1),
  BUCKET: z.string().min(1),
  POSTGRES_PRISMA_URL: z.string(),
  POSTGRES_URL_NON_POOLING: z.string(),
});

const SolverEvent = z.object({
  course: z.string(),
  level: z.number(),
  args: z.string(),
  experimentId: z.number().nullable(),
});

export const solver: Handler = async (rawEvent, _context) => {
  const env = SolverEnv.parse(process.env);
  const event = SolverEvent.parse(rawEvent);

  await solve({
    ...event,
    commitId: env.COMMIT_ID,
    tmpDir: "/tmp",
    courseDir: "./courses",
    solverDir: path.join("target", "release"),
    bucket: env.BUCKET,
    experimentId: event.experimentId,
  });
};

const ExperimentEnv = z.object({
  COMMIT_ID: z.string().min(1),
  SOLVER_LAMBDA_ARN: z.string().startsWith("arn:aws:lambda:"),
  POSTGRES_PRISMA_URL: z.string(),
  POSTGRES_URL_NON_POOLING: z.string(),
});

const ExperimentEvent = z.object({
  course: z.string(),
  levels: z.string().min(1).max(32),
  tag: z.string().max(16),
  args: z.string().max(256),
});

export const experiment: Handler = async (rawEvent, _context) => {
  const env = ExperimentEnv.parse(process.env);
  const event = ExperimentEvent.parse(rawEvent);

  // Parse levels (e.g., "1,2,3-5,7" => {1, 2, 3, 4, 5, 7})
  const ids: Set<number> = new Set([]);
  for (const ps of event.levels.split(",")) {
    if (ps.includes("-")) {
      const [fromS, toS] = ps.split("-");
      const [from, to] = [parseInt(fromS, 10), parseInt(toS, 10)];
      for (let i = from; i <= to; i++) {
        ids.add(i);
      }
    } else {
      ids.add(parseInt(ps, 10));
    }
  }

  const lambda = new LambdaClient({ region: "ap-northeast-1" });

  const { id: experimentId } = await prisma.experiment.create({
    data: {
      args: event.args,
      course: event.course,
      tag: event.tag,
      levels: event.levels,
      commitId: env.COMMIT_ID,
    },
  });

  for (const level of Array.from(ids)) {
    console.log(`invoke: ${event.course}${level}`);

    const solverEvent: z.infer<typeof SolverEvent> = {
      course: event.course,
      level,
      args: event.args,
      experimentId,
    };

    await lambda.send(
      new InvokeCommand({
        InvocationType: InvocationType.Event,
        FunctionName: env.SOLVER_LAMBDA_ARN,
        Payload: JSON.stringify(solverEvent),
      })
    );
  }
  return { ok: true, value: {} };
};

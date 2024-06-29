"use server";

import { z } from "zod";

import * as env from "@/lib/env";
import {
  LambdaClient,
  InvokeCommand,
  InvocationType,
} from "@aws-sdk/client-lambda";
import { ok, err, Result } from "@/lib/result";

const ExperimentParams = z.object({
  course: z.string(),
  levels: z.string().min(1).max(32),
  tag: z.string().max(16),
  args: z.string().max(256),
});

type ExperimentParams = z.infer<typeof ExperimentParams>;

export async function experiment(
  params: ExperimentParams
): Promise<Result<{}, string>> {
  const { AWS_DEFAULT_REGION, EXPERIMENT_LAMBDA_ARN } = env.load();
  const lambda = new LambdaClient({ region: AWS_DEFAULT_REGION });

  try {
    const command = new InvokeCommand({
      InvocationType: InvocationType.Event,
      FunctionName: EXPERIMENT_LAMBDA_ARN,
      Payload: JSON.stringify(ExperimentParams.parse(params)),
    });
    await lambda.send(command);
    return ok({});
  } catch (e) {
    return err(e instanceof Error ? e.message : JSON.stringify(e));
  }
}

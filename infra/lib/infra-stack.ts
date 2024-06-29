import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as child_process from "child_process";

import "dotenv/config";
import { z } from "zod";

const Env = z.object({
  POSTGRES_PRISMA_URL: z.string(),
  POSTGRES_URL_NON_POOLING: z.string(),
});

const env = Env.parse(process.env);

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const commitHash = child_process
      .execSync("git rev-parse --short HEAD")
      .toString()
      .trim();

    const bucket = new s3.Bucket(this, "Bucket", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      cors: [
        {
          allowedMethods: [s3.HttpMethods.GET],
          allowedOrigins: [
            "http://localhost:3000",
            "https://kmc-ob-icfpc2024.vercel.app",
          ],
          allowedHeaders: ["*"],
        },
      ],
    });

    // https://benlimmer.com/2024/04/08/caching-cdk-dockerimageasset-github-actions/
    const isCI = process.env.CI !== undefined;

    // NOTE: to run the following image manually,
    // 1. docker build -t seikichi/ifcpf2024-lambda -f lambda/Dockerfile .
    // 2. docker run --rm -p 9000:8080 seikichi/ifcpf2024-lambda lambda.handler
    // 3. curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{}'
    const solver = new lambda.DockerImageFunction(this, "Solver", {
      code: lambda.DockerImageCode.fromImageAsset("../", {
        file: "lambda/Dockerfile",
        cmd: ["lambda.solver"],
        ...(isCI
          ? {
              cacheTo: {
                type: "gha",
                params: { mode: "max" /** scope: "solver" **/ },
              },
              cacheFrom: [{ type: "gha" /** params: { scope: "solver" } **/ }],
              outputs: ["type=docker"],
            }
          : {}),
      }),
      timeout: cdk.Duration.minutes(15),
      memorySize: 256,
      environment: {
        COMMIT_ID: commitHash,
        // BUCKET: bucket.bucketName,
        // POSTGRES_PRISMA_URL: env.POSTGRES_PRISMA_URL,
        // POSTGRES_URL_NON_POOLING: env.POSTGRES_URL_NON_POOLING,
      },
    });

    bucket.grantReadWrite(solver);

    // Experiment
    const experiment = new lambda.DockerImageFunction(this, "Experiment", {
      code: lambda.DockerImageCode.fromImageAsset("../", {
        file: "lambda/Dockerfile",
        cmd: ["lambda.experiment"],
        ...(isCI
          ? {
              cacheTo: {
                type: "gha",
                params: { mode: "max" /** scope: "experiment" **/ },
              },
              cacheFrom: [
                { type: "gha" /** params: { scope: "experiment" } **/ },
              ],
              outputs: ["type=docker"],
            }
          : {}),
      }),
      timeout: cdk.Duration.minutes(15),
      memorySize: 256,
      environment: {
        COMMIT_ID: commitHash,
        SOLVER_LAMBDA_ARN: solver.functionArn,
        // BUCKET: bucket.bucketName,
        // POSTGRES_PRISMA_URL: env.POSTGRES_PRISMA_URL,
        // POSTGRES_URL_NON_POOLING: env.POSTGRES_URL_NON_POOLING,
      },
    });

    solver.grantInvoke(experiment);
  }
}

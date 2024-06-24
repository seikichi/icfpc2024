import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as child_process from "child_process";

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

    // NOTE: to run the following image manually,
    // 1. docker build -t seikichi/ifcpf2024-lambda -f lambda/Dockerfile .
    // 2. docker run --rm -p 9000:8080 seikichi/ifcpf2024-lambda lambda.handler
    // 3. curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{}'
    const solver = new lambda.DockerImageFunction(this, "Solver", {
      code: lambda.DockerImageCode.fromImageAsset("../", {
        file: "lambda/Dockerfile",
        cmd: ["lambda.handler"],
      }),
      timeout: cdk.Duration.minutes(15),
      memorySize: 4096,
      environment: {
        COMMIT_ID: commitHash,
        BUCKET: bucket.bucketName,
      },
    });

    bucket.grantReadWrite(solver);

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'InfraQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}

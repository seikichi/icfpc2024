import { promisify } from "util";
import * as child_process from "child_process";
import * as path from "path";
import * as fs from "fs/promises";

import { z } from "zod";

import {
  GetObjectCommand,
  PutObjectCommand,
  S3Client,
} from "@aws-sdk/client-s3";

import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient();

const exec = promisify(child_process.exec);
const s3 = new S3Client({ region: "ap-northeast-1" });

type Params = {
  course: string;
  level: number;

  tmpDir: string;
  solverDir: string;
  courseDir: string;
  args: string;

  commitId: string;
  experimentId: number | null;
  bucket: string;
};

const SolverOutput = z.object({
  solution: z.string(),
  score: z.number(),
});

export async function solve(params: Params) {
  const start = performance.now();
  const {
    course,
    level,
    tmpDir,
    courseDir,
    solverDir,
    args,
    commitId,
    experimentId,
    bucket,
  } = params;

  console.log({
    course,
    level,
    tmpDir,
    courseDir,
    solverDir,
    args,
    commitId,
    experimentId,
    bucket,
  });

  const targetPath = path.join(courseDir, course, `${course}${level}`);

  const solverPath = path.join(solverDir, course);
  const command = `${solverPath} -Q --input ${targetPath} ${args}`;

  let runId: number | null = null;
  try {
    const run = await prisma.run.create({
      data: {
        args,
        commitId,
        course,
        level,
        experimentId,
      },
    });
    runId = run.id;

    const { stdout, stderr } = await exec(command);
    console.log({ stdout, stderr });

    const output = SolverOutput.parse(JSON.parse(stdout));
    const elapsedSec = Math.ceil((performance.now() - start) / 1000);

    await s3.send(
      new PutObjectCommand({
        Bucket: bucket,
        Key: `runs/${runId}/solution`,
        Body: output.solution,
      })
    );

    await prisma.run.update({
      where: { id: runId },
      data: {
        score: output.score,
        elapsedSec,
      },
    });
  } catch (e) {
    console.error(e);

    const elapsedSec = Math.ceil((performance.now() - start) / 1000);
    if (runId !== null) {
      await prisma.run.update({
        where: { id: runId },
        data: {
          error: e instanceof Error ? e.message : JSON.stringify(e),
          elapsedSec,
        },
      });
    } else {
      await prisma.run.create({
        data: {
          args,
          commitId,
          course,
          level,
          experimentId,
          error: e instanceof Error ? e.message : JSON.stringify(e),
          elapsedSec,
        },
      });
    }
  }
}

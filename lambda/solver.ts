import { promisify } from "util";
import * as child_process from "child_process";
import * as path from "path";
import * as fs from "fs/promises";

// import { PrismaClient } from "@prisma/client";
// const prisma = new PrismaClient();

const exec = promisify(child_process.exec);

type Params = {
  course: string;
  level: number;

  tmpDir: string;
  solverDir: string;
  courseDir: string;
  args: string;

  commitId: string;
  // experimentId: number | null;
};

export async function solve(params: Params) {
  const start = performance.now();
  const { course, level, tmpDir, courseDir, solverDir, args, commitId } =
    params;

  console.log({
    course,
    level,
    tmpDir,
    courseDir,
    solverDir,
    args,
    commitId,
  });

  const targetPath = path.join(courseDir, course, `${course}${level}`);

  // outDir ...

  const solverPath = path.join(solverDir, course);
  const command = `${solverPath} --input ${targetPath} ${args}`;

  const { stdout, stderr } = await exec(command);

  console.log({ stdout, stderr });

  try {
  } catch (e) {
    console.error(e);
  }
}

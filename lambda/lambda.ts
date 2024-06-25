import * as child_process from "child_process";
import { Handler } from "aws-lambda";

import * as path from "path";
import { promisify } from "util";

import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient();

const exec = promisify(child_process.exec);

export const handler: Handler = async (_event, _context) => {
  try {
    console.log("# of problems:", await prisma.problem.count());
    const args = "--ai Simple,Simple";
    const solverPath = path.join("target", "release", "cli");
    const command = `${solverPath} ${args}`;
    console.log(`run: ${command}`);

    const { stdout, stderr } = await exec(command);

    console.log("stdout:", stdout);
    console.log("stderr:", stderr);
  } catch (e) {
    console.error(e);
  }
};

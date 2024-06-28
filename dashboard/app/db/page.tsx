"use server";

import { prisma } from "@/lib/prisma";

export default async function Page() {
  const problems = await prisma.problem.count();

  return (
    <>
      <h1>Database Access Demo</h1>
      <div># of Problems: {problems}</div>
    </>
  );
}

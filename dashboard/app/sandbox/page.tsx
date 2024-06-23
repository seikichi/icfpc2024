"use server";

import { prisma } from "@/lib/prisma";

export default async function Page() {
  const problems = await prisma.problem.count();
  return <># of Problems: {problems}</>;
}

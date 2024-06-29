"use server";

import { prisma } from "@/lib/prisma";

export default async function Page() {
  const experiments = await prisma.experiment.count();

  return (
    <>
      <h1>Database Access Demo</h1>
      <div># of Experiments: {experiments}</div>
    </>
  );
}

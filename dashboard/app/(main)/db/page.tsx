"use server";

import { prisma } from "@/lib/prisma";

export default async function Page() {
  const problems = await prisma.problem.count();

  return (
    <>
      <h1 className="text-lg font-semibold text-gray-900 sm:text-xl dark:text-gray-50">
        Database Access Demo
      </h1>
      <div className="mt-4 sm:mt-6 lg:mt-10">
        <div className="my-40 flex w-full flex-col items-center justify-center">
          <h2 className="mt-6 text-lg font-semibold sm:text-xl">
            # of Problems: {problems}
          </h2>
        </div>
      </div>
    </>
  );
}

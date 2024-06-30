import RunList from "@/app/experiments/[experimentId]/runlist";
import { prisma } from "@/lib/prisma";

export const dynamic = "force-dynamic";

export default async function Page({
  params,
}: {
  params: { course: string; level: string };
}) {
  const runs = await prisma.run.findMany({
    where: {
      course: params.course,
      level: Number(params.level),
    },
    orderBy: { score: "asc" },
    take: 20,
  });

  //   experiment.runs.sort((a, b) => a.level - b.level);
  return (
    <>
      <h1>
        {params.course}
        {params.level} (Top 20)
      </h1>

      <RunList runs={runs} />
    </>
  );
}

import { prisma } from "@/lib/prisma";
import RunList from "./runlist";

export const dynamic = "force-dynamic";

export default async function Page({
  params,
}: {
  params: { experimentId: string };
}) {
  const experimentId = params.experimentId;

  const experiment = await prisma.experiment.findFirst({
    where: { id: Number(experimentId) },
    include: { runs: true },
  });

  if (!experiment) {
    return <></>;
  }

  experiment.runs.sort((a, b) => a.level - b.level);

  return (
    <>
      <h1>Experiment {experimentId}</h1>
      <RunList experiment={experiment} />
    </>
  );
}

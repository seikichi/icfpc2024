import { prisma } from "@/lib/prisma";

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

  return (
    <>
      <h1>Experiment {experimentId}</h1>

      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Course</th>
            <th>Level</th>
            <th>Args</th>
            <th>Results</th>
            <th>Score</th>
            <th>Error</th>
          </tr>
        </thead>
        <tbody>
          {experiment.runs.map((run) => (
            <tr key={run.id}>
              <td>{run.id}</td>
              <td>{run.course}</td>
              <td>{run.level}</td>
              <td>{run.args}</td>
              <td>{run.score ? "ok" : run.error ? "failed" : "running"}</td>
              <td>{run.score && String(run.score)}</td>
              <td>{run.error && String(run.error)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

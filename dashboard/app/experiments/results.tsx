import { prisma } from "@/lib/prisma";

export default async function ExperimentResults() {
  const experiments = await prisma.experiment.findMany({
    take: 10,
    orderBy: { createdAt: "desc" },
    include: {
      runs: {
        include: {
          success: true,
          failed: true,
        },
      },
    },
  });

  return (
    <>
      <h2>Experiment Results</h2>

      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Course</th>
            <th>Levels</th>
            <th>Tag</th>
            <th>Args</th>
            <th>Succeeds</th>
            <th>Fails</th>
          </tr>
        </thead>
        <tbody>
          {experiments.map((e) => (
            <tr key={e.id}>
              <td>
                <a href={`/experiments/${e.id}`}>{e.id}</a>
              </td>
              <td>{e.course}</td>
              <td>{e.levels}</td>
              <td>{e.tag}</td>
              <td>{e.args}</td>
              <td>{e.runs.filter((r) => !!r.success).length}</td>
              <td>{e.runs.filter((r) => !!r.failed).length}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

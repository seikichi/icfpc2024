import { prisma } from "@/lib/prisma";

export default async function ExperimentResults() {
  const experiments = await prisma.experiment.findMany({
    take: 10,
    orderBy: { createdAt: "desc" },
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
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

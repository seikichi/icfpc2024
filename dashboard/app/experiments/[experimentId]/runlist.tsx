"use client";

import { Prisma } from "@prisma/client";
import { generateSolutionUrl } from "./actions";

type Props = {
  experiment: Prisma.ExperimentGetPayload<{ include: { runs: true } }>;
};

export default function RunList({ experiment }: Props) {
  const onDownload = async (runId: number) => {
    try {
      const res = await generateSolutionUrl(runId);
      if (!res.ok) {
        throw new Error(res.error);
      }

      const link = document.createElement("a");
      try {
        link.href = res.value;
        link.setAttribute("target", "_blank");
        document.body.appendChild(link);
        link.click();
      } finally {
        link.parentNode?.removeChild(link);
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
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
          <th>Solution</th>
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
            <td>
              {run.score && (
                <button onClick={() => onDownload(run.id)}>Download</button>
              )}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

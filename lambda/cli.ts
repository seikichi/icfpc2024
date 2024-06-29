import { solve } from "./solver";
import * as path from "path";

// テスト用

(async () => {
  await solve({
    course: "lambdaman",
    level: 1,
    courseDir: path.join("..", "courses"),
    solverDir: path.join("..", "solver", "target", "debug"),
    tmpDir: "./tmp",
    args: "--ai Simple,Simple",
    commitId: "DUMMY",
    bucket: "DUMMY",
    experimentId: null,
  });
})();

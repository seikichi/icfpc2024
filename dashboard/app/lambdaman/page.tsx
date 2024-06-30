"use client";

const NUM_LEVELS = 21;

import { SubmitButton } from "@/components/submit";
import { useCallback, useEffect, useRef, useState } from "react";
import { WorkerRequest, WorkerResponse } from "./utils";
import { z } from "zod";

import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-haskell";
import "ace-builds/src-noconflict/theme-github";
import "ace-builds/src-noconflict/ext-language_tools";

const DEFAULT_CODE = `\
"solve lambdaman9 " . (
  -- sを3回繰り返す
  let f = \\s -> s . s . s in
  -- sを81回繰り返す
  let g = \\s -> f (f (f (f s))) in
  g (g "R" . g "L" . "D")
)
`;

function useWASM() {
  const workerRef = useRef<Worker>();

  const makeUniqueId = (() => {
    let counter = 0;
    return () => ++counter;
  })();

  useEffect(() => {
    const worker = new Worker(new URL("./wasm.worker.ts", import.meta.url));
    workerRef.current = worker;
    return () => {
      workerRef.current = undefined;
      worker.terminate();
    };
  }, []);

  return {
    transpile(code: string): Promise<string> {
      const id = makeUniqueId();
      const request: WorkerRequest = { id, type: "TRANSPILE", source: code };

      return new Promise((resolve, reject) => {
        if (!workerRef.current) {
          return reject("worker not found");
        }

        const handler = (event: MessageEvent<WorkerResponse>) => {
          if (event.data.id !== id) {
            return;
          }
          workerRef.current?.removeEventListener("message", handler);

          if (!event.data.result.ok) {
            reject(event.data.result.error);
            return;
          }
          resolve(event.data.result.value);
        };

        workerRef.current.addEventListener("message", handler);
        workerRef.current.postMessage(request);
      });
    },
    async eval(expression: string): Promise<string> {
      const id = makeUniqueId();
      const request: WorkerRequest = { id, type: "EVAL", expression };

      return new Promise((resolve, reject) => {
        if (!workerRef.current) {
          return reject("worker not found");
        }

        const handler = (event: MessageEvent<WorkerResponse>) => {
          if (event.data.id !== id) {
            return;
          }

          workerRef.current?.removeEventListener("message", handler);
          if (!event.data.result.ok) {
            reject(event.data.result.error);
            return;
          }
          resolve(event.data.result.value);
        };

        workerRef.current.addEventListener("message", handler);
        workerRef.current.postMessage(request);
      });
    },
  };
}

function renderMapToCanvas(map: string, canvas: HTMLCanvasElement) {
  const CELL_SIZE = 5; // px
  const GRID_COLOR = "#CCCCCC";
  const WALL_COLOR = "#000000";
  const EMPTY_COLOR = "#FFFFFF";
  const PILL_COLOR = "#fdff00";
  const LAMBDA_COLOR = "#d03e19";

  const cells = map
    .trim()
    .split("\n")
    .map((line) => Array.from(line));

  const height = cells.length;
  const width = cells[0].length;

  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext("2d")!;

  // draw grid
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }
  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();

  // draw cells
  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const cell = cells[row][col] as Cell;

      ctx.fillStyle =
        cell === "_"
          ? EMPTY_COLOR
          : cell === "#"
            ? WALL_COLOR
            : cell === "L"
              ? LAMBDA_COLOR
              : PILL_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
}

function applySolution(map: string, solution: string): string {
  const cells = map
    .trim()
    .split("\n")
    .map((line) => Array.from(line));

  // find lambda cell (L)
  let lambdaRow = -1;
  let lambdaCol = -1;
  for (let row = 0; row < cells.length; row++) {
    for (let col = 0; col < cells[row].length; col++) {
      if (cells[row][col] === "L") {
        lambdaRow = row;
        lambdaCol = col;
        break;
      }
    }
  }

  if (lambdaRow === -1 || lambdaCol === -1) {
    throw "Failed to find Lambda";
  }

  // NOTE: moves is "R", "L", "U", "D" list
  const moves = solution.split(" ")[2];

  for (const move of moves) {
    const newRow = lambdaRow + (move === "D" ? 1 : move === "U" ? -1 : 0);
    const newCol = lambdaCol + (move === "R" ? 1 : move === "L" ? -1 : 0);

    // if out of map, do not move
    if (
      newRow < 0 ||
      newRow >= cells.length ||
      newCol < 0 ||
      newCol >= cells[0].length
    ) {
      continue;
    }
    // if wall, do not move
    if (cells[newRow][newCol] === "#") {
      continue;
    }

    cells[lambdaRow][lambdaCol] = "_";
    cells[newRow][newCol] = "L";
    lambdaRow = newRow;
    lambdaCol = newCol;
  }

  // create newMap from cells
  const newMap = cells.map((line) => line.join("")).join("\n");
  return newMap;
}

// pill, lambda, wall, empty
type Cell = "." | "L" | "#" | "_";

export default function Page() {
  const wasm = useWASM();

  const [expression, setExpression] = useState<string>("");
  const [value, setValue] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [level, setLevel] = useState("");
  const [map, setMap] = useState<string | null>(null);
  const [code, setCode] = useState(DEFAULT_CODE);

  const onEditorChange = useCallback((code: string) => {
    setCode(code);
  }, []);

  const formAction = useCallback(async () => {
    setExpression("");
    setValue("");
    setError(null);

    try {
      // const code = z.string().parse(formData.get("code"));
      const expression = await wasm.transpile(code);
      setExpression(expression);
      const value = await wasm.eval(expression);
      setValue(value);

      if (map === null) {
        return;
      }

      renderMapToCanvas(applySolution(map, value), canvasRef.current!);
    } catch (e) {
      console.error(e);
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [wasm, map, code]);

  const onLevelChange = useCallback(
    (event: React.ChangeEvent<HTMLSelectElement>) => {
      setMap(null);
      const level = event.target.value;
      setLevel(level);
      if (level === "") {
        return;
      }

      (async () => {
        try {
          const res = await fetch(`/courses/lambdaman/lambdaman${level}`);
          const text = await res.text();
          setMap(text);
          renderMapToCanvas(text, canvasRef.current!);
        } catch (e) {
          console.error(e);
          setError(e instanceof Error ? e.message : String(e));
        }
      })();
    },
    []
  );

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const reload = useCallback(() => {
    setMap(null);
    if (level === "") {
      return;
    }
    (async () => {
      try {
        const res = await fetch(`/courses/lambdaman/lambdaman${level}`);
        const text = await res.text();
        setMap(text);
        renderMapToCanvas(text, canvasRef.current!);
      } catch (e) {
        console.error(e);
        setError(e instanceof Error ? e.message : String(e));
      }
    })();
  }, [level]);

  return (
    <>
      <h1>Lambdaman</h1>

      <section>
        <select
          name="level"
          id="level"
          style={{ width: "100%" }}
          value={level}
          onChange={onLevelChange}
        >
          <option value="">--Please choose an level--</option>
          {Array(NUM_LEVELS)
            .fill(0)
            .map((_, key) => (
              <option key={key} value={`${key + 1}`}>
                lambdaman{key + 1}
              </option>
            ))}
        </select>

        <button onClick={reload} style={{ width: "100%", marginTop: "6px" }}>
          Reload
        </button>
      </section>

      <section>
        <h2>DSL</h2>
        <form action={formAction}>
          <div>
            <AceEditor
              mode="haskell"
              theme="github"
              name="code"
              value={code}
              style={{ width: "100%", boxSizing: "border-box" }}
              editorProps={{ $blockScrolling: true }}
              onChange={onEditorChange}
              height="160px"
            />
          </div>

          <div>
            <SubmitButton style={{ width: "100%" }}>Execute</SubmitButton>
          </div>
        </form>

        <div>
          <code style={{ backgroundColor: "lightgray" }}>{expression}</code>
        </div>

        <div>
          <code style={{ backgroundColor: "lightgray" }}>{value}</code>
        </div>

        {error && (
          <div>
            <code style={{ backgroundColor: "pink" }}>{error}</code>
          </div>
        )}

        <div>
          <canvas style={{ width: "100%" }} ref={canvasRef} />
        </div>
      </section>
    </>
  );
}

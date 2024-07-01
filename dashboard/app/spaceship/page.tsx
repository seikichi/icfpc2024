"use client";

const NUM_LEVELS = 25;

// import { SubmitButton } from "@/components/submit";
import { ChangeEvent, useCallback, useEffect, useRef, useState } from "react";
// import { WorkerRequest, WorkerResponse } from "./utils";
import { z } from "zod";

type SquareList = [number, number][];

const vyvx: { [key: number]: [number, number] } = {
  1: [-1, -1],
  2: [-1, 0],
  3: [-1, 1],
  4: [0, -1],
  5: [0, 0],
  6: [0, 1],
  7: [1, -1],
  8: [1, 0],
  9: [1, 1],
} as const;

function renderSquaresToCanvas(
  squares: SquareList,
  canvas: HTMLCanvasElement,
  moves: number[]
) {
  const GRID_COLOR = "#DDDDDD";
  const SQUARE_COLOR = "#0000ff";
  const LAMBDA_COLOR = "#d03e19";

  // NOTE: Lambda は (0, 0) からスタート
  // square は (-100, 10) など負の座標を取る
  // まずは盤面のサイズを把握したいので最小・最大の x, y を取得する
  let minX = 0,
    minY = 0,
    maxX = 0,
    maxY = 0;
  for (const [x, y] of squares) {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  }

  const width = maxX - minX + 1;
  const height = maxY - minY + 1;

  // マスのサイズ (適当)
  const CELL_SIZE = width * height < 100 ? 30 : 5;

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

  // draw squares
  ctx.beginPath();

  // Lamdba (0, 0) を塗る (座標のオフセットを考慮)
  {
    ctx.fillStyle = LAMBDA_COLOR;
    let col = -minX;
    let row = -minY;
    ctx.fillRect(
      col * (CELL_SIZE + 1) + 1,
      row * (CELL_SIZE + 1) + 1,
      CELL_SIZE,
      CELL_SIZE
    );
  }

  // squares を塗る
  for (const [x, y] of squares) {
    ctx.fillStyle = SQUARE_COLOR;
    let col = x - minX;
    let row = y - minY;
    ctx.fillRect(
      col * (CELL_SIZE + 1) + 1,
      row * (CELL_SIZE + 1) + 1,
      CELL_SIZE,
      CELL_SIZE
    );
  }
  ctx.stroke();

  if (moves.length === 0) {
    return;
  }

  // moves を塗る
  ctx.beginPath();
  ctx.strokeStyle = "red";
  ctx.lineWidth = 2;
  let x = 0,
    y = 0,
    vx = 0,
    vy = 0;

  for (const move of moves) {
    const [dy, dx] = vyvx[move];
    ctx.moveTo(
      (x - minX) * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1,
      (y - minY) * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1
    );
    vx += dx;
    vy += dy;
    x += vx;
    y += vy;
    ctx.lineTo(
      (x - minX) * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1,
      (y - minY) * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1
    );
  }
  ctx.stroke();
}

export default function Page() {
  // const wasm = useWASM();

  // const [expression, setExpression] = useState<string>("");
  // const [value, setValue] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [level, setLevel] = useState("");
  // const [map, setMap] = useState<string | null>(null);
  const [squares, setSquares] = useState<[number, number][] | null>(null);

  const onLevelChange = useCallback(
    (event: React.ChangeEvent<HTMLSelectElement>) => {
      setSquares(null);
      const level = event.target.value;
      setLevel(level);
      if (level === "") {
        return;
      }

      (async () => {
        try {
          const res = await fetch(`/courses/spaceship/spaceship${level}`);
          const text = await res.text();
          const squares = z
            .tuple([z.number(), z.number()])
            .array()
            .parse(
              text
                .trim()
                .split("\n")
                .map((line) => line.split(" ").map((v) => Number(v)))
            );
          setSquares(squares);
          renderSquaresToCanvas(squares, canvasRef.current!, []);
        } catch (e) {
          console.error(e);
          setError(e instanceof Error ? e.message : String(e));
        }
      })();
    },
    []
  );

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const onFileChange = useCallback(
    async (event: ChangeEvent<HTMLInputElement>) => {
      const files = event.currentTarget.files;
      if (files?.length !== 1) {
        return;
      }
      const file = files[0];

      (async () => {
        const reader = new FileReader();
        reader.addEventListener("load", () => {
          if (squares === null) {
            return;
          }
          try {
            const moves = z
              .string()
              .parse(reader.result)
              .trim()
              .split("")
              .map((c) => Number(c));
            renderSquaresToCanvas(squares, canvasRef.current!, moves);
          } catch (e) {
            console.error(e);
            setError(e instanceof Error ? e.message : String(e));
          }
        });
        reader.readAsText(file);
      })();
    },
    [squares]
  );

  return (
    <>
      <h1>Spaceship</h1>

      <section>
        <p>NOTE: でかいマップ死にます (ごめん)</p>
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
                spaceship{key + 1}
              </option>
            ))}
        </select>

        {level !== "" && (
          <div>
            Previous Experiments:{" "}
            <a
              target="_blank"
              href={`/courses/spaceship/levels/${level}`}
              rel="noopener noreferrer"
            >
              spaceship{level}
            </a>
          </div>
        )}
        {squares !== null && (
          <div>
            <input
              onChange={onFileChange}
              style={{ width: "100%", marginTop: "4px" }}
              type="file"
            />
          </div>
        )}
      </section>

      {error && (
        <div>
          <code style={{ backgroundColor: "pink" }}>{error}</code>
        </div>
      )}

      <section>
        <div>
          <canvas style={{ width: "100%" }} ref={canvasRef} />
        </div>
      </section>
    </>
  );
}

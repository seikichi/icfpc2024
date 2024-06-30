// NOTE: あとで Lambda にするかもね

import "dotenv/config";

import * as path from "path";
import { z } from "zod";
import {
  GetObjectCommand,
  PutObjectCommand,
  S3Client,
} from "@aws-sdk/client-s3";

import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient();
const s3 = new S3Client({ region: "ap-northeast-1" });

// result
export type Result<S, E> = { ok: true; value: S } | { ok: false; error: E };

export function ok<S, E>(value: S): Result<S, E> {
  return { ok: true, value };
}

export function err<S, E>(error: E): Result<S, E> {
  return { ok: false, error };
}

// icfp
const TABLE =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

const RTABLE: { [key: string]: number } = {};
for (let i = 0; i < TABLE.length; i++) {
  RTABLE[TABLE[i]] = i;
}

export function decodeString(s: string): string | null {
  if (s[0] !== "S") {
    return null;
  }

  let result = "";
  for (let i = 1; i < s.length; i++) {
    result += TABLE[s.charCodeAt(i) - 33];
  }
  return result;
}

export function encodeString(s: string): string {
  let result = "S";
  for (let i = 0; i < s.length; i++) {
    result += String.fromCharCode(RTABLE[s[i]] + 33);
  }
  return result;
}

// client
const URL = "https://boundvariable.space/communicate";
const API_TOKEN = process.env.API_TOKEN!;

export const client = {
  async send(
    body: string
  ): Promise<Result<{ raw: string; evaluated: string | null }, string>> {
    const res = await fetch(URL, {
      method: "POST",
      headers: { Authorization: `Bearer ${API_TOKEN}` },
      body: encodeString(body),
      cache: "no-store",
    });
    if (!res.ok) {
      return err(await res.text());
    }
    const raw = await res.text();
    const evaluated = decodeString(raw);
    return ok({ raw, evaluated });
  },

  async sendRaw(
    body: string
  ): Promise<Result<{ raw: string; evaluated: string | null }, string>> {
    const res = await fetch(URL, {
      method: "POST",
      headers: { Authorization: `Bearer ${API_TOKEN}` },
      body,
      cache: "no-store",
    });
    if (!res.ok) {
      return err(await res.text());
    }
    const raw = await res.text();
    const evaluated = decodeString(raw);
    return ok({ raw, evaluated });
  },
};

function compress(str: string) {
  const TABLE =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

  const RTABLE: { [key: string]: number } = {};
  for (let i = 0; i < TABLE.length; i++) {
    RTABLE[TABLE[i]] = i;
  }

  let encoded = "";
  let count = 1;

  for (let i = 1; i <= str.length; i++) {
    if (str[i] === str[i - 1] && count < 93) {
      count++;
    } else {
      encoded +=
        String.fromCharCode(RTABLE[str[i - 1]] + 33) +
        String.fromCharCode(count + 33);
      count = 1;
    }
  }

  const compressed = "S" + encoded;
  // const code =
  //   'B$ B$ Lg B$ Lf B$ vf vf Lh B$ vg Lx B$ B$ vh vh vx Lr Li ? B= vi S S B. B$ B$ B$ Lg B$ Lf B$ vf vf Lh B$ vg Lx B$ B$ vh vh vx Ls Lc Lk ? B= vk I! S B. vc B$ B$ vs vc B- vk I" BT I" vi U# BT I" BD I" vi B$ vr BD I# vi';
  // const code =
  //   'B$ Ly B$ vy Lr Li ? B= vi S S B. B$ B$ B$ vy Ls Lc Lk ? B= vk I! S B. vc B$ B$ vs vc B- vk I" BT I" vi U# BT I" BD I" vi B$ vr BD I# vi Lg B$ Lf B$ vf vf Lh B$ vg Lx B$ B$ vh vh vx';
  const code = `B$ B$ Ly B$ vy Lr Li ? B= vi S S B. B$ B$ B$ vy Ls Lc Lk ? B= vk I! S B. vc B$ B$ vs vc B- vk I" BT I" vi U# BT I" BD I" vi B$ vr BD I# vi Lg B$ Lf B$ vf vf Lh B$ vg Lx B$ B$ vh vh vx`;

  return `${code} ${compressed}`;
}

function packInt(course: string, level: number, solution: string) {
  // packint
  const body =
    'BD I" B$ B$ L! B$ L" B$ v! B$ v" v" L" B$ v! B$ v" v" L# L$ ? B= v$ I! S B. B$ v# B/ v$ I% BT I" BD B% v$ I% SFL>O';

  function encodeInt(n: bigint) {
    if (n === BigInt(0)) {
      return "I!";
    }
    let ret: string[] = [];
    while (n > BigInt(0)) {
      ret.push(String.fromCharCode(Number((n % BigInt(94)) + BigInt(33))));
      n /= BigInt(94);
    }
    ret.reverse();
    return "I" + ret.join("");
  }

  let packInt = BigInt(1);
  const mapping: { [key: string]: number } = { L: 0, R: 1, D: 2, U: 3 };
  for (const c of solution) {
    packInt = packInt * BigInt(4) + BigInt(mapping[c]);
  }

  const prefix = encodeString(`solve ${course}${level} `);
  const code = `B. ${prefix} ${body} ${encodeInt(packInt)}`;
  // console.log({ encodeInt: code });
  return code;
}

function rlpack(course: string, level: number, solution: string) {
  function runLength(input: string): [string, number][] {
    if (input.length === 0) {
      return [];
    }
    let prev = input[0];
    let run = 1;
    let ret: [string, number][] = [];
    for (let c of input.slice(1)) {
      if (c === prev) {
        run += 1;
      } else {
        ret.push([prev, run]);
        prev = c;
        run = 1;
      }
    }
    ret.push([prev, run]);
    return ret;
  }

  function splitTooLargeRuns(
    runs: [string, number][],
    nLengthBits: number
  ): [string, number][] {
    const maxRun = 2 ** nLengthBits - 1;
    let ret: [string, number][] = [];
    for (let [c, run] of runs) {
      while (run > maxRun) {
        ret.push([c, maxRun]);
        run -= maxRun;
      }
      ret.push([c, run]);
    }
    return ret;
  }

  function runsToInt(runs: [string, number][], nLengthBits: number): bigint {
    let ret = BigInt(1);
    for (let [c, run] of runs) {
      if (run >= 2 ** nLengthBits) {
        throw new Error(`run is too large: ${c} ${run}`);
      }
      ret = (ret << BigInt(nLengthBits)) + BigInt(run);
      const d = "LRDU".indexOf(c);
      ret = (ret << BigInt(2)) + BigInt(d);
    }
    return ret;
  }

  function encodeInt(n: bigint): string {
    if (n === BigInt(0)) {
      return "I!";
    }
    let ret = "";
    while (n > BigInt(0)) {
      ret += String.fromCharCode(Number(n % BigInt(94)) + 33);
      n /= BigInt(94);
    }
    return "I" + ret.split("").reverse().join("");
  }

  function decoder(n: bigint): string {
    const code = encodeInt(n);
    return `B$ L! B$ L$ B$ B$ v! L& L( ? B= v( I! S B$ L) B$ L* B. B$ v& B/ v( I#e B$ B$ v$ v) v* B% B/ v( I% Ia BT I" BD B% v( I% SFL>O ${code} L% B$ v! L& L' ? B= v' I! S B. B$ v& B- v' I" v% L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v#`;
  }

  const N_LENGTH_BITS = 6;
  const runs = runLength(solution);
  const splitRuns = splitTooLargeRuns(runs, N_LENGTH_BITS);
  const encoded = runsToInt(splitRuns, N_LENGTH_BITS);

  const prefix = encodeString(`solve ${course}${level} `);
  const code = `B. ${prefix} ${decoder(encoded)}`;
  console.log(code);
  return code;
}

(async () => {
  const courses = { lambdaman: 21, spaceship: 25 };
  // const courses = { spaceship: 25 };
  // const courses = { lambdaman: 21 };
  for (const [course, levels] of Object.entries(courses)) {
    for (let i = 1; i <= levels; i++) {
      console.log(`submit: ${course}${i}`);
      try {
        const run = await prisma.run.findFirst({
          where: {
            course,
            level: i,
            score: { not: null },
          },
          orderBy: {
            score: "asc",
          },
        });

        if (!run) {
          throw `failed to fetch run`;
        }

        const obj = await s3.send(
          new GetObjectCommand({
            Bucket: process.env.BUCKET!,
            Key: `runs/${run.id}/solution`,
          })
        );

        if (!obj.Body) {
          throw `failed to fetch body`;
        }

        const solution = await obj.Body.transformToString();
        // console.log({ solution });

        const res = await client.send(`solve ${course}${i} ${solution}`);
        console.log(res);
        await new Promise((r) => setTimeout(r, 3000));

        if (course === "lambdaman") {
          console.log("run length encoding...");
          console.log(
            await client.sendRaw(compress(`solve ${course}${i} ${solution}`))
          );
          await new Promise((r) => setTimeout(r, 3000));

          console.log("int pack...");
          console.log(await client.sendRaw(packInt(course, i, solution)));
          await new Promise((r) => setTimeout(r, 3000));

          console.log("int rlpack...");
          console.log(await client.sendRaw(rlpack(course, i, solution)));
          await new Promise((r) => setTimeout(r, 3000));
        }
      } catch (e) {
        console.error(course, i, e);
      }
    }
  }
})();

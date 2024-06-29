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
};

(async () => {
  // const courses = { lambdaman: 21, spaceship: 25 };
  const courses = { spaceship: 25 };
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
        const res = await client.send(`solve ${course}${i} ${solution}`);
        console.log(res);

        await new Promise((r) => setTimeout(r, 3000));
      } catch (e) {
        console.error(course, i, e);
      }
    }
  }
})();

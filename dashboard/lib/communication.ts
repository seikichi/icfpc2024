import { Result, err, ok } from "@/lib/result";
import * as icfp from "@/lib/icfp";

const URL = "https://boundvariable.space/communicate";
const API_TOKEN = process.env.API_TOKEN!;

export const client = {
  async send(
    body: string
  ): Promise<Result<{ raw: string; evaluated: string | null }, string>> {
    const res = await fetch(URL, {
      method: "POST",
      headers: { Authorization: `Bearer ${API_TOKEN}` },
      body: icfp.encodeString(body),
    });
    if (!res.ok) {
      return err(await res.text());
    }
    const raw = await res.text();
    const evaluated = icfp.decodeString(raw);
    return ok({ raw, evaluated });
  },
};

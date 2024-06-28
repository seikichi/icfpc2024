"use server";

import "server-only";
import { client } from "@/lib/communication";

export async function communicate(message: string) {
  return await client.send(message);
}

"use server";

import { Result, ok, err } from "@/lib/result";
import { GetObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";

export async function generateSolutionUrl(
  runId: number
): Promise<Result<string, string>> {
  const s3 = new S3Client({ region: "ap-northeast-1" });
  console.log({ bucket: process.env.BUCKET! });
  const command = new GetObjectCommand({
    Bucket: process.env.BUCKET!,
    Key: `runs/${runId}/solution`,
  });
  try {
    return ok(await getSignedUrl(s3, command, { expiresIn: 15 * 60 }));
  } catch (e) {
    console.log(e);
    return err(e instanceof Error ? e.message : "Unknown error");
  }
}

import { z } from "zod";

const Env = z.object({
  AUTH_PASSWORD: z.string().min(1),
  AUTH_USER: z.string().min(1),
  POSTGRES_PRISMA_URL: z.string().startsWith("postgres://"),
  POSTGRES_URL_NON_POOLING: z.string().startsWith("postgres://"),
  API_TOKEN: z.string().min(1),
  EXPERIMENT_LAMBDA_ARN: z.string().startsWith("arn:aws:lambda"),
  AWS_ACCESS_KEY_ID: z.string().startsWith("AKIA"),
  AWS_SECRET_ACCESS_KEY: z.string().min(1),
  AWS_DEFAULT_REGION: z.string().min(1),
});

export function load() {
  return Env.parse(process.env);
}

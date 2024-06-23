// https://github.com/vercel/examples/blob/main/edge-middleware/basic-auth-password/pages/api/auth.ts
export async function GET() {
  return new Response("Auth Required.", {
    status: 401,
    headers: { "WWW-authenticate": 'Basic realm="Secure Area"' },
  });
}

import { client } from "@/lib/communication";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";

export const dynamic = "force-dynamic";

export default async function Page({ params }: { params: { page: string } }) {
  const res = await client.send(`get ${params.page}`);
  if (!res.ok) {
    return <code>res.error</code>;
  }
  if (!res.value.evaluated) {
    return <code>{res.value.raw}</code>;
  }

  const source = res.value.evaluated.replace(
    /\[([^\]]+)\]/g,
    "[$1](/browse/$1)"
  );
  return <Markdown remarkPlugins={[remarkGfm]}>{source}</Markdown>;
}

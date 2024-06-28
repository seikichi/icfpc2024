import { client } from "@/lib/communication";
import Markdown from "react-markdown";

export const dynamic = "force-dynamic";

export default async function Page({ params }: { params: { page: string } }) {
  const res = await client.send(`get ${params.page}`);
  if (!res.ok) {
    return <code>res.error</code>;
  }
  if (!res.value.evaluated) {
    return <code>{res.value.raw}</code>;
  }
  // return <code style={{ whiteSpace: "pre-wrap" }}>{res.value.evaluated}</code>;
  const source = res.value.evaluated.replace(
    /\[([^\]]+)\]/g,
    "[$1](/communicate/$1)"
  );
  console.log(source);
  return <Markdown>{source}</Markdown>;
}

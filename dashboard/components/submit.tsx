"use client";

import { useFormStatus } from "react-dom";

// NOTE: わざわざコンポーネント別にしないで済む方法ないんだっけ... (useFormStatus)
export function SubmitButton(params: { children: React.ReactNode }) {
  const { pending } = useFormStatus();
  return <button disabled={pending}>{params.children}</button>;
}

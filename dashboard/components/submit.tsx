"use client";

import { ButtonHTMLAttributes } from "react";
import { useFormStatus } from "react-dom";

// NOTE: わざわざコンポーネント別にしないで済む方法ないんだっけ... (useFormStatus)
export function SubmitButton(props: ButtonHTMLAttributes<HTMLButtonElement>) {
  const { pending } = useFormStatus();
  return <button {...props} disabled={props.disabled || pending}></button>;
}

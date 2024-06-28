export type Result<S, E> = { ok: true; value: S } | { ok: false; error: E };

export function ok<S, E>(value: S): Result<S, E> {
  return { ok: true, value };
}

export function err<S, E>(error: E): Result<S, E> {
  return { ok: false, error };
}

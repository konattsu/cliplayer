export type Result<T, E> = { isOk: true; val: T } | { isOk: false; err: E };

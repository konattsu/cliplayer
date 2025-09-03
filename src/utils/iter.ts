export function getAt<T>(arr: T[], idx: number): T | undefined {
  return idx >= 0 && idx < arr.length ? arr[idx] : undefined;
}

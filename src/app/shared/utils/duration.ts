export function formatDuration(totalSecs: number): string {
  const normalizedSecs = Math.max(0, Math.floor(totalSecs));
  const mins = Math.floor(normalizedSecs / 60);
  const secs = normalizedSecs % 60;
  return `${mins}:${secs.toString().padStart(2, "0")}`;
}

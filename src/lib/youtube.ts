import type { VideoId } from "@/types/youtube";

export function createYouTubeUrl(videoId: VideoId): string {
  return `https://www.youtube.com/watch?v=${videoId}`;
}

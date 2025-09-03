import { ofetch } from "ofetch";

import type { Clip } from "@/types/music";
import type { ZodUUID } from "zod";

let clipsCache: Record<string, Clip> | null = null;

export async function fetchClipByUuid(
  uuid: ZodUUID,
): Promise<Clip | undefined> {
  if (clipsCache === null) {
    clipsCache = await ofetch<Record<string, Clip>>(
      "/music_data/clips.min.json",
    );
  }
  return clipsCache[uuid as unknown as string];
}

import { z } from "zod";

const iso8601DurationOnlyHMSRegex = /^PT(?:(\d+H)?(\d+M)?(\d+S)?)$/;

export const MusicSchema = z.object({
  id: z.string().regex(/^[a-zA-Z0-9_-]{11}$/, "invalid YouTube video ID"),
  title: z.string().min(1, "title cannot be empty"),
  artists: z.array(z.string()).nonempty("artists must be a non-empty array"),
  startTime: z
    .string()
    .regex(iso8601DurationOnlyHMSRegex, "startTime must be ISO 8601 duration")
    .optional(),
  endTime: z
    .string()
    .regex(iso8601DurationOnlyHMSRegex, "endTime must be ISO 8601 duration")
    .optional(),
});

export const PlaylistSchema = z.object({
  items: z
    .array(MusicSchema)
    .nonempty("playlist must contain at least one item"),
});

export type Music = z.infer<typeof MusicSchema>;
export type Playlist = z.infer<typeof PlaylistSchema>;

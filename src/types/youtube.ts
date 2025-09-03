import { z } from "zod";

export const VideoIdSchema = z.string().regex(/^[a-zA-Z0-9_-]{11}$/);
export type VideoId = z.infer<typeof VideoIdSchema>;

export const ChannelIdSchema = z.string().regex(/^UC[a-zA-Z0-9_-]{22}$/);
export type ChannelId = z.infer<typeof ChannelIdSchema>;

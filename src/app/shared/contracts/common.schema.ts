import { z } from "zod";

export const LocaleSchema = z.enum(["ja", "en"]);
export const DatasetBuildIdSchema = z.string().min(1);
export const ClipUuidSchema = z.string().min(1);
export const VideoIdSchema = z.string().min(1);
export const ArtistIdSchema = z.string().min(1);
export const TagIdSchema = z.string().min(1);
export const ChannelIdSchema = z.string().min(1);
export const TriStateSchema = z.enum(["any", "only", "exclude"]);
export const SearchSortSchema = z.enum([
  "published_at_desc",
  "published_at_asc",
]);

export type Locale = z.infer<typeof LocaleSchema>;
export type DatasetBuildId = z.infer<typeof DatasetBuildIdSchema>;
export type ClipUuid = z.infer<typeof ClipUuidSchema>;
export type VideoId = z.infer<typeof VideoIdSchema>;
export type ArtistId = z.infer<typeof ArtistIdSchema>;
export type TagId = z.infer<typeof TagIdSchema>;
export type ChannelId = z.infer<typeof ChannelIdSchema>;
export type TriState = z.infer<typeof TriStateSchema>;
export type SearchSort = z.infer<typeof SearchSortSchema>;

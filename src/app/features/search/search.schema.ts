import { z } from "zod";

import {
  ArtistIdSchema,
  ChannelIdSchema,
  SearchSortSchema,
  TagIdSchema,
  TriStateSchema,
} from "../../shared/contracts/common.schema";
import {
  ClipCardViewSchema,
  TagEntitySchema,
} from "../../shared/contracts/dataset.schema";

export const SearchUiStateSchema = z.object({
  artistIds: z.array(ArtistIdSchema),
  artistInput: z.string(),
  tagIds: z.array(TagIdSchema),
  channelIds: z.array(ChannelIdSchema),
  embeddable: TriStateSchema,
  isUnlisted: TriStateSchema,
  publishedFrom: z.string().nullable(),
  publishedTo: z.string().nullable(),
  sort: SearchSortSchema,
  pageCursor: z.string().nullable(),
});

export const SearchJoinedResultSchema = z.object({
  clipCards: z.array(ClipCardViewSchema),
  nextCursor: z.string().nullable(),
  totalMode: z.enum(["none", "exact"]),
  total: z.number().int().nullable(),
  hasMore: z.boolean(),
  warnings: z.array(z.string()),
});

export const ChannelOptionSchema = z.object({
  channelId: ChannelIdSchema,
  label: z.string(),
  matchedKey: z.string(),
});

export const SearchArtistSuggestionSchema = z.object({
  artistId: ArtistIdSchema,
  label: z.string(),
  matchedKey: z.string(),
});

export const SearchCanonicalStateSchema = z.object({
  artistIds: z.array(ArtistIdSchema),
  tagIds: z.array(TagIdSchema),
  channelIds: z.array(ChannelIdSchema),
  embeddable: TriStateSchema,
  isUnlisted: TriStateSchema,
  publishedFromEpochSec: z.number().int().nullable(),
  publishedToEpochSecExclusive: z.number().int().nullable(),
  sort: z.object({
    field: z.literal("published_at"),
    order: z.enum(["asc", "desc"]),
  }),
  pageCursor: z.string().nullable(),
  fingerprint: z.string(),
});

export const SearchTagOptionSchema = TagEntitySchema;

export type SearchUiState = z.infer<typeof SearchUiStateSchema>;
export type SearchJoinedResult = z.infer<typeof SearchJoinedResultSchema>;
export type ChannelOption = z.infer<typeof ChannelOptionSchema>;
export type SearchArtistSuggestion = z.infer<
  typeof SearchArtistSuggestionSchema
>;
export type SearchCanonicalState = z.infer<typeof SearchCanonicalStateSchema>;

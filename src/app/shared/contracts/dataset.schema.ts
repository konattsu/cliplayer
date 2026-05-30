import { z } from "zod";

import {
  ArtistIdSchema,
  ChannelIdSchema,
  ClipUuidSchema,
  DatasetBuildIdSchema,
  TagIdSchema,
  VideoIdSchema,
} from "./common.schema";

export const DatasetAssetDescriptorSchema = z.object({
  assetKind: z.enum([
    "livers",
    "livers_search_index",
    "official_channels",
    "channels",
    "tags",
    "clips",
    "videos",
  ]),
  url: z.string().min(1),
  assetSchemaVersion: z.number().int(),
  sha256: z.string().min(1).optional(),
});

export const SearchIndexAssetDescriptorSchema = z.object({
  assetKind: z.literal("search_index"),
  url: z.string().min(1),
  formatVersion: z.number().int(),
  sha256: z.string().min(1).optional(),
});

export const DatasetManifestSchema = z.object({
  datasetBuildId: DatasetBuildIdSchema,
  datasetSchemaVersion: z.number().int(),
  basePath: z.string(),
  assets: z.object({
    livers: DatasetAssetDescriptorSchema,
    liversSearchIndex: DatasetAssetDescriptorSchema,
    officialChannels: DatasetAssetDescriptorSchema,
    channels: DatasetAssetDescriptorSchema,
    tags: DatasetAssetDescriptorSchema,
    clips: DatasetAssetDescriptorSchema,
    videos: DatasetAssetDescriptorSchema,
    searchIndex: SearchIndexAssetDescriptorSchema,
  }),
});

export const LiverMinSchema = z.object({
  ja: z.string(),
  jah: z.string(),
  en: z.string(),
  color: z.string(),
  channelId: ChannelIdSchema,
  isGraduated: z.literal(true).optional(),
});

export const OfficialChannelMinSchema = z
  .object({
    ja: z.string(),
    jah: z.string(),
    en: z.string(),
    channelId: ChannelIdSchema.optional(),
    channel_id: ChannelIdSchema.optional(),
  })
  .transform((value) => ({
    ja: value.ja,
    jah: value.jah,
    en: value.en,
    channelId: value.channelId ?? value.channel_id ?? "",
  }))
  .pipe(
    z.object({
      ja: z.string(),
      jah: z.string(),
      en: z.string(),
      channelId: ChannelIdSchema,
    }),
  );

export const ChannelOwnerMinSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("liver"),
    id: ArtistIdSchema,
  }),
  z.object({
    kind: z.literal("official"),
    id: z.string().min(1),
  }),
]);

export const TagMinSchema = z.object({
  ja: z.string(),
  en: z.string(),
  intId: z.number().int(),
  blocked: z.literal(true).optional(),
});

export const LiverSearchIndexEntryMinSchema = z.object({
  key: z.string(),
  liverId: ArtistIdSchema,
  isAlias: z.literal(true).optional(),
});

export const ClipMinSchema = z.object({
  videoId: VideoIdSchema,
  startTimeSecs: z.number().int(),
  endTimeSecs: z.number().int(),
  songTitle: z.string(),
  liverIds: z.array(ArtistIdSchema),
  externalArtistsName: z.array(z.string()).optional(),
  clippedVideoId: VideoIdSchema.optional(),
});

export const VideoMinSchema = z.object({
  clipUuids: z.array(ClipUuidSchema),
  durationSecs: z.number().int(),
  title: z.string(),
  channelId: ChannelIdSchema,
  uploaderName: z.string().optional(),
  publishedAt: z.string().optional(),
  publishedAtIso: z.string().optional(),
  publishedAtEpochSec: z.number().int().optional(),
  syncedAt: z.string().optional(),
  syncedAtIso: z.string().optional(),
  syncedAtEpochSec: z.number().int().optional(),
  privacyStatus: z.enum(["public", "unlisted", "private"]),
  embeddable: z.boolean(),
  videoTags: z.array(TagIdSchema),
});

export const LiversMinDataSchema = z.record(ArtistIdSchema, LiverMinSchema);
export const LiversSearchIndexMinDataSchema = z.array(
  LiverSearchIndexEntryMinSchema,
);
export const OfficialChannelsMinDataSchema = z.record(
  z.string().min(1),
  OfficialChannelMinSchema,
);
export const ChannelsMinDataSchema = z.record(
  ChannelIdSchema,
  ChannelOwnerMinSchema,
);
export const TagsMinDataSchema = z.record(TagIdSchema, TagMinSchema);
export const ClipsMinDataSchema = z.record(ClipUuidSchema, ClipMinSchema);
export const VideosMinDataSchema = z.record(VideoIdSchema, VideoMinSchema);

export const createMinEnvelopeSchema = <T extends z.ZodTypeAny>(
  dataSchema: T,
): z.ZodObject<{
  schemaVersion: z.ZodNumber;
  datasetBuildId: typeof DatasetBuildIdSchema;
  data: T;
}> =>
  z.object({
    schemaVersion: z.number().int(),
    datasetBuildId: DatasetBuildIdSchema,
    data: dataSchema,
  });

export const DatasetMetaSchema = z.object({
  schemaVersion: z.number().int(),
  datasetBuildId: DatasetBuildIdSchema,
  searchIndexFormatVersion: z.number().int(),
});

export const ArtistEntitySchema = z.object({
  artistId: ArtistIdSchema,
  nameJa: z.string(),
  nameJaHiragana: z.string(),
  nameEn: z.string(),
  colorHex: z.string(),
  channelId: ChannelIdSchema,
  isGraduated: z.boolean(),
});

export const LiverSearchEntrySchema = z.object({
  key: z.string(),
  normalizedKey: z.string(),
  artistId: ArtistIdSchema,
  isAlias: z.boolean(),
});

export const ArtistSearchIndexSchema = z.object({
  entries: z.array(LiverSearchEntrySchema),
});

export const OfficialChannelEntitySchema = z.object({
  officialChannelId: z.string().min(1),
  nameJa: z.string(),
  nameJaHiragana: z.string(),
  nameEn: z.string(),
  channelId: ChannelIdSchema,
});

export const ChannelOwnerEntitySchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("liver"),
    channelId: ChannelIdSchema,
    artistId: ArtistIdSchema,
  }),
  z.object({
    kind: z.literal("official"),
    channelId: ChannelIdSchema,
    officialChannelId: z.string().min(1),
  }),
  z.object({
    kind: z.literal("external"),
    channelId: ChannelIdSchema,
  }),
]);

export const TagEntitySchema = z.object({
  tagId: TagIdSchema,
  nameJa: z.string(),
  nameEn: z.string(),
  intId: z.number().int(),
  blocked: z.boolean(),
});

export const ClipEntitySchema = z.object({
  clipUuid: ClipUuidSchema,
  videoId: VideoIdSchema,
  videoClipIndex: z.number().int(),
  startTimeSecs: z.number().int(),
  endTimeSecs: z.number().int(),
  durationSecs: z.number().int(),
  songTitle: z.string(),
  liverIds: z.array(ArtistIdSchema),
  externalArtistsName: z.array(z.string()),
  clippedVideoId: VideoIdSchema.nullable(),
});

export const VideoEntitySchema = z.object({
  videoId: VideoIdSchema,
  clipUuids: z.array(ClipUuidSchema),
  durationSecs: z.number().int(),
  title: z.string(),
  channelId: ChannelIdSchema,
  uploaderName: z.string(),
  publishedAtIso: z.string(),
  publishedAtEpochSec: z.number().int(),
  syncedAtIso: z.string(),
  syncedAtEpochSec: z.number().int(),
  privacyStatus: z.enum(["public", "unlisted", "private"]),
  embeddable: z.boolean(),
  videoTags: z.array(TagIdSchema),
});

export const AppDatasetSchema = z.object({
  meta: DatasetMetaSchema,
  artists: z.record(ArtistIdSchema, ArtistEntitySchema),
  artistSearchIndex: ArtistSearchIndexSchema,
  officialChannels: z.record(z.string().min(1), OfficialChannelEntitySchema),
  channels: z.record(ChannelIdSchema, ChannelOwnerEntitySchema),
  tags: z.record(TagIdSchema, TagEntitySchema),
  clips: z.record(ClipUuidSchema, ClipEntitySchema),
  videos: z.record(VideoIdSchema, VideoEntitySchema),
  clipOrderByVideo: z.record(VideoIdSchema, z.array(ClipUuidSchema)),
});

export const ClipPlayableStateSchema = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("playable") }),
  z.object({ kind: z.literal("blocked_tag"), tagId: TagIdSchema }),
  z.object({ kind: z.literal("not_embeddable") }),
  z.object({ kind: z.literal("privacy_private") }),
  z.object({ kind: z.literal("missing_video") }),
  z.object({ kind: z.literal("invalid_clip_range") }),
  z.object({
    kind: z.literal("youtube_runtime_error"),
    code: z.string(),
  }),
]);

export const ClipArtistViewSchema = z.object({
  artistId: ArtistIdSchema,
  nameJa: z.string(),
  nameEn: z.string(),
  colorHex: z.string(),
});

export const ClipVideoViewSchema = z.object({
  videoId: VideoIdSchema,
  title: z.string(),
  publishedAtIso: z.string(),
  channelId: ChannelIdSchema,
  channelLabel: z.string(),
  embeddable: z.boolean(),
  privacyStatus: z.enum(["public", "unlisted", "private"]),
});

export const ClipTagViewSchema = z.object({
  tagId: TagIdSchema,
  nameJa: z.string(),
  nameEn: z.string(),
  blocked: z.boolean(),
});

export const ClipCardViewSchema = z.object({
  clipUuid: ClipUuidSchema,
  songTitle: z.string(),
  artists: z.array(ClipArtistViewSchema),
  externalArtistsName: z.array(z.string()),
  video: ClipVideoViewSchema,
  tags: z.array(ClipTagViewSchema),
  startTimeSecs: z.number().int(),
  endTimeSecs: z.number().int(),
  durationSecs: z.number().int(),
  playableState: ClipPlayableStateSchema,
  siblingClipUuids: z.array(ClipUuidSchema),
});

export type DatasetManifest = z.infer<typeof DatasetManifestSchema>;
export type LiversMinData = z.infer<typeof LiversMinDataSchema>;
export type LiversSearchIndexMinData = z.infer<
  typeof LiversSearchIndexMinDataSchema
>;
export type OfficialChannelsMinData = z.infer<
  typeof OfficialChannelsMinDataSchema
>;
export type ChannelsMinData = z.infer<typeof ChannelsMinDataSchema>;
export type TagsMinData = z.infer<typeof TagsMinDataSchema>;
export type ClipsMinData = z.infer<typeof ClipsMinDataSchema>;
export type VideosMinData = z.infer<typeof VideosMinDataSchema>;
export type ArtistEntity = z.infer<typeof ArtistEntitySchema>;
export type LiverSearchEntry = z.infer<typeof LiverSearchEntrySchema>;
export type OfficialChannelEntity = z.infer<typeof OfficialChannelEntitySchema>;
export type ChannelOwnerEntity = z.infer<typeof ChannelOwnerEntitySchema>;
export type TagEntity = z.infer<typeof TagEntitySchema>;
export type ClipEntity = z.infer<typeof ClipEntitySchema>;
export type VideoEntity = z.infer<typeof VideoEntitySchema>;
export type AppDataset = z.infer<typeof AppDatasetSchema>;
export type ClipCardView = z.infer<typeof ClipCardViewSchema>;

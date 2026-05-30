import { Injectable, signal } from "@angular/core";
import { z } from "zod";

import {
  SearchArtistSuggestionSchema,
  SearchCanonicalStateSchema,
  SearchJoinedResultSchema,
} from "../../features/search/search.schema";
import {
  AppDatasetSchema,
  ChannelsMinDataSchema,
  ClipsMinDataSchema,
  createMinEnvelopeSchema,
  DatasetManifestSchema,
  LiversMinDataSchema,
  LiversSearchIndexMinDataSchema,
  OfficialChannelsMinDataSchema,
  TagsMinDataSchema,
  VideosMinDataSchema,
} from "../contracts/dataset.schema";

import type {
  SearchArtistSuggestion,
  SearchCanonicalState,
  SearchJoinedResult,
  SearchUiState,
} from "../../features/search/search.schema";
import type { Locale } from "../contracts/common.schema";
import type { ChannelId, ClipUuid } from "../contracts/common.schema";
import type {
  AppDataset,
  ArtistEntity,
  ChannelOwnerEntity,
  ClipCardView,
  ClipEntity,
  LiverSearchEntry,
  OfficialChannelEntity,
  TagEntity,
  VideoEntity,
} from "../contracts/dataset.schema";

const AppLoadStateSchema = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("booting") }),
  z.object({ kind: z.literal("loading_assets") }),
  z.object({ kind: z.literal("ready"), dataset: AppDatasetSchema }),
  z.object({ kind: z.literal("fatal_error"), error: z.string() }),
]);

const DatasetAssetUrlsSchema = z.object({
  livers: z.string(),
  liversSearchIndex: z.string(),
  officialChannels: z.string(),
  channels: z.string(),
  tags: z.string(),
  clips: z.string(),
  videos: z.string(),
  searchIndexFormatVersion: z.number().int(),
});

export type AppLoadState = z.infer<typeof AppLoadStateSchema>;
type DatasetAssetUrls = z.infer<typeof DatasetAssetUrlsSchema>;

const LiversEnvelopeSchema = createMinEnvelopeSchema(LiversMinDataSchema);
const LiversSearchIndexEnvelopeSchema = createMinEnvelopeSchema(
  LiversSearchIndexMinDataSchema,
);
const OfficialChannelsEnvelopeSchema = createMinEnvelopeSchema(
  OfficialChannelsMinDataSchema,
);
const ChannelsEnvelopeSchema = createMinEnvelopeSchema(ChannelsMinDataSchema);
const TagsEnvelopeSchema = createMinEnvelopeSchema(TagsMinDataSchema);
const ClipsEnvelopeSchema = createMinEnvelopeSchema(ClipsMinDataSchema);
const VideosEnvelopeSchema = createMinEnvelopeSchema(VideosMinDataSchema);

const FALLBACK_ASSETS = DatasetAssetUrlsSchema.parse({
  livers: "/music/livers.min.json",
  liversSearchIndex: "/music/livers_search_index.min.json",
  officialChannels: "/music/official_channels.min.json",
  channels: "/music/channels.min.json",
  tags: "/music/tags.min.json",
  clips: "/music/clips.min.json",
  videos: "/music/videos.min.json",
  searchIndexFormatVersion: 1,
});

const PAGE_SIZE = 50;

@Injectable({ providedIn: "root" })
export class DatasetService {
  private loadPromise: Promise<void> | null = null;

  readonly state = signal<AppLoadState>(
    AppLoadStateSchema.parse({ kind: "booting" }),
  );

  async load(): Promise<void> {
    const current = this.state();
    if (current.kind === "ready") {
      return;
    }
    if (this.loadPromise !== null) {
      return this.loadPromise;
    }

    this.loadPromise = this.loadInternal();
    try {
      await this.loadPromise;
    } finally {
      this.loadPromise = null;
    }
  }

  private async loadInternal(): Promise<void> {
    this.state.set(AppLoadStateSchema.parse({ kind: "loading_assets" }));

    try {
      const urls = await this.resolveAssetUrls();
      const [
        livers,
        liversSearchIndex,
        officialChannels,
        channels,
        tags,
        clips,
        videos,
      ] = await Promise.all([
        this.fetchEnvelope(urls.livers, LiversEnvelopeSchema),
        this.fetchEnvelope(
          urls.liversSearchIndex,
          LiversSearchIndexEnvelopeSchema,
        ),
        this.fetchEnvelope(
          urls.officialChannels,
          OfficialChannelsEnvelopeSchema,
        ),
        this.fetchEnvelope(urls.channels, ChannelsEnvelopeSchema),
        this.fetchEnvelope(urls.tags, TagsEnvelopeSchema),
        this.fetchEnvelope(urls.clips, ClipsEnvelopeSchema),
        this.fetchEnvelope(urls.videos, VideosEnvelopeSchema),
      ]);

      const dataset = this.buildDataset(
        livers,
        liversSearchIndex,
        officialChannels,
        channels,
        tags,
        clips,
        videos,
        urls.searchIndexFormatVersion,
      );

      this.state.set(AppLoadStateSchema.parse({ kind: "ready", dataset }));
    } catch (error) {
      console.error("Failed to load dataset", error);
      this.state.set(
        AppLoadStateSchema.parse({
          kind: "fatal_error",
          error: error instanceof Error ? error.message : "Unknown load error",
        }),
      );
    }
  }

  joinClip(
    dataset: AppDataset,
    clipUuid: ClipUuid,
    locale: Locale = "ja",
  ): ClipCardView | null {
    const clip = dataset.clips[clipUuid];
    if (clip === undefined) {
      return null;
    }

    const video = dataset.videos[clip.videoId];
    if (video === undefined) {
      return null;
    }

    const tags = video.videoTags
      .map((tagId) => dataset.tags[tagId])
      .filter((tag): tag is TagEntity => tag !== undefined);
    const blockedTag = tags.find((tag) => tag.blocked);

    return {
      clipUuid,
      songTitle: clip.songTitle,
      artists: clip.liverIds
        .map((artistId) => dataset.artists[artistId])
        .filter((artist): artist is ArtistEntity => artist !== undefined)
        .map((artist) => ({
          artistId: artist.artistId,
          nameJa: artist.nameJa,
          nameEn: artist.nameEn,
          colorHex: artist.colorHex,
        })),
      externalArtistsName: clip.externalArtistsName,
      video: {
        videoId: video.videoId,
        title: video.title,
        publishedAtIso: video.publishedAtIso,
        channelId: video.channelId,
        channelLabel: this.channelLabel(dataset, video, locale),
        embeddable: video.embeddable,
        privacyStatus: video.privacyStatus,
      },
      tags: tags.map((tag) => ({
        tagId: tag.tagId,
        nameJa: tag.nameJa,
        nameEn: tag.nameEn,
        blocked: tag.blocked,
      })),
      startTimeSecs: clip.startTimeSecs,
      endTimeSecs: clip.endTimeSecs,
      durationSecs: clip.durationSecs,
      playableState:
        blockedTag !== undefined
          ? { kind: "blocked_tag", tagId: blockedTag.tagId }
          : video.privacyStatus === "private"
            ? { kind: "privacy_private" }
            : !video.embeddable
              ? { kind: "not_embeddable" }
              : clip.startTimeSecs >= clip.endTimeSecs ||
                  clip.endTimeSecs > video.durationSecs
                ? { kind: "invalid_clip_range" }
                : { kind: "playable" },
      siblingClipUuids: video.clipUuids.filter((uuid) => uuid !== clipUuid),
    };
  }

  search(dataset: AppDataset, ui: SearchUiState): SearchJoinedResult {
    const canonical = this.canonicalizeSearch(dataset, ui);
    const offset =
      canonical.pageCursor === null ? 0 : Number(canonical.pageCursor);
    const safeOffset = Number.isInteger(offset) && offset > 0 ? offset : 0;

    const matchingClipUuids = Object.values(dataset.clips)
      .filter((clip) => this.matchesSearch(dataset, clip, canonical))
      .sort((left, right) => {
        const leftVideo = dataset.videos[left.videoId];
        const rightVideo = dataset.videos[right.videoId];
        const leftPublished = leftVideo?.publishedAtEpochSec ?? 0;
        const rightPublished = rightVideo?.publishedAtEpochSec ?? 0;
        const direction = canonical.sort.order === "asc" ? 1 : -1;
        const byDate = (leftPublished - rightPublished) * direction;
        return byDate === 0 ? left.startTimeSecs - right.startTimeSecs : byDate;
      })
      .map((clip) => clip.clipUuid);

    const pageClipUuids = matchingClipUuids.slice(
      safeOffset,
      safeOffset + PAGE_SIZE,
    );
    const nextOffset = safeOffset + PAGE_SIZE;

    return SearchJoinedResultSchema.parse({
      clipCards: pageClipUuids
        .map((clipUuid) => this.joinClip(dataset, clipUuid))
        .filter((clip): clip is ClipCardView => clip !== null),
      nextCursor:
        nextOffset < matchingClipUuids.length ? String(nextOffset) : null,
      totalMode: "exact",
      total: matchingClipUuids.length,
      hasMore: nextOffset < matchingClipUuids.length,
      warnings: [],
    });
  }

  suggestArtists(dataset: AppDataset, query: string): SearchArtistSuggestion[] {
    const normalizedQuery = normalizeSearchText(query);
    if (normalizedQuery.length === 0) {
      return [];
    }

    const scored = dataset.artistSearchIndex.entries
      .map((entry) => ({
        entry,
        score: scoreArtistEntry(entry, normalizedQuery),
      }))
      .filter((item) => item.score > 0)
      .sort((left, right) => {
        const byScore = right.score - left.score;
        if (byScore !== 0) {
          return byScore;
        }

        return Number(left.entry.isAlias) - Number(right.entry.isAlias);
      });

    const seen = new Set<string>();
    const suggestions: SearchArtistSuggestion[] = [];
    for (const item of scored) {
      if (seen.has(item.entry.artistId)) {
        continue;
      }

      const artist = dataset.artists[item.entry.artistId];
      if (artist === undefined) {
        continue;
      }

      suggestions.push(
        SearchArtistSuggestionSchema.parse({
          artistId: artist.artistId,
          label: artist.nameJa,
          matchedKey: item.entry.key,
        }),
      );
      seen.add(item.entry.artistId);

      if (suggestions.length >= 8) {
        break;
      }
    }

    return suggestions;
  }

  suggestChannels(
    dataset: AppDataset,
    query: string,
    locale: Locale,
  ): Array<{ channelId: ChannelId; label: string; matchedKey: string }> {
    const normalizedQuery = normalizeSearchText(query);
    if (normalizedQuery.length === 0) {
      return [];
    }

    const suggestions = new Map<
      ChannelId,
      { channelId: ChannelId; label: string; matchedKey: string; score: number }
    >();

    for (const item of dataset.artistSearchIndex.entries
      .map((entry) => ({
        entry,
        score: scoreArtistEntry(entry, normalizedQuery),
      }))
      .filter((candidate) => candidate.score > 0)
      .sort((left, right) => right.score - left.score)) {
      const artist = dataset.artists[item.entry.artistId];
      if (artist === undefined) {
        continue;
      }

      const current = suggestions.get(artist.channelId);
      if (current === undefined || item.score > current.score) {
        suggestions.set(artist.channelId, {
          channelId: artist.channelId,
          label: locale === "en" ? artist.nameEn : artist.nameJa,
          matchedKey: item.entry.key,
          score: item.score,
        });
      }
    }

    for (const official of Object.values(dataset.officialChannels)) {
      const label = locale === "en" ? official.nameEn : official.nameJa;
      const score = Math.max(
        scorePlainText(label, normalizedQuery),
        scorePlainText(official.nameJaHiragana, normalizedQuery),
      );
      if (score === 0) {
        continue;
      }

      suggestions.set(official.channelId, {
        channelId: official.channelId,
        label,
        matchedKey: label,
        score,
      });
    }

    for (const video of Object.values(dataset.videos)) {
      const label = this.channelLabel(dataset, video, locale);
      const score = scorePlainText(label, normalizedQuery);
      if (score === 0 || suggestions.has(video.channelId)) {
        continue;
      }

      suggestions.set(video.channelId, {
        channelId: video.channelId,
        label,
        matchedKey: label,
        score,
      });
    }

    return Array.from(suggestions.values())
      .sort((left, right) => {
        const byScore = right.score - left.score;
        return byScore === 0
          ? left.label.localeCompare(right.label, locale)
          : byScore;
      })
      .slice(0, 8)
      .map(({ channelId, label, matchedKey }) => ({
        channelId,
        label,
        matchedKey,
      }));
  }

  channelLabelById(
    dataset: AppDataset,
    channelId: ChannelId,
    locale: Locale,
  ): string {
    const video = Object.values(dataset.videos).find(
      (item) => item.channelId === channelId,
    );
    if (video !== undefined) {
      return this.channelLabel(dataset, video, locale);
    }

    const owner = dataset.channels[channelId];
    if (owner?.kind === "liver") {
      const artist = dataset.artists[owner.artistId];
      return artist === undefined
        ? channelId
        : locale === "en"
          ? artist.nameEn
          : artist.nameJa;
    }
    if (owner?.kind === "official") {
      const official = dataset.officialChannels[owner.officialChannelId];
      return official === undefined
        ? channelId
        : locale === "en"
          ? official.nameEn
          : official.nameJa;
    }

    return channelId;
  }

  private async resolveAssetUrls(): Promise<DatasetAssetUrls> {
    const response = await fetch("/dataset-manifest.json", {
      cache: "no-cache",
    });
    if (!response.ok) {
      return FALLBACK_ASSETS;
    }

    const manifest = DatasetManifestSchema.parse(await response.json());
    return DatasetAssetUrlsSchema.parse({
      livers: manifest.assets.livers.url,
      liversSearchIndex: manifest.assets.liversSearchIndex.url,
      officialChannels: manifest.assets.officialChannels.url,
      channels: manifest.assets.channels.url,
      tags: manifest.assets.tags.url,
      clips: manifest.assets.clips.url,
      videos: manifest.assets.videos.url,
      searchIndexFormatVersion: manifest.assets.searchIndex.formatVersion,
    });
  }

  private async fetchEnvelope<T extends z.ZodTypeAny>(
    url: string,
    schema: T,
  ): Promise<z.infer<T>> {
    const response = await fetch(url, { cache: "no-cache" });
    if (!response.ok) {
      throw new Error(`Failed to load dataset asset: ${url}`);
    }

    return schema.parse(await response.json());
  }

  private buildDataset(
    liversEnvelope: z.infer<typeof LiversEnvelopeSchema>,
    liversSearchIndexEnvelope: z.infer<typeof LiversSearchIndexEnvelopeSchema>,
    officialChannelsEnvelope: z.infer<typeof OfficialChannelsEnvelopeSchema>,
    channelsEnvelope: z.infer<typeof ChannelsEnvelopeSchema>,
    tagsEnvelope: z.infer<typeof TagsEnvelopeSchema>,
    clipsEnvelope: z.infer<typeof ClipsEnvelopeSchema>,
    videosEnvelope: z.infer<typeof VideosEnvelopeSchema>,
    searchIndexFormatVersion: number,
  ): AppDataset {
    const envelopes = [
      liversSearchIndexEnvelope,
      officialChannelsEnvelope,
      channelsEnvelope,
      tagsEnvelope,
      clipsEnvelope,
      videosEnvelope,
    ];
    for (const envelope of envelopes) {
      if (envelope.datasetBuildId !== liversEnvelope.datasetBuildId) {
        throw new Error("Dataset build id mismatch between min JSON assets");
      }
      if (envelope.schemaVersion !== liversEnvelope.schemaVersion) {
        throw new Error(
          "Dataset schema version mismatch between min JSON assets",
        );
      }
    }

    const artists: Record<string, ArtistEntity> = {};
    for (const [artistId, liver] of Object.entries(liversEnvelope.data)) {
      artists[artistId] = {
        artistId,
        nameJa: liver.ja,
        nameJaHiragana: liver.jah,
        nameEn: liver.en,
        colorHex: `#${liver.color}`,
        channelId: liver.channelId,
        isGraduated: liver.isGraduated === true,
      };
    }

    const officialChannels: Record<string, OfficialChannelEntity> = {};
    for (const [officialChannelId, channel] of Object.entries(
      officialChannelsEnvelope.data,
    )) {
      officialChannels[officialChannelId] = {
        officialChannelId,
        nameJa: channel.ja,
        nameJaHiragana: channel.jah,
        nameEn: channel.en,
        channelId: channel.channelId,
      };
    }

    const channels: Record<ChannelId, ChannelOwnerEntity> = {};
    for (const [channelId, owner] of Object.entries(channelsEnvelope.data)) {
      channels[channelId] =
        owner.kind === "liver"
          ? { kind: "liver", channelId, artistId: owner.id }
          : { kind: "official", channelId, officialChannelId: owner.id };
    }

    const tags: Record<string, TagEntity> = {};
    for (const [tagId, tag] of Object.entries(tagsEnvelope.data)) {
      tags[tagId] = {
        tagId,
        nameJa: tag.ja,
        nameEn: tag.en,
        intId: tag.intId,
        blocked: tag.blocked === true,
      };
    }

    const videos: Record<string, VideoEntity> = {};
    for (const [videoId, video] of Object.entries(videosEnvelope.data)) {
      const publishedAtIso = video.publishedAtIso ?? video.publishedAt ?? "";
      const syncedAtIso = video.syncedAtIso ?? video.syncedAt ?? "";
      videos[videoId] = {
        videoId,
        clipUuids: video.clipUuids,
        durationSecs: video.durationSecs,
        title: video.title,
        channelId: video.channelId,
        uploaderName: video.uploaderName ?? "",
        publishedAtIso,
        publishedAtEpochSec:
          video.publishedAtEpochSec ??
          Math.floor(Date.parse(publishedAtIso) / 1000),
        syncedAtIso,
        syncedAtEpochSec:
          video.syncedAtEpochSec ?? Math.floor(Date.parse(syncedAtIso) / 1000),
        privacyStatus: video.privacyStatus,
        embeddable: video.embeddable,
        videoTags: video.videoTags,
      };

      if (channels[video.channelId] === undefined) {
        channels[video.channelId] = {
          kind: "external",
          channelId: video.channelId,
        };
      }
    }

    const clips: Record<string, ClipEntity> = {};
    for (const [clipUuid, clip] of Object.entries(clipsEnvelope.data)) {
      const video = videos[clip.videoId];
      if (video === undefined) {
        throw new Error(`Clip references missing video: ${clipUuid}`);
      }

      const videoClipIndex = video.clipUuids.indexOf(clipUuid);
      if (videoClipIndex < 0) {
        throw new Error(`Video does not reference clip: ${clipUuid}`);
      }

      clips[clipUuid] = {
        clipUuid,
        videoId: clip.videoId,
        videoClipIndex,
        startTimeSecs: clip.startTimeSecs,
        endTimeSecs: clip.endTimeSecs,
        durationSecs: clip.endTimeSecs - clip.startTimeSecs,
        songTitle: clip.songTitle,
        liverIds: clip.liverIds,
        externalArtistsName: clip.externalArtistsName ?? [],
        clippedVideoId: clip.clippedVideoId ?? null,
      };
    }

    const clipOrderByVideo: Record<string, ClipUuid[]> = {};
    for (const [videoId, video] of Object.entries(videos)) {
      clipOrderByVideo[videoId] = video.clipUuids;
    }

    const entries: LiverSearchEntry[] = liversSearchIndexEnvelope.data.map(
      (entry) => ({
        key: entry.key,
        normalizedKey: normalizeSearchText(entry.key),
        artistId: entry.liverId,
        isAlias: entry.isAlias === true,
      }),
    );

    return AppDatasetSchema.parse({
      meta: {
        schemaVersion: liversEnvelope.schemaVersion,
        datasetBuildId: liversEnvelope.datasetBuildId,
        searchIndexFormatVersion,
      },
      artists,
      artistSearchIndex: { entries },
      officialChannels,
      channels,
      tags,
      clips,
      videos,
      clipOrderByVideo,
    });
  }

  private canonicalizeSearch(
    dataset: AppDataset,
    ui: SearchUiState,
  ): SearchCanonicalState {
    const artistIds = normalizeIds(ui.artistIds, dataset.artists);
    const tagIds = normalizeIds(ui.tagIds, dataset.tags);
    const channelIds = normalizeIds(ui.channelIds, dataset.channels);
    const publishedFromEpochSec = dateStartEpoch(ui.publishedFrom);
    const publishedToEpochSecExclusive = dateEndExclusiveEpoch(ui.publishedTo);
    const order = ui.sort === "published_at_asc" ? "asc" : "desc";

    return SearchCanonicalStateSchema.parse({
      artistIds,
      tagIds,
      channelIds,
      embeddable: ui.embeddable,
      isUnlisted: ui.isUnlisted,
      publishedFromEpochSec,
      publishedToEpochSecExclusive,
      sort: { field: "published_at", order },
      pageCursor: ui.pageCursor,
      fingerprint: [
        artistIds.join(","),
        tagIds.join(","),
        channelIds.join(","),
        ui.embeddable,
        ui.isUnlisted,
        publishedFromEpochSec ?? "",
        publishedToEpochSecExclusive ?? "",
        order,
      ].join("|"),
    });
  }

  private matchesSearch(
    dataset: AppDataset,
    clip: ClipEntity,
    search: SearchCanonicalState,
  ): boolean {
    const video = dataset.videos[clip.videoId];
    if (video === undefined) {
      return false;
    }

    if (
      video.videoTags.some((tagId) => dataset.tags[tagId]?.blocked === true)
    ) {
      return false;
    }

    if (
      search.artistIds.length > 0 &&
      !search.artistIds.some((artistId) => clip.liverIds.includes(artistId))
    ) {
      return false;
    }

    if (
      search.tagIds.length > 0 &&
      !search.tagIds.some((tagId) => video.videoTags.includes(tagId))
    ) {
      return false;
    }

    if (
      search.channelIds.length > 0 &&
      !search.channelIds.includes(video.channelId)
    ) {
      return false;
    }

    if (search.embeddable === "only" && !video.embeddable) {
      return false;
    }

    if (search.embeddable === "exclude" && video.embeddable) {
      return false;
    }

    const isUnlisted = video.privacyStatus === "unlisted";
    if (search.isUnlisted === "only" && !isUnlisted) {
      return false;
    }

    if (search.isUnlisted === "exclude" && isUnlisted) {
      return false;
    }

    if (
      search.publishedFromEpochSec !== null &&
      video.publishedAtEpochSec < search.publishedFromEpochSec
    ) {
      return false;
    }

    if (
      search.publishedToEpochSecExclusive !== null &&
      video.publishedAtEpochSec >= search.publishedToEpochSecExclusive
    ) {
      return false;
    }

    return true;
  }

  private channelLabel(
    dataset: AppDataset,
    video: VideoEntity,
    locale: Locale,
  ): string {
    const owner = dataset.channels[video.channelId];
    if (owner?.kind === "liver") {
      const artist = dataset.artists[owner.artistId];
      return artist === undefined
        ? video.channelId
        : locale === "en"
          ? artist.nameEn
          : artist.nameJa;
    }
    if (owner?.kind === "official") {
      const official = dataset.officialChannels[owner.officialChannelId];
      return official === undefined
        ? video.channelId
        : locale === "en"
          ? official.nameEn
          : official.nameJa;
    }

    return video.uploaderName.length > 0 ? video.uploaderName : video.channelId;
  }
}

function normalizeIds<T>(values: string[], known: Record<string, T>): string[] {
  return Array.from(
    new Set(values.filter((value) => known[value] !== undefined)),
  ).sort();
}

function normalizeSearchText(value: string): string {
  return value.normalize("NFKC").trim().toLocaleLowerCase();
}

function dateStartEpoch(value: string | null): number | null {
  if (value === null || value.length === 0) {
    return null;
  }

  const epochMs = Date.parse(`${value}T00:00:00Z`);
  return Number.isNaN(epochMs) ? null : Math.floor(epochMs / 1000);
}

function dateEndExclusiveEpoch(value: string | null): number | null {
  if (value === null || value.length === 0) {
    return null;
  }

  const epochMs = Date.parse(`${value}T00:00:00Z`);
  if (Number.isNaN(epochMs)) {
    return null;
  }

  return Math.floor(epochMs / 1000) + 86_400;
}

function scoreArtistEntry(entry: LiverSearchEntry, query: string): number {
  if (entry.normalizedKey === query) {
    return entry.isAlias ? 80 : 100;
  }

  if (entry.normalizedKey.startsWith(query)) {
    return entry.isAlias ? 60 : 75;
  }

  if (entry.normalizedKey.includes(query)) {
    return entry.isAlias ? 35 : 45;
  }

  return fuzzyIncludes(entry.normalizedKey, query) ? 20 : 0;
}

function scorePlainText(value: string, query: string): number {
  const normalizedValue = normalizeSearchText(value);
  if (normalizedValue === query) {
    return 90;
  }

  if (normalizedValue.startsWith(query)) {
    return 70;
  }

  if (normalizedValue.includes(query)) {
    return 40;
  }

  return fuzzyIncludes(normalizedValue, query) ? 18 : 0;
}

function fuzzyIncludes(target: string, query: string): boolean {
  let targetIndex = 0;
  for (const char of query) {
    targetIndex = target.indexOf(char, targetIndex);
    if (targetIndex < 0) {
      return false;
    }
    targetIndex += 1;
  }

  return true;
}

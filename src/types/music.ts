import { z } from "zod";

import { ArtistIdSchema } from "./artist-id";
import { ChannelIdSchema, VideoIdSchema } from "./youtube";

export const ClipSchema = z
  .object({
    /** このクリップが含まれる動画id */
    videoId: VideoIdSchema,
    /** 楽曲名 */
    songTitle: z.string(),
    /** アーティストIDの配列 */
    artists: ArtistIdSchema.array().nonempty(),
    /** 外部アーティスト名の配列 */
    externalArtists: z.array(z.string().nonempty()).nonempty().optional(),
    /** 切り抜いた動画が存在した場合の動画id */
    clippedVideoId: VideoIdSchema.optional(),
    /** 曲が始まる時間 */
    startTimeSecs: z.number().min(0),
    /** 曲が終わる時間 */
    endTimeSecs: z.number().min(0),
    /** クリップのタグ */
    clipTags: z.array(z.string().nonempty()).nonempty().optional(),
    /** 音量の正規化時に設定すべき音量 */
    volumePercent: z.number().min(1).max(100).optional(),
  })
  .readonly();
export type Clip = z.infer<typeof ClipSchema>;
export const ClipsRecordSchema = z.record(z.uuidv4(), ClipSchema);
export type ClipsRecord = z.infer<typeof ClipsRecordSchema>;

export const VideoSchema = z.object({
  /** この動画が持つクリップの一覧 */
  clipsUuids: z.array(z.uuidv4()).nonempty(),
  /** 少なくとも1回クリップに含まれるアーティスト */
  artists: z.array(ArtistIdSchema).nonempty(),
  /** 動画の長さ */
  durationSecs: z.number().min(0),
  /** 動画のタイトル */
  title: z.string(),
  /** チャンネルid */
  channelId: ChannelIdSchema,
  /** 動画の公開者, 箱外の時に付与 */
  uploaderName: z.string().optional(),
  /** 動画の公開時刻 */
  publishedAt: z.iso.datetime(),
  /** これらの情報が最後に同期された時刻 */
  syncedAt: z.iso.datetime(),
  /** 動画の公開状況 */
  privacyStatus: z.enum(["public", "unlisted", "private"]),
  /** 埋め込み可能かどうか */
  embeddable: z.boolean(),
  /** 動画のタグ */
  videoTags: z.array(z.string().nonempty()).nonempty().optional(),
});
export type Video = z.infer<typeof VideoSchema>;
export const VideosRecordSchema = z.record(VideoIdSchema, VideoSchema);
export type VideosRecord = z.infer<typeof VideosRecordSchema>;

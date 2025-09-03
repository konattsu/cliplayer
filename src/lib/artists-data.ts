import { z } from "zod";

import type { lang } from "@/types/lang";
import type { Clip } from "@/types/music";
import type { Result } from "@/types/result";

import artistsJson from "@/music_data/artists.min.json";
import { ArtistIdSchema, type ArtistId } from "@/types/artist-id";

const ArtistSchema = z
  .object({
    /** 日本語 */
    ja: z.string().nonempty(),
    /** 平仮名 */
    jah: z.string().nonempty(),
    /** 英語 */
    en: z.string().nonempty(),
    /** モチーフカラー */
    color: z.string().nonempty(),
    /** 卒業したか */
    isGraduated: z.boolean().default(false),
  })
  .readonly();
export type Artist = z.infer<typeof ArtistSchema>;

const ArtistsIdRecordsSchema = z
  .record(ArtistIdSchema, ArtistSchema)
  .readonly();
type Artists = z.infer<typeof ArtistsIdRecordsSchema>;

/**
 * `artistId`とそれに紐づくアーティスト情報のマップ
 */
const ArtistsIdRecords: Artists = ArtistsIdRecordsSchema.parse(artistsJson);

/**
 * `artistId`からアーティスト情報を取得
 * @return `artistId`に対応するアーティスト情報が存在しないと"not_found"
 */
export function getArtistDataFromId(
  artistId: ArtistId,
): Result<Artist, "not_found"> {
  const artist = ArtistsIdRecords[artistId];
  if (artist !== undefined) {
    return { isOk: true, val: artist };
  } else {
    return { isOk: false, err: "not_found" };
  }
}

/**
 * `clip`からアーティストの名前を取得
 * @param clip クリップ
 * @param lang 言語
 */
export function getArtistNamesFromClip(clip: Clip, lang: lang): string[] {
  let artistNames = clip.artists.map((artistId) => {
    const artist = getArtistDataFromId(artistId);
    if (artist.isOk) {
      return artist.val[lang];
    } else {
      console.warn(`Artist not found for ID: ${artistId}`);
      return "";
    }
  });
  if (Array.isArray(clip.externalArtists) && clip.externalArtists.length > 0) {
    artistNames = artistNames.concat(clip.externalArtists);
  }
  return artistNames.filter((name) => name !== "");
}

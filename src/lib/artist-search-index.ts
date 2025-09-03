import { z } from "zod";

import artistSearchIndexJson from "@/music_data/artist_search_index.min.json";
import { ArtistIdSchema } from "@/types/artist-id";

const artistSearchIndexSchema = z
  .object({
    /** アーティスト名 */
    key: ArtistIdSchema,
    /** アーティストid */
    artistId: z.string().nonempty(),
    /** アーティスト名がaliasかどうか */
    isAlias: z.boolean().default(false),
  })
  .readonly();
type ArtistSearchIndex = z.infer<typeof artistSearchIndexSchema>;

/** Vec<artistSearchIndex> */
export const channelArtistId: ArtistSearchIndex = artistSearchIndexSchema.parse(
  artistSearchIndexJson,
);

import { z } from "zod";

import channelsJson from "@/music_data/channels.min.json";
import { ArtistIdSchema } from "@/types/artist-id";
import { ChannelIdSchema } from "@/types/youtube";

const ChannelArtistIdSchema = z
  .record(ChannelIdSchema, ArtistIdSchema)
  .readonly();
type ChannelArtistId = z.infer<typeof ChannelArtistIdSchema>;

/** (channelId, artistId) */
export const channelArtistId: ChannelArtistId =
  ChannelArtistIdSchema.parse(channelsJson);

import React from "react";

import type { ArtistId } from "@/types/artist-id";

import { getArtistDataFromId } from "@/lib/artists-data";

/** 再生している楽曲などの情報表示のprops */
interface ClipInfoProps {
  songTitle: string;
  artists: ArtistId[];
  externalArtists?: string[];
}

/** 再生している楽曲などの情報表示 */
const ClipInfo: React.FC<ClipInfoProps> = ({
  songTitle,
  artists,
  externalArtists,
}) => {
  const artistsNames = artists.map((artistId) => {
    const artist = getArtistDataFromId(artistId);
    if (artist.isOk) {
      return artist.val.ja;
    } else {
      console.warn(`Artist not found for ID: ${artistId}`);
      return artist;
    }
  });
  if (externalArtists != null && externalArtists.length > 0) {
    artistsNames.push(...externalArtists);
  }

  return (
    <div className="mb-2">
      {/* <div className="mb-2 leading-6"> */}
      <div className="text-2xl">{songTitle}</div>
      <div className="p-3">{artistsNames.join(", ")}</div>
    </div>
  );
};

export { ClipInfo, type ClipInfoProps };

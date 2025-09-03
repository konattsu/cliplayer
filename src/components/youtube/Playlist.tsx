import React from "react";

import type { ClipsRecord } from "@/types/music";

import { getArtistNamesFromClip } from "@/lib/artists-data";

type ClipSelectHandler = (_index: number) => void;

interface PlaylistProps {
  clips: ClipsRecord;
  currentIndex: number;
  onClipSelect: ClipSelectHandler;
}

function formatDuration(start: number, end: number): string {
  const secs = Math.max(0, end - start);
  const min = Math.floor(secs / 60);
  const sec = secs % 60;
  return `${min}:${sec.toString().padStart(2, "0")}`;
}

export function Playlist({
  clips,
  currentIndex,
  onClipSelect,
}: PlaylistProps): React.JSX.Element {
  const clipsArray = Object.entries(clips);

  return (
    <div className="w-full px-1">
      <h3 className="mb-4 text-lg font-semibold">Playlist</h3>
      <div className="space-y-1">
        {clipsArray.map(([uuid, clip], index) => {
          const isPlaying = index === currentIndex;
          const artistNames = getArtistNamesFromClip(clip, "ja");
          const durationStr = formatDuration(
            clip.startTimeSecs,
            clip.endTimeSecs,
          );

          return (
            <div
              key={
                uuid ??
                `${clip.videoId}-${clip.startTimeSecs}-${clip.endTimeSecs}-${index}`
              }
              onClick={() => onClipSelect(index)}
              className={`cursor-pointer rounded-lg p-3 transition-colors ${
                isPlaying ? "" : ""
                // ? "border-l-4 border-blue-500 bg-blue-100 dark:border-blue-400 dark:bg-blue-900/20"
                // : "bg-gray-50 hover:bg-gray-100 dark:bg-gray-800 dark:hover:bg-gray-700"
              }`}
            >
              <div className="flex items-start justify-between">
                <div className="min-w-0 flex-1">
                  <h4
                    className={`truncate font-medium ${
                      isPlaying ? "" : ""
                      // isPlaying ? "text-blue-700 dark:text-blue-300" : ""
                    }`}
                  >
                    {clip.songTitle}
                  </h4>
                  <p className="mt-1 truncate text-sm text-gray-600 dark:text-gray-400">
                    {artistNames.join(", ")}
                  </p>
                </div>
                <div className="ml-3 flex flex-col items-end text-xs text-gray-500">
                  <span>{durationStr}</span>
                  {isPlaying && (
                    <span className="mt-1 text-blue-500 dark:text-blue-400">
                      ♪ Playing
                    </span>
                  )}
                </div>
              </div>
              <div className="mt-2 flex items-center gap-2">
                <span className="text-xs">#{index + 1}</span>
                <span className="text-xs">ID: {clip.videoId}</span>
              </div>
            </div>
          );
        })}
      </div>
      {clipsArray.length === 0 && (
        <div className="py-8 text-center text-gray-500">
          <p>No clips available</p>
        </div>
      )}
    </div>
  );
}

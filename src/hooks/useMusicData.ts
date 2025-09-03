import { useState, useEffect } from "react";

import type { VideosRecord, ClipsRecord } from "../types/music";

const CLIPS_DATA_URL = "/music_data/clips.min.json";
const VIDEOS_DATA_URL = "/music_data/videos.min.json";

// loading, errorなどはアンチパターンに見えたがloading/error状態をuiに描画するのに必要らしい
interface MusicData {
  videosRecord: VideosRecord | undefined;
  clipsRecord: ClipsRecord | undefined;
  loading: boolean;
  error: string | null;
}

/**
 * 音楽データを取得するカスタムフック
 */
export function useMusicData(): MusicData {
  const [videos, setVideos] = useState<VideosRecord>();
  const [videoClipRecords, setVideoClipRecords] = useState<ClipsRecord>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // (() => {/* */})これは即時関数. 定義部で呼び出されるらしい
  // 対して () => {/* */} はアロー関数
  useEffect(() => {
    async function loadMusicData(): Promise<void> {
      try {
        // 音楽データを並行して取得
        const [videosResponse, videoClipRecordsResponse] = await Promise.all([
          globalThis.fetch(VIDEOS_DATA_URL),
          globalThis.fetch(CLIPS_DATA_URL),
        ]);

        if (!videosResponse.ok) {
          throw new Error(`Failed to load videos: ${videosResponse.status}`);
        }
        if (!videoClipRecordsResponse.ok) {
          throw new Error(
            `Failed to load clips: ${videoClipRecordsResponse.status}`,
          );
        }

        const videosData: VideosRecord = await videosResponse.json();
        const videoClipRecordsData: ClipsRecord =
          await videoClipRecordsResponse.json();

        console.log(`videoClipRecords fetched: ${videoClipRecordsData}`);

        setVideos(videosData);
        setVideoClipRecords(videoClipRecordsData);
      } catch (err: unknown) {
        if (err instanceof Error) {
          setError(err.message);
        } else {
          setError(`Unknown error occurred: ${String(err)}`);
        }
      } finally {
        setLoading(false);
      }
    }

    void loadMusicData();
    // この霧みたいな存在感の`[]`が依存配列. 一回のみ実行される.
    // 逆に任意の変数を入れると, その変数の変更に応じて実行される
  }, []);

  return {
    videosRecord: videos,
    clipsRecord: videoClipRecords,
    loading: loading,
    error,
  };
}

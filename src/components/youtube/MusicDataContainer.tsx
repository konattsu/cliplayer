import React, { useState, useCallback } from "react";

import { MusicPlayer } from "./MusicPlayer";
import { Playlist } from "./Playlist";

import { useMusicData } from "@/hooks/useMusicData";

/**
 * 音楽データの取得・状態管理・描画分岐を担うコンテナコンポーネント
 */
export const MusicDataContainer: React.FC = () => {
  // @ts-expect-error ts(2339)
  const { _videosRecord, clipsRecord, loading, error } = useMusicData();
  const [currentIndex, setCurrentIndex] = useState(0);

  // clipsRecordが未取得の場合は空配列扱い
  const clipsArray =
    clipsRecord !== undefined ? Object.entries(clipsRecord) : [];
  const currentClip =
    clipsArray.length > 0 ? clipsArray[currentIndex]?.[1] : undefined;

  // プレイリストで曲を選択したとき
  const handleClipSelect = useCallback((index: number) => {
    setCurrentIndex(index);
  }, []);

  // 次の曲へ
  const handleNext = useCallback(() => {
    setCurrentIndex((prev) => {
      if (clipsArray.length === 0) return 0;
      return (prev + 1) % clipsArray.length;
    });
  }, [clipsArray.length]);

  // 前の曲へ
  const handlePrevious = useCallback(() => {
    setCurrentIndex((prev) => {
      if (clipsArray.length === 0) return 0;
      return (prev - 1 + clipsArray.length) % clipsArray.length;
    });
  }, [clipsArray.length]);

  // ローディング・エラー・データなし分岐
  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="text-center">
          <div className="mx-auto h-8 w-8 animate-spin rounded-full border-4 border-blue-500 border-t-transparent"></div>
          <p className="mt-2 text-gray-600">Loading music data...</p>
        </div>
      </div>
    );
  }
  if (error !== null) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="rounded-lg bg-red-50 p-6 text-center">
          <div className="text-red-500">⚠️</div>
          <h2 className="mt-2 text-lg font-semibold text-red-800">
            Error Loading Music Data
          </h2>
          <p className="mt-1 text-red-600">{error}</p>
        </div>
      </div>
    );
  }
  if (clipsArray.length === 0 || currentClip === null) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="text-center">
          <p className="">No music clips available</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen">
      <div className="container mx-auto px-2 py-8">
        <header className="mb-8 text-center">
          <h1 className="text-3xl font-bold">Clip Player</h1>
          <p className="mt-2">YouTube music clip continuous player</p>
        </header>

        <div className="flex flex-col gap-4 md:flex-col lg:flex-row">
          <MusicPlayer
            clip={currentClip!}
            onNext={handleNext}
            onPrevious={handlePrevious}
          />
          <div className="md:max-w-full lg:max-w-90">
            <Playlist
              clips={clipsRecord ?? {}}
              currentIndex={currentIndex}
              onClipSelect={handleClipSelect}
            />
          </div>
        </div>

        <footer className="mt-16 text-center text-sm">
          <p>Unofficial Project</p>
        </footer>
      </div>
    </div>
  );
};

// TODO 楽曲情報でvideoのmetaの実集めたもの作成(rust), min化
// videoIdでO(1)でアクセスできるように(かつvideoId自体もフィールドに持つ)
// これを使用してclipの再生時にチャンネル名を@~で表示したり, 公開日を表示したりする

// TODO playlistの現在再生しているのは一番上に表示. 順番は保持しておく. タップするとその順番までジャンプ
// 他のplaylistは"曲名"タップで遷移

// TODO 入力データから実際の再生/終了時刻は1,2s遅延いれる. lib/で時刻パースするときにoptional引数で遅らせる/速めるをintで取ったり...

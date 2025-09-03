import React, { useRef, useState, useEffect } from "react";

import PlayerControls from "./PlayerControls";
import YouTubeEmbed from "./YouTubeEmbed";

import type { Clip } from "@/types/music";
import type { YouTubePlayer } from "react-youtube";

// 動画の現在の時間を取得するインターバル
const CURRENT_VIDEO_TIME_INTERVAL_MSEC = 500;

interface MusicPlayerProps {
  clip: Clip;
  onNext: () => void;
  onPrevious: () => void;
}
/**
 * 楽曲を再生するcomponent
 */
export const MusicPlayer: React.FC<MusicPlayerProps> = ({
  clip,
  onNext,
  onPrevious,
}) => {
  const playerRef = useRef<YouTubePlayer | null>(null);
  const [playing, setPlaying] = useState(true);
  const [currentTimeSecs, setCurrentTimeSecs] = useState(0);
  const [volumePercent, setVolumePercent] = useState(100);

  const startSecs: number = clip.startTimeSecs ?? 0;
  const endSecs: number = clip.endTimeSecs ?? 0;
  const durationSecs: number = endSecs - startSecs;

  // progress barの為の動画の再生している時間を定期的に取得
  useEffect(() => {
    let intervalId: number | undefined;
    if (playing && playerRef.current !== null) {
      intervalId = window.setInterval(() => {
        const player = playerRef.current;
        if (player !== null && typeof player.getCurrentTime === "function") {
          setCurrentTimeSecs(player.getCurrentTime());
        }
      }, CURRENT_VIDEO_TIME_INTERVAL_MSEC);
    }

    return (): void => {
      if (intervalId !== undefined) clearInterval(intervalId);
    };
  }, [playing, clip]);

  // シーク時のハンドラ
  const handleSeek = (time: number): void => {
    const player = playerRef.current;
    if (player !== null && typeof player.seekTo === "function") {
      player.seekTo(time + startSecs, true);
      setCurrentTimeSecs(time + startSecs);
    }
  };

  // YouTubeプレイヤー準備完了時に開始位置にシーク
  const handleReady = (event: { target: YouTubePlayer }): void => {
    playerRef.current = event.target;
    if (typeof event.target.seekTo === "function") {
      event.target.seekTo(startSecs, true);
      setPlaying(true);
    } else {
      console.error(`YouTube player is not ready for seeking, ${event.target}`);
    }
  };

  // YouTubeプレイヤーの状態変化でplayingを同期, 再生位置が区間終了を超えたら自動で次へ
  const handleStateChange = (event: { target: YouTubePlayer }): void => {
    const playerState = event.target.getPlayerState?.();
    if (playerState === 1) {
      // 再生中
      setPlaying(true);
    } else if (playerState === 2) {
      // 一時停止
      setPlaying(false);
    }
    if (event.target.getCurrentTime() >= endSecs) onNext();
  };

  // 再生/一時停止
  // player側の変更でも反映されるように
  const togglePlayPause = (): void => {
    setPlaying((prev: boolean) => {
      const player = playerRef.current;
      if (player !== null) {
        if (prev && typeof player.pauseVideo === "function") {
          player.pauseVideo();
        } else if (!prev && typeof player.playVideo === "function") {
          player.playVideo();
        } else {
          console.error(
            `YouTube player is not ready for play/pause, ${player}`,
          );
        }
      } else {
        console.error("YouTube player reference is null");
      }
      return !prev;
    });
  };

  // 音量変更時にYouTubeプレイヤーへ反映
  useEffect(() => {
    const player = playerRef.current;
    if (player !== null && typeof player.setVolume === "function") {
      player.setVolume(volumePercent);
    }
  }, [volumePercent]);

  return (
    <div className="w-full">
      {/* 動画本体 */}
      <div className="aspect-video w-full">
        <YouTubeEmbed
          key={clip.videoId + clip.startTimeSecs.toString()}
          videoId={clip.videoId}
          startSeconds={startSecs}
          endSeconds={endSecs}
          playerRef={playerRef}
          onReady={handleReady}
          onStateChange={handleStateChange}
        />
      </div>
      {/* 動画のメタデータ/コントロール部分 */}
      <PlayerControls
        seekBarProps={{
          clipCurrentTimeSecs: currentTimeSecs - startSecs,
          durationSecs: durationSecs,
          onSeek: handleSeek,
        }}
        volumeBarProps={{
          volumePercent: volumePercent,
          setVolumePercent: setVolumePercent,
        }}
        clipInfoProps={{
          songTitle: clip.songTitle,
          artists: clip.artists,
          externalArtists: clip.externalArtists,
        }}
        mediaControlsProps={{
          onPrevious: onPrevious,
          onNext: onNext,
          onPlayPause: togglePlayPause,
          playing: playing,
        }}
      />
    </div>
  );
};

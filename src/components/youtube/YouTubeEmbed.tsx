import type React from "react";

import YouTube from "react-youtube";

import type { VideoId } from "@/types/youtube";
import type { YouTubePlayer } from "react-youtube";

/** YouTube埋め込みのprops  */
interface YouTubeEmbedProps {
  videoId: VideoId;
  startSeconds: number;
  endSeconds: number;
  playerRef: React.RefObject<YouTubePlayer>;
  onReady: (event: { target: YouTubePlayer }) => void;
  onStateChange: (event: { target: YouTubePlayer }) => void;
}
/** YouTube埋め込み */
const YouTubeEmbed: React.FC<YouTubeEmbedProps> = ({
  videoId,
  startSeconds,
  endSeconds,
  playerRef,
  onReady,
  onStateChange,
}) => {
  return (
    // TODO 最大化を引き継ぎたいなら自分のサイトで<div>z-index-99999..かつ最大化.
    // たしか最大化してその上に要素かぶせは規約違反やから動画9割, その他1割にするべき
    // そこにYouTubeEmbed重ねるしかない. おそらくセキュリティ上の制限でこれ以外できない
    // これ以外はレンダリング安定しない
    <div style={{ width: "100%", aspectRatio: "16/9" }}>
      <YouTube
        videoId={videoId}
        ref={playerRef}
        opts={{
          width: "100%",
          height: "100%",
          playerVars: {
            autoplay: true,
            controls: true,
            // for dev
            rel: false,
            mute: true,
            start: startSeconds,
            end: endSeconds,
          },
        }}
        style={{ width: "100%", height: "100%" }}
        onReady={onReady}
        onStateChange={onStateChange}
      />
    </div>
  );
};

export default YouTubeEmbed;

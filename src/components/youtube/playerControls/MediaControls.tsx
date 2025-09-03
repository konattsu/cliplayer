import type React from "react";

/** 再生コントロール部分のprops */
interface MediaControlsProps {
  onPrevious: () => void;
  onNext: () => void;
  onPlayPause: () => void;
  playing: boolean;
}
/** 再生コントロール部分 */
const MediaControls: React.FC<MediaControlsProps> = ({
  onPrevious,
  onNext,
  onPlayPause,
  playing,
}) => {
  return (
    <div className="flex justify-center gap-4">
      <button onClick={onPrevious}>⏮️</button>
      <button onClick={onPlayPause}>{playing ? "⏸️" : "▶️"}</button>
      <button onClick={onNext}>⏭️</button>
    </div>
  );
};

export { MediaControls, type MediaControlsProps };

import React from "react";

import { formatTimeFromSecs } from "@/lib/duration";

/** シークバのprops */
interface SeekBarProps {
  clipCurrentTimeSecs: number;
  durationSecs: number;
  onSeek: (time: number) => void;
}
/** シークバ */
const SeekBar: React.FC<SeekBarProps> = ({
  clipCurrentTimeSecs,
  durationSecs,
  onSeek,
}) => {
  const progress =
    durationSecs > 0 ? (clipCurrentTimeSecs / durationSecs) * 100 : 0;

  // シークバー上を直接操作したとき
  const handleSeek = (
    e: React.MouseEvent<HTMLDivElement, MouseEvent>,
  ): void => {
    const bar = e.currentTarget;
    const rect = bar.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = Math.min(Math.max(x / rect.width, 0), 1);
    onSeek(percent * durationSecs);
  };

  const currentTimeFormatted = formatTimeFromSecs(clipCurrentTimeSecs);
  const durationFormatted = formatTimeFromSecs(durationSecs);

  return (
    <div className="flex items-center justify-between gap-2">
      {/* 左の現在時間表示 */}
      <div className="min-w-6 text-xs">{currentTimeFormatted}</div>
      {/* シークバー本体 */}
      <div
        className="my-2 h-1 w-full cursor-pointer rounded bg-gray-300"
        onClick={handleSeek}
      >
        <div
          className="h-1 rounded bg-blue-500"
          style={{ width: `${Math.min(Math.max(progress, 0), 100)}%` }}
        ></div>
      </div>
      {/* 右のclipの時間表示 */}
      <div className="min-w-6 text-xs">{durationFormatted}</div>
    </div>
  );
};

export { SeekBar, type SeekBarProps };

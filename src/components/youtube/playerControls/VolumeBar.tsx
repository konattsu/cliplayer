import React from "react";

/** 音量バーのprops */
interface VolumeBarProps {
  volumePercent: number;
  setVolumePercent: (v: number) => void;
}
/** 音量バー */
const VolumeBar: React.FC<VolumeBarProps> = ({
  volumePercent,
  setVolumePercent,
}) => {
  return (
    <div className="my-2 flex items-center gap-2">
      <span className="text-xs">🔊</span>
      <input
        type="range"
        min={0}
        max={100}
        value={volumePercent}
        onChange={(e) => setVolumePercent(Number(e.target.value))}
        className="w-32 accent-blue-500"
      />
      <span className="w-8 text-right text-xs">{volumePercent}</span>
    </div>
  );
};

export { VolumeBar, type VolumeBarProps };

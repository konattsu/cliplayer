import React from "react";

import { ClipInfo } from "./playerControls/ClipInfo";
import { MediaControls } from "./playerControls/MediaControls";
import { SeekBar } from "./playerControls/SeekBar";
import { VolumeBar } from "./playerControls/VolumeBar";

import type { ClipInfoProps } from "./playerControls/ClipInfo";
import type { MediaControlsProps } from "./playerControls/MediaControls";
import type { SeekBarProps } from "./playerControls/SeekBar";
import type { VolumeBarProps } from "./playerControls/VolumeBar";

interface PlayerControlsProps {
  seekBarProps: SeekBarProps;
  volumeBarProps: VolumeBarProps;
  clipInfoProps: ClipInfoProps;
  mediaControlsProps: MediaControlsProps;
}

const PlayerControls: React.FC<PlayerControlsProps> = ({
  seekBarProps,
  volumeBarProps,
  clipInfoProps,
  mediaControlsProps,
}) => {
  const [isOpen, setIsOpen] = React.useState(false);

  const toggleDetailVisibility = (isOpen: boolean): React.JSX.Element => {
    const msg = isOpen ? "一部を表示" : "...詳細を表示";
    return (
      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          setIsOpen(!isOpen);
        }}
        className="text-sm italic opacity-70"
      >
        {msg}
      </button>
    );
  };

  return (
    <>
      <SeekBar {...seekBarProps} />
      <div
        className={`mx-1 my-5 rounded-2xl bg-gray-100 p-3 dark:bg-zinc-800 ${
          isOpen ? "" : "cursor-pointer"
        }`}
        onClick={() => setIsOpen(true)}
      >
        <ClipInfo {...clipInfoProps} />
        <>
          {isOpen && (
            <>
              チャンネル名など...
              <VolumeBar {...volumeBarProps} />
              <MediaControls {...mediaControlsProps} />
            </>
          )}
          {toggleDetailVisibility(isOpen)}
        </>
      </div>
    </>
  );
};

export default PlayerControls;

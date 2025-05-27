import React from "react";

import HeaderIcon from "@/components/HeaderIcon";
import PlaylistSideBar from "@/components/PlaylistSidebar";
import VideoInfo from "@/components/VideoInfo";
import VideoPlayer from "@/components/VideoPlayer";

const Home: React.FC = () => {
  return (
    <div className="flex flex-col min-h-screen">
      <div className="flex p-2">
        <HeaderIcon />
      </div>

      <div className="flex flex-1 p-4">
        <div className="flex-1 mr-4">
          <VideoPlayer />
          <VideoInfo />
        </div>
        <PlaylistSideBar />
      </div>
    </div>
  );
};

export default Home;

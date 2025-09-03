import React from "react";

import { MusicDataContainer } from "./components/youtube/MusicDataContainer";

export const App: React.FC = () => {
  return (
    <div className="min-h-screen">
      <MusicDataContainer />
    </div>
  );
};

export default App;

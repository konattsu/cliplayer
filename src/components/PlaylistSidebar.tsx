import type React from "react";

const PlaylistSideBar: React.FC = () => {
  const dummyList = ["Video 1", "Video 2", "Video 3", "Video 4", "Video 5"];

  return (
    <div className="bg-red-500 w-60 bg-gray-100 p-2 overflow-y-auto ">
      <h3 className="border-4 border-red-600 text-blue-500 p-4">再生リスト</h3>
      <ul>
        {dummyList.map((title, idx) => (
          <li
            key={idx}
            className="py-2 border-b hover:bg-gray-200 cursor-pointer"
          >
            {title}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default PlaylistSideBar;

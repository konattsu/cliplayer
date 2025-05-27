import React from "react";

const HeaderIcon: React.FC = () => {
  return (
    <div
      className="w-12 h-12 bg-gray-300 rounded"
      onClick={() => (window.location.href = "/")}
    >
      Home
    </div>
  );
};

export default HeaderIcon;

import React from "react";
import Sidebar from "./components/Sidebar";
import Content from "./components/Content";

const page = () => {
  return (
    <div>
      <div className="flex flex-row w-full">
        <div className="w-1/4 border-r-2 h-screen border-gray-800">
          <Sidebar />
        </div>
        <div className="w-3/4">
          <Content />
        </div>
      </div>
    </div>
  );
};

export default page;
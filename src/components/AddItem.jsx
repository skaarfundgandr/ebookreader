import React, { useState } from "react";
import { CiCirclePlus } from "react-icons/ci";

export default function AddItem() {
  const [showPopup, setShowPopup] = useState(false);

  const handleFiles = (e) => {
    const files = e.target.files;
    console.log(files);
  };

  return (
    <div className="relative">
      {/* Add Button */}
      <div className="h-12 w-40 rounded-full">
        <button
          onClick={() => setShowPopup(true)}
          className="
            flex items-center justify-start p-2 h-full w-full 
            bg-[var(--color-primary)] text-white rounded-full
            transition-colors duration-150 
            hover:bg-[#ea580c]
            active:bg-[#c2410c]
          "
        >
          <CiCirclePlus size={30} className="m-1" />
          <span className="text-lg text-center font-semibold">Add Book</span>
        </button>
      </div>

      {/* Popup */}
      {showPopup && (
        <div
          className="fixed inset-0 bg-black/60 backdrop-blur-sm flex justify-center items-center z-50"
          onClick={() => setShowPopup(false)} // click outside closes popup
        >
          <div
            className="bg-white rounded-xl p-6 w-[500px] shadow-lg text-gray-900"
            onClick={(e) => e.stopPropagation()} // prevent closing when clicking inside
          >
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-xl font-bold">Import Books</h2>
              <button
                onClick={() => setShowPopup(false)}
                className="bg-gray-400 text-white px-3 py-1 rounded hover:bg-gray-500"
              >
                Close
              </button>
            </div>

            {/* Custom File Area */}
            <div className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-lg h-[300px] cursor-pointer hover:bg-gray-50 transition">
              <label
                htmlFor="fileInput"
                className="flex flex-col items-center justify-center h-full w-full cursor-pointer"
              >
                <span className="text-gray-500 text-sm">
                  Click or drag a folder here
                </span>
              </label>
              <input
                type="file"
                id="fileInput"
                webkitdirectory="true"
                directory="true"
                onChange={handleFiles}
                className="hidden"
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

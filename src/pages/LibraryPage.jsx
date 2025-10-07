import { useState } from "react";
import {
  FaPlus,
  FaFolder,
  FaChevronDown,
  FaChevronRight,
} from "react-icons/fa";
import BookCard from "../assets/Bookdata/BookCard";

export default function LibraryPage() {
  const [folders, setFolders] = useState([]);
  const [showPopup, setShowPopup] = useState(false);
  const [previewFolder, setPreviewFolder] = useState(null);

  // 🟠 Open popup when clicking the "+" button
  const handleAddFolderClick = () => {
    setShowPopup(true);
  };

  // 🟣 Handle folder selection inside the popup
  const handleSelectFolder = async () => {
    try {
      const dirHandle = await window.showDirectoryPicker();
      const files = [];

      for await (const entry of dirHandle.values()) {
        if (entry.kind === "file" && /\.(pdf|epub)$/i.test(entry.name)) {
          files.push({
            title: entry.name.replace(/\.[^/.]+$/, ""), // remove extension
            author: "Unknown",
            coverImage: null,
          });
        }
      }

      if (files.length === 0) {
        alert("No supported book files (PDF or EPUB) found in this folder.");
        return;
      }

      setPreviewFolder({
        name: dirHandle.name,
        books: files,
      });
    } catch (err) {
      console.warn("Folder selection canceled or failed:", err);
    }
  };

  // 🟢 Confirm and add folder to library
  const handleConfirmAdd = () => {
    if (!previewFolder) return;

    setFolders((prev) => [
      ...prev,
      { ...previewFolder, expanded: false },
    ]);

    setPreviewFolder(null);
    setShowPopup(false);
  };

  // 🔴 Cancel and close popup
  const handleCancel = () => {
    setPreviewFolder(null);
    setShowPopup(false);
  };

  // 🟣 Toggle expand/collapse
  const toggleExpand = (index) => {
    setFolders((prev) =>
      prev.map((folder, i) =>
        i === index ? { ...folder, expanded: !folder.expanded } : folder
      )
    );
  };

  return (
    <div className="min-h-screen w-full text-white p-6 space-y-6 relative">
      {/* 🟠 Center view when no folders */}
      {folders.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-[80vh] text-center space-y-4">
          <button
            onClick={handleAddFolderClick}
            className="flex items-center justify-center bg-gradient-to-br from-[#ff8a00] to-[#ff4500] hover:from-[#ff9f40] hover:to-[#ff5c00] text-white p-5 rounded-full shadow-lg shadow-orange-600/30 transition-transform hover:scale-110"
          >
            <FaPlus size={28} />
          </button>
          <div>
            <p className="text-black text-lg font-medium">
              No folders added yet.
            </p>
            <p className="text-gray-500 text-sm">
              Click the + button to add your first folder.
            </p>
          </div>
        </div>
      ) : (
        <>
          {/* Top add folder button */}
          <div className="flex justify-start mb-4">
            <button
              onClick={handleAddFolderClick}
              className="flex items-center justify-center bg-gradient-to-br from-[#ff8a00] to-[#ff4500] hover:from-[#ff9f40] hover:to-[#ff5c00] text-white p-4 rounded-full shadow-lg shadow-orange-600/30 transition-transform hover:scale-110"
              title="Add another folder"
            >
              <FaPlus size={22} />
            </button>
          </div>

          {/* Folder List */}
          {folders.map((folder, index) => (
            <div
              key={index}
              className="bg-[var(--color-primary)] rounded-2xl p-4 shadow-lg border border-[#ff8a00]/10"
            >
              {/* Folder Header */}
              <div
                className="flex items-center cursor-pointer select-none"
                onClick={() => toggleExpand(index)}
              >
                <FaFolder className="text-yellow-400 mr-3" size={24} />
                <span className="text-lg font-semibold tracking-wide">
                  {folder.name}
                </span>
                <div className="ml-auto">
                  {folder.expanded ? (
                    <FaChevronDown className="text-gray-400" />
                  ) : (
                    <FaChevronRight className="text-gray-400" />
                  )}
                </div>
              </div>

              {/* Books in folder */}
              {folder.expanded && (
                <div className="flex flex-wrap gap-4 mt-4">
                  {folder.books.length > 0 ? (
                    folder.books.map((book, i) => (
                      <BookCard
                        key={i}
                        title={book.title}
                        author={book.author}
                        coverImage={book.coverImage}
                      />
                    ))
                  ) : (
                    <p className="text-gray-400">
                      No books found in this folder.
                    </p>
                  )}
                </div>
              )}
            </div>
          ))}
        </>
      )}

      {/* 🪄 Popup Overlay */}
      {showPopup && (
        <div
          className="fixed inset-0 bg-black/60 backdrop-blur-sm flex justify-center items-center z-50"
          onClick={handleCancel}
        >
          <div
            className="bg-white rounded-xl p-6 w-[500px] shadow-lg text-gray-900"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-xl font-bold text-[#ff4500]">
                Import Books
              </h2>
              <button
                onClick={handleCancel}
                className="bg-gray-400 text-white px-3 py-1 rounded hover:bg-gray-500"
              >
                Close
              </button>
            </div>

            {/* If no folder selected yet */}
            {!previewFolder && (
              <div
                onClick={handleSelectFolder}
                className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-lg h-[300px] cursor-pointer hover:bg-gray-50 transition text-gray-600 text-sm hover:text-[#ff8a00]"
              >
                Click anywhere here to select a folder to scan
              </div>
            )}
            
            {/* Folder preview */}
            {previewFolder && (
              <div>
                <h3 className="text-lg font-semibold mb-2">
                  Folder: {previewFolder.name}
                </h3>
                <div className="max-h-[200px] overflow-y-auto border rounded-md p-2 bg-gray-50">
                  {previewFolder.books.map((b, i) => (
                    <p
                      key={i}
                      className="text-gray-700 text-sm border-b last:border-none py-1"
                    >
                      📘 {b.title}
                    </p>
                  ))}
                </div>

                <div className="flex justify-end gap-3 mt-4">
                  <button
                    onClick={handleCancel}
                    className="px-4 py-2 bg-gray-300 text-gray-800 rounded hover:bg-gray-400"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleConfirmAdd}
                    className="px-4 py-2 bg-gradient-to-br from-[#ff8a00] to-[#ff4500] text-white rounded hover:scale-105 transition"
                  >
                    Confirm
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

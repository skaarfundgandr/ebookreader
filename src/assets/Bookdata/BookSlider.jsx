import { useState, useEffect } from "react";
import BookCard from "./BookCard";
import { SlArrowRight, SlArrowLeft } from "react-icons/sl";

export default function BookSlider({ books, title }) {
  const [currentPage, setCurrentPage] = useState(0);
  const [visibleCount, setVisibleCount] = useState(10);

  // Update visibleCount on resize
  useEffect(() => {
    const updateVisibleCount = () => {
      if (window.innerWidth < 640) setVisibleCount(2); // mobile
      else if (window.innerWidth < 1024) setVisibleCount(4); // tablet
      else setVisibleCount(10); // desktop
    };

    updateVisibleCount();
    window.addEventListener("resize", updateVisibleCount);
    return () => window.removeEventListener("resize", updateVisibleCount);
  }, []);

  const totalPages = Math.ceil(books.length / visibleCount);
  const startIndex = currentPage * visibleCount;
  const visibleBooks = books.slice(startIndex, startIndex + visibleCount);

  const handleNext = () => setCurrentPage((prev) => (prev + 1) % totalPages);
  const handlePrev = () => setCurrentPage((prev) => (prev - 1 + totalPages) % totalPages);

  return (
    <div className="relative w-full h-full p-6 pt-0">
      {/* Header (Title + Controls) */}
      <div className="flex justify-between items-center px-10 mb-4">
        {title && (
          <h2 className="text-xl sm:text-2xl font-bold text-white tracking-wide">
            {title}
          </h2>
        )}
        <div className="flex gap-3">
          <button
            onClick={handlePrev}
            className="w-9 h-9 sm:w-10 sm:h-10 
                       bg-gradient-to-br from-orange-500/60 to-violet-600/60 
                       text-white rounded-full flex items-center justify-center 
                       hover:from-orange-500 hover:to-violet-600 hover:scale-105 
                       transition-all duration-300 shadow-[0_0_10px_rgba(255,153,51,0.4)]"
          >
            <SlArrowLeft size={18} />
          </button>

          <button
            onClick={handleNext}
            className="w-9 h-9 sm:w-10 sm:h-10 
                       bg-gradient-to-br from-orange-500/60 to-violet-600/60 
                       text-white rounded-full flex items-center justify-center 
                       hover:from-orange-500 hover:to-violet-600 hover:scale-105 
                       transition-all duration-300 shadow-[0_0_10px_rgba(255,153,51,0.4)]"
          >
            <SlArrowRight size={18} />
          </button>
        </div>
      </div>

      {/* Books row */}
      <div className="flex gap-2 sm:gap-4 overflow-hidden px-10">
        {visibleBooks.map((book, index) => (
          <BookCard key={index} {...book} />
        ))}
      </div>
    </div>
  );
}

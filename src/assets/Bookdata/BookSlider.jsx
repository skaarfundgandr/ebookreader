import { useState, useEffect } from "react";
import BookCard from "./BookCard";
import { SlArrowRight, SlArrowLeft } from "react-icons/sl";

export default function BookSlider({ books, title }) {
  const [currentPage, setCurrentPage] = useState(0);
  const [visibleCount, setVisibleCount] = useState(10);

  // Update visibleCount on resize
  useEffect(() => {
    const updateVisibleCount = () => {
      if (window.innerWidth < 640) setVisibleCount(2);     // mobile
      else if (window.innerWidth < 1024) setVisibleCount(4); // tablet
      else setVisibleCount(10);                             // desktop
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
      {/* Section Title */}
      {title && (
        <h2 className="text-xl sm:text-2xl font-bold text-gray-800 mb-4 px-10">
          {title}
        </h2>
      )}

      {/* Books row */}
      <div className="flex gap-2 sm:gap-4 overflow-hidden px-10">
        {visibleBooks.map((book, index) => (
          <BookCard key={index} {...book} />
        ))}
      </div>

      {/* Prev button */}
      <button
        onClick={handlePrev}
        className="absolute left-2 top-1/2 -translate-y-1/2 w-8 h-8 sm:w-10 sm:h-10 
                   bg-blue-500 text-white rounded-full flex items-center justify-center hover:bg-blue-600 transition"
      >
        <SlArrowLeft size={16} />
      </button>

      {/* Next button */}
      <button
        onClick={handleNext}
        className="absolute right-2 top-1/2 -translate-y-1/2 w-8 h-8 sm:w-10 sm:h-10 
                   bg-blue-500 text-white rounded-full flex items-center justify-center hover:bg-blue-600 transition"
      >
        <SlArrowRight size={16} />
      </button>
    </div>
  );
}

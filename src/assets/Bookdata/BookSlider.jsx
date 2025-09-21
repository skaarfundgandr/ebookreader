import { useState } from "react";
import BookCard from "./BookCard";
import { SlArrowRight } from "react-icons/sl";
import { SlArrowLeft } from "react-icons/sl";
export default function BookSlider({ books, visibleCount = 5, step = 5 }) {
  const [currentIndex, setCurrentIndex] = useState(0);

  const visibleBooks = [];
  for (let i = 0; i < visibleCount; i++) {
    visibleBooks.push(books[(currentIndex + i) % books.length]);
  }

  const handleNext = () => {
    setCurrentIndex((prev) => (prev + step) % books.length);
  };

  const handlePrev = () => {
    setCurrentIndex((prev) => (prev - step + books.length) % books.length);
  };

  return (
    <div className="flex flex-col items-center w-full h-full p-4 pb-0">
        <div className="flex justify-start w-full mb-4 pl-2 font-bold text-xl">
            <h1>CONTINUE READING</h1>
        </div>
            {/* Controls */}
            <div className="flex items-center gap-4 mt-4">
        {/* Prev button */}
        <button
          onClick={handlePrev}
          className="w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center"
        >
          <SlArrowLeft size={16} />
        </button>

        {/* Books row */}
        <div className="flex gap-4">
          {visibleBooks.map((book, index) => (
            <BookCard key={index} {...book} />
          ))}
        </div>

        {/* Next button */}
        <button
          onClick={handleNext}
          className="w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center"
        >
          <SlArrowRight size={16} />
        </button>
      </div>
    </div>
  );
}
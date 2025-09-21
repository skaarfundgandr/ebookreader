import React from 'react';
import { FaStar } from 'react-icons/fa';

export default function StarRate({ rating = 0, size = 30 }) {
  return (
    <div className="flex justify-center gap-2">
      {[...Array(5)].map((_, index) => {
        const ratingValue = index + 1;
        return (
          <FaStar
            key={index}
            size={size}
            color={ratingValue <= rating ? "yellow" : "grey"}
          />
        );
      })}
    </div>
  );
}

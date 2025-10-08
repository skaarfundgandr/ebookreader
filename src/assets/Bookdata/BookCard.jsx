import { useState } from "react";

export default function BookCard({ title, author, coverImage }) {
  const [imageError, setImageError] = useState(false);
  const hasImage = Boolean(coverImage) && !imageError;

  return (
    <div
      className={`
        w-24 sm:w-28 md:w-32 lg:w-34 xl:w-38
        flex flex-col 
        shadow-md rounded-lg overflow-hidden
        hover:shadow-lg transition
      `}
    >
      {hasImage ? (
        <div className="aspect-[3/4] bg-stellar-dark flex items-center justify-center overflow-hidden">
          <img
            src={coverImage}
            alt={title}
            className="w-full h-full object-cover"
            onError={() => setImageError(true)}
          />
        </div>
      ) : (
        <div className="aspect-[3/4] flex flex-col justify-center items-center bg-gray-200">
          <span className="text-black text-sm text-center">{title}</span>
        </div>
      )}

      {/* Details */}
      <div className="bg-black p-2 text-center">
        <h3 className="text-xs sm:text-sm md:text-base font-bold truncate">
          {title}
        </h3>
        <p className="text-[10px] sm:text-xs md:text-sm text-gray-600">
          {author}
        </p>
      </div>
    </div>
  );
}

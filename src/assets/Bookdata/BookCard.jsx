export default function BookCard({ title, author, coverImage }) {
  return (
    <div
      className="
        w-32 sm:w-32 md:w-36 lg:w-40 
        flex flex-col 
        shadow-md rounded-lg overflow-hidden
        hover:shadow-lg transition
      "
    >
      {/* Cover area fixed height/aspect */}
      <div className="aspect-[3/4] bg-gray-200 flex items-center justify-center overflow-hidden">
        {coverImage ? (
          <img
            src={coverImage}
            alt={title}
            className="w-full h-full object-cover"
          />
        ) : (
          <span className="text-gray-400 text-xs">No cover</span>
        )}
      </div>

      {/* Details */}
      <div className="bg-white p-2 text-center">
        <h3 className="text-sm sm:text-base font-bold truncate">{title}</h3>
        <p className="text-xs sm:text-sm text-gray-600">{author}</p>
      </div>
    </div>
  );
}

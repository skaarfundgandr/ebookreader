export default function HistoryCard({ image, title, value }) {
  return (
    <div
      className="
        w-full max-w-xs sm:max-w-sm md:max-w-md 
        bg-white rounded-md shadow-md 
        flex items-center p-4 
        hover:shadow-lg transition
      "
    >
      {/* Image */}
      <img
        src={image}
        alt={title}
        className="
          w-12 h-12 sm:w-16 sm:h-16 md:w-20 md:h-20 
          object-cover rounded-full flex-shrink-0
        "
      />

      {/* Text */}
      <div className="flex flex-col ml-4 text-black items-center">
        <h1 className="font-bold text-3xl sm:text-5xl md:text-6xl lg:text-7xl">
          {value}
        </h1>
        <h1 className="font-bold text-sm sm:text-base md:text-lg">{title}</h1>
      </div>
    </div>
  );
}

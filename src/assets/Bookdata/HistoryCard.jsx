export default function HistoryCard({ image, title, value }) {
  return (
    <div
      className="
        w-full max-w-[85%] sm:max-w-xs md:max-w-sm
        bg-white rounded-lg shadow-sm 
        flex flex-col sm:flex-row items-center sm:items-center
        p-3 sm:p-4 md:p-5
        hover:shadow-md transition-shadow duration-300
      "
    >
      {/* Image */}
      <img
        src={image}
        alt={title}
        className="
          w-10 h-10 sm:w-12 sm:h-12 md:w-14 md:h-14
          object-cover rounded-full
          mb-2 sm:mb-0 sm:mr-3
        "
      />

      {/* Text */}
      <div className="flex flex-col items-center sm:items-start text-black">
        <h1
          className="
            font-bold 
            text-xl sm:text-2xl md:text-3xl 
            text-center sm:text-left
            leading-snug
          "
        >
          {value}
        </h1>
        <h2
          className="
            font-medium 
            text-xs sm:text-sm md:text-base 
            text-center sm:text-left
            opacity-80
          "
        >
          {title}
        </h2>
      </div>
    </div>
  );
}

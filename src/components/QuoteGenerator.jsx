import { useState, useEffect } from "react";
import backgroundImage from "../images/quotebackground.jpg";

export default function QuoteGenerator() {
  const [quote, setQuote] = useState("");
  const [author, setAuthor] = useState("");
  const [loading, setLoading] = useState(false);

  // Function to fetch a quote from API
  const fetchQuote = async () => {
    try {
      setLoading(true);
      const response = await fetch("https://quotes-api-self.vercel.app/quote");
      const data = await response.json();
      setQuote(data.quote);
      setAuthor(data.author);
      setLoading(false);
    } catch (error) {
      console.error("Error fetching quote:", error);
      setQuote("Failed to load quote. Try again!");
      setAuthor("");
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchQuote();
  }, []);

  return (
    <div className="p-4 sm:p-6 md:p-8 lg:p-12 relative">
      {/* Tag */}
      <div className="absolute top-6 left-6 sm:top-8 sm:left-8 md:top-10 md:left-10 
                      bg-blue-500 text-white px-3 sm:px-4 py-1 rounded-full 
                      text-xs sm:text-sm md:text-base font-semibold italic 
                      shadow-lg shadow-black/50">
        Quote Today
      </div>

      {/* Quote Box */}
      <div
        className="w-full 
                  max-w-sm sm:max-w-md md:max-w-2xl lg:max-w-5xl 
                  min-h-[180px] sm:min-h-[200px] md:min-h-[220px] lg:min-h-full 
                  text-gray-800 flex flex-col items-center justify-center 
                  rounded-xl shadow-lg 
                  p-3 sm:p-4 md:p-6 lg:p-8 
                  bg-cover bg-center mx-auto"
        style={{ backgroundImage: `url(${backgroundImage})` }}
      >

        {loading ? (
          <p className="text-base sm:text-lg md:text-xl lg:text-2xl">Loading...</p>
        ) : (
          <>
            <p className="text-lg sm:text-xl md:text-2xl lg:text-3xl 
                          font-semibold text-center mb-2 
                          px-2 sm:px-6 lg:px-12">
              “{quote}”
            </p>
            <p className="text-sm sm:text-base md:text-lg lg:text-xl italic text-center">
              - {author}
            </p>
          </>
        )}
      </div>
    </div>
  );
}

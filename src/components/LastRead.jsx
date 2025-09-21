import StarRate from '../assets/StarRate';
import ebookCover from '../images/bookCover.png';

export default function LastRead() {
  return (
    <div className="[grid-area: last] p-8 flex justify-center items-center h-100">
        
      {/* Outer card container */}
      <div className="w-full h-full flex items-center justify-center rounded-xl shadow-lg bg-[var(--color-primary)] text-white">
        
        {/* Book cover */}
        <div className="flex justify-start">
          <img 
            src={ebookCover} 
            alt="Book cover" 
            className="h-60 w-35 ml-4 rounded-lg shadow-lg object-cover" 
          />
        </div>

        {/* Book details + actions */}
        <div className="flex flex-col gap-2 ml-4 pr-9 justify-center text-left text-md">
          
          {/* Title / question */}
          <h1 className="text-center">
            Did you read <b>The Lost World</b> by Arthur Conan Doyle?
          </h1>

          {/* Static star rating (passed via prop) */}
          <StarRate rating={4} />

          {/* Call-to-action button */}
          <button 
            className="bg-gray-500 flex items-center justify-center h-[40px] 
                       ml-20 mr-20 mt-4 text-white text-base rounded-full 
                       shadow-lg shadow-black/50 shadow-[8px_0px_10px_rgba(0,0,0,0.5)]
                       hover:bg-gray-600 transition-colors"
          >
            READ TODAY
          </button>
        </div>
      </div>
    </div>
  );
}

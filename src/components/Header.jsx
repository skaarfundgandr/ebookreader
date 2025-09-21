import { IoIosSearch } from "react-icons/io";
import { IoIosNotificationsOutline } from "react-icons/io";
import { CgProfile } from "react-icons/cg";
import { MdKeyboardArrowDown } from "react-icons/md";

export default function Header() {
  return (
    <div className="[grid-area:header] bg-[var(--color-header)] text-white flex items-center px-4 gap-4 sm:gap-6">
      {/* Search Bar */}
      <div className="pl-2 sm:pl-4 md:pl-6 lg:pl-8 flex-1 sm:max-w-md md:max-w-xl lg:max-w-5xl">
        <div className="relative w-full">
          <input
            type="text"
            placeholder="Search"
            className="
              w-full p-2 pl-10 rounded-full 
              text-[var(--color-text)] 
              focus:outline-gray-300 bg-white 
              outline-2 outline-transparent 
              hover:outline-gray-300 
              transition-all duration-500
            "
          />
          <IoIosSearch className="absolute top-1/2 left-3 -translate-y-1/2 text-black" />
        </div>
      </div>

      {/* Right Section */}
      <div className="flex items-center gap-3 sm:gap-6 ml-auto shrink-0">
        {/* Notification Icon */}
        <IoIosNotificationsOutline className="text-xl sm:text-2xl text-[var(--color-text)]" />

        {/* Profile Section */}
        <div className="hidden sm:flex items-center gap-2">
          <CgProfile className="text-2xl sm:text-3xl text-black bg-red-300 rounded-full p-1" />
          <h1 className="hidden lg:block text-sm sm:text-base md:text-lg text-black font-medium">
            Seth A. Pinca
          </h1>
          <MdKeyboardArrowDown className="text-xl sm:text-2xl text-black" />
        </div>
      </div>
    </div>
  );
}

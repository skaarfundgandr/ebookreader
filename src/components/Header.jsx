import { useState, useRef, useEffect } from "react";
import { IoIosSearch } from "react-icons/io";
import { IoIosNotificationsOutline } from "react-icons/io";
import { CgProfile } from "react-icons/cg";
import { MdKeyboardArrowDown } from "react-icons/md";
import { useNavigate } from "react-router";

export default function Header() {
  const [open, setOpen] = useState(false);
  const dropdownRef = useRef(null);
  const navigate = useNavigate();

  // Close dropdown if clicked outside
  useEffect(() => {
    function handleClickOutside(event) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleLogout = () => {
    localStorage.clear();
    navigate("/login")
  };

  return (
    <header className="w-full bg-[var(--color-header)] flex items-center px-4 py-6 shadow-md relative">
      {/* Search Bar */}
      <div className="flex-1 max-w-3xl ml-6">
        <div className="relative w-full">
          <input
            type="text"
            placeholder="Search"
            className="w-full p-2 pl-10 rounded-full bg-gradient-to-r from-white/20 to-white/10 text-white placeholder-gray-300 backdrop-blur-md focus:ring-2 focus:ring-indigo-400 transition"
          />
          <IoIosSearch className="absolute top-1/2 left-3 -translate-y-1/2 text-black" />
        </div>
      </div>

      {/* Right Side */}
      <div className="flex items-center gap-4 ml-auto" ref={dropdownRef}>
        <IoIosNotificationsOutline className="text-2xl text-black" />

        {/* Profile Dropdown */}
        <div
          className="relative flex items-center gap-2 cursor-pointer select-none"
          onClick={() => setOpen((prev) => !prev)}
        >
          <CgProfile className="text-3xl bg-red-300 rounded-full p-1 text-black" />
          <h1 className="hidden md:block text-black font-medium">
            Seth A. Pinca
          </h1>
          <MdKeyboardArrowDown
            className={`text-2xl text-black transition-transform ${
              open ? "rotate-180" : ""
            }`}
          />

          {/* Dropdown Menu */}
          {open && (
            <div className="absolute right-0 top-full mt-3 bg-white shadow-lg rounded-lg w-35 py-2 z-50">
              <button
                onClick={handleLogout}
                className="block w-full text-left px-4 py-2 text-gray-700 hover:bg-gray-100 transition"
              >
                Log Out
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
}

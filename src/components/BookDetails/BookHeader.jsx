import React from "react";
import { IoIosArrowRoundBack } from "react-icons/io";
import { SlOptionsVertical } from "react-icons/sl";

export default function BookHeader({ onBack, onMenu }) {
  return (
    <header className="flex items-center justify-between px-6 py-4 bg-transparent">
      {/* Back Button */}
      <button
        onClick={onBack}
        className="flex items-center gap-2 text-white hover:text-orange-400 transition-colors"
      >
        <IoIosArrowRoundBack size={34} />
        <span className="text-lg font-medium">Back</span>
      </button>

      {/* Options Menu */}
      <button
        onClick={onMenu}
        className="text-white hover:text-orange-400 transition-colors"
      >
        <SlOptionsVertical size={22} />
      </button>
    </header>
  );
}

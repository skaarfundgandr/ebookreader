import { IoHomeOutline } from "react-icons/io5";
import { RiExpandRightLine, RiExpandLeftLine } from "react-icons/ri";
import { MdDashboardCustomize } from "react-icons/md";
import { IoSettingsOutline } from "react-icons/io5";
import { HiMiniQuestionMarkCircle } from "react-icons/hi2";
import { IoBook } from "react-icons/io5";

export default function Sidebar({ isExpanded, setIsExpanded }) {
  return (
    <div
      className="
        [grid-area:sidebar] bg-[var(--color-primary)] text-white h-full p-4
        flex flex-col justify-between transition-all duration-300
      "
    >
      {/* Top Logo */}
      <div className="flex items-center gap-4 pt-4">
        <IoBook size={40} />
        <span
          className={`
            text-lg font-bold whitespace-nowrap overflow-hidden transition-all duration-300
            ${isExpanded ? "opacity-100 max-w-[200px]" : "opacity-0 max-w-0"}
          `}
        >
          My Library
        </span>
      </div>

      {/* Navigation */}
      <div className="flex flex-col gap-5 items-start justify-center mt-10">
        <div className="flex items-center gap-3">
          <IoHomeOutline size={25} />
          {isExpanded && <span>Home</span>}
        </div>
        <div className="flex items-center gap-3">
          <MdDashboardCustomize size={25} />
          {isExpanded && <span>Dashboard</span>}
        </div>
        <div className="flex items-center gap-3">
          <IoSettingsOutline size={25} />
          {isExpanded && <span>Settings</span>}
        </div>
        <div className="flex items-center gap-3">
          <HiMiniQuestionMarkCircle size={25} />
          {isExpanded && <span>Help</span>}
        </div>
      </div>

      {/* Expand / Collapse Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="flex items-center gap-2 mb-6 p-2 rounded-md hover:bg-white/20 transition"
      >
        {isExpanded ? (
          <>
            <RiExpandLeftLine size={25} />
            <span>Collapse</span>
          </>
        ) : (
          <RiExpandRightLine size={25} />
        )}
      </button>
    </div>
  );
}

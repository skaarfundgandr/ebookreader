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
        sticky top-0 self-start
        [grid-area:sidebar] bg-[var(--color-primary)] text-white
        h-screen p-4 flex flex-col justify-between
        transition-all duration-300
      "
    >
      {/* Top Logo */}
      <div
        className={`
          flex items-center pt-4 transition-all duration-300
          ${isExpanded ? "justify-start" : "justify-center"}
        `}
      >
        <IoBook size={40} />
        <span
          className={`
            text-lg font-bold whitespace-nowrap overflow-hidden transition-all duration-300
            ${isExpanded 
              ? "opacity-100 max-w-[200px] ml-4" 
              : "opacity-0 max-w-0 ml-0"}
          `}
        >
          My Library
        </span>
      </div>

      {/* Navigation */}
      <div className="flex flex-col mt-10 gap-5">
        {[
          { icon: <IoHomeOutline size={25} />, label: "Home" },
          { icon: <MdDashboardCustomize size={25} />, label: "Dashboard" },
          { icon: <IoSettingsOutline size={25} />, label: "Settings" },
          { icon: <HiMiniQuestionMarkCircle size={25} />, label: "Help" },
        ].map(({ icon, label }, i) => (
          <div
            key={i}
            className={`
              flex items-center transition-all duration-300
              ${isExpanded ? "justify-start" : "justify-center"}
            `}
          >
            <div className="flex items-center">
              {icon}
              <span
                className={`
                  whitespace-nowrap overflow-hidden transition-all duration-300
                  ${isExpanded 
                    ? "opacity-100 max-w-[150px] ml-3" 
                    : "opacity-0 max-w-0 ml-0"}
                `}
              >
                {label}
              </span>
            </div>
          </div>
        ))}
      </div>

      {/* Expand / Collapse Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className={`
          flex items-center mb-6 p-2 rounded-md hover:bg-white/20 transition-all duration-300
          ${isExpanded ? "justify-start" : "justify-center"}
        `}
      >
        <div className="flex items-center">
          {isExpanded ? <RiExpandLeftLine size={25} /> : <RiExpandRightLine size={25} />}
          <span
            className={`
              whitespace-nowrap overflow-hidden transition-all duration-300
              ${isExpanded 
                ? "opacity-100 max-w-[120px] ml-2" 
                : "opacity-0 max-w-0 ml-0"}
            `}
          >
            Collapse
          </span>
        </div>
      </button>
    </div>
  );
}

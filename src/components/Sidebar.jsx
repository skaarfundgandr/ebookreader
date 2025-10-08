import {
  IoHomeOutline,
  IoSettingsOutline,
  IoBook,
  IoLibraryOutline,
} from "react-icons/io5";
import { IoIosHelpCircleOutline } from "react-icons/io";
import { RiExpandRightLine, RiExpandLeftLine } from "react-icons/ri";
import { NavLink } from "react-router-dom";

export default function Sidebar({ isExpanded, setIsExpanded }) {
  const navItems = [
    { icon: <IoHomeOutline size={25} />, label: "Home", path: "/home" },
    { icon: <IoLibraryOutline size={25} />, label: "Library", path: "/library" },
  ];

  return (
    <aside
      className={`
        bg-[rgba(49,40,61,0.86)] 
        backdrop-blur-md         
        text-white
        p-4 flex flex-col justify-between
        transition-all duration-300 h-full
      `}
    >
      {/* Logo Section */}
      <div
        className={`flex items-center pt-4 transition-all duration-300 ${
          isExpanded ? "justify-start" : "justify-center"
        }`}
      >
        <IoBook size={36} className="text-gray-100" />
        {isExpanded && (
          <span className="text-xl font-semibold ml-3 text-white tracking-wide">
            Stellaron
          </span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex flex-col mt-10 gap-3">
        {navItems.map(({ icon, label, path }, i) => (
          <NavLink
            key={i}
            to={path}
            className={({ isActive }) =>
              `flex items-center gap-3 p-2 rounded-md transition-all duration-200
              ${
                isActive
                  ? "bg-white/10 text-orange-400"
                  : "text-gray-100 hover:text-orange-300 hover:bg-white/5"
              }
              ${isExpanded ? "justify-start" : "justify-center"}`
            }
          >
            {icon}
            {isExpanded && <span className="whitespace-nowrap">{label}</span>}
          </NavLink>
        ))}
      </nav>

      {/* Collapse / Expand Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className={`flex items-center p-2 rounded-md hover:bg-white/10 transition-all duration-300 ${
          isExpanded ? "justify-start" : "justify-center"
        } text-gray-300 hover:text-orange-400`}
      >
        {isExpanded ? (
          <RiExpandLeftLine size={26} />
        ) : (
          <RiExpandRightLine size={26} />
        )}
        {isExpanded && <span className="ml-2 text-sm font-medium">Collapse</span>}
      </button>
    </aside>
  );
}

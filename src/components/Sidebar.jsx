import { IoHomeOutline, IoSettingsOutline, IoBook, IoLibraryOutline  } from "react-icons/io5";
import { IoIosHelpCircleOutline } from "react-icons/io";
import { RiExpandRightLine, RiExpandLeftLine } from "react-icons/ri";
import { NavLink } from "react-router-dom";
import { IoLibrary } from "react-icons/io5";

export default function Sidebar({ isExpanded, setIsExpanded }) {
  const navItems = [
    { icon: <IoHomeOutline size={25} />, label: "Home", path: "/home" },
    { icon: <IoSettingsOutline size={25} />, label: "Settings", path: "/settings" },
    { icon: <IoIosHelpCircleOutline size={25} />, label: "Help", path: "/help" },
    { icon: <IoLibraryOutline size={25} />, label: "Library", path: "/library" },
  ];

  return (
    <aside
      className={`
        bg-[var(--color-primary)] text-white p-4 flex flex-col justify-between
        transition-all duration-300 h-full
      `}
    >
      {/* Logo */}
      <div
        className={`flex items-center pt-4 transition-all duration-300 ${
          isExpanded ? "justify-start" : "justify-center"
        }`}
      >
        <IoBook size={40} />
        {isExpanded && (
          <span className="text-lg font-bold ml-4 whitespace-nowrap">My Library</span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex flex-col mt-10 gap-4">
        {navItems.map(({ icon, label, path }, i) => (
          <NavLink
            key={i}
            to={path}
            className={({ isActive }) =>
              `flex items-center gap-3 p-2 rounded-md transition-all duration-200 ${
                isActive ? "bg-white/20 " : "hover:bg-white/10"
              } ${isExpanded ? "justify-start" : "justify-center"}`
            }
          >
            {icon}
            {isExpanded && <span className="whitespace-nowrap">{label}</span>}
          </NavLink>
        ))}
      </nav>

      {/* Collapse Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className={`flex items-center p-2 rounded-md hover:bg-white/20 transition-all duration-300 ${
          isExpanded ? "justify-start" : "justify-center"
        }`}
      >
        {isExpanded ? <RiExpandLeftLine size={30} /> : <RiExpandRightLine size={30} />}
        {isExpanded && <span className="ml-2">Collapse</span>}
      </button>
    </aside>
  );
}

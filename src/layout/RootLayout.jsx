import { useState } from "react";
import { Outlet } from "react-router-dom";
import Sidebar from "../components/Sidebar";
import Header from "../components/Header";

export default function RootLayout() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div
      className={`
        grid h-screen
        grid-rows-[90px_1fr] 
        [grid-template-areas:'sidebar_header''sidebar_main']

        sm:grid-rows-[90px_1fr] 
        sm:[grid-template-areas:'sidebar_header''sidebar_main']

        max-sm:grid-cols-1 
        max-sm:grid-rows-[90px_1fr_auto] 
        max-sm:[grid-template-areas:'header''main''sidebar']

        transition-all duration-300
        ${isExpanded ? "grid-cols-[250px_1fr]" : "grid-cols-[70px_1fr]"}
      `}
    >
      {/* Header (Top row) */}
      <div className="[grid-area:header] pb-0">
        <Header />
      </div>

      {/* Sidebar (Left column) */}
      <div className="[grid-area:sidebar]">
        <Sidebar isExpanded={isExpanded} setIsExpanded={setIsExpanded} />
      </div>

      {/* Main Content Area (Outlet renders pages here) */}
      <main 
      className="[grid-area:main] bg-gray-200p-6 overflow-auto"
      >
        <Outlet />
      </main>
    </div>
  );
}

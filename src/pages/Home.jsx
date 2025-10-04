import { useState } from "react";
import Sidebar from "../components/Sidebar";
import Header from "../components/Header";
import Main from "../components/Main";

export default function Home() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div
      className={`
        grid h-full
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
      <Header />
      <Sidebar isExpanded={isExpanded} setIsExpanded={setIsExpanded} />
      <Main className="[grid-area: main]" />
    
    </div>
  );
}

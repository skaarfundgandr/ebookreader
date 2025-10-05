import Sidebar from "../components/Sidebar";
import Header from "../components/Header";
import { useState } from "react";

export default function Navbar() {
  const [isSidebarExpanded, setIsSidebarExpanded] = useState(true);

  return (
    <>
      <Header />
      <Sidebar isExpanded={isSidebarExpanded} setIsExpanded={setIsSidebarExpanded} />
    </>
  );
}

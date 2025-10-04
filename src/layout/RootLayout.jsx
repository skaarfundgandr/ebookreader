import { Outlet } from "react-router-dom";
import Navbar from "../components/Navbar";

export default function RootLayout() {
  return (
    <div className="h-screen flex flex-col">
      {/* Navbar includes Sidebar + Header */}
      <Navbar />

      {/* Page content will render here */}
      <main className="flex-1 bg-gray-100 p-6 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}

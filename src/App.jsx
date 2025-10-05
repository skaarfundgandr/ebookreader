import { Routes, Route } from "react-router-dom";
import RootLayout from "./layout/RootLayout";
import HomePage from "./pages/HomePage";
import SettingsPage from "./pages/SettingsPage";
import LibraryPage from "./pages/LibraryPage";
import HelpPage from "./pages/HelpPage";
import LoginPage from "./pages/LoginPage";
import "./App.css";
import "./theme.css";

function App() {
  return (
    <Routes>
      {/* Login page (no layout) */}
      <Route path="/" element={<LoginPage />} />
      <Route path="/login" element={<LoginPage />} />

      {/* Authenticated pages (with layout) */}
      <Route element={<RootLayout />}>
        <Route path="/home" element={<HomePage />} />
        <Route path="/library" element={<LibraryPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/help" element={<HelpPage />} />
      </Route>
    </Routes>
  );
}

export default App;

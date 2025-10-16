import { Routes, Route } from "react-router-dom";
import RootLayout from "./layout/RootLayout";
import HomePage from "./pages/HomePage";
import SettingsPage from "./pages/SettingsPage";
import LibraryPage from "./pages/LibraryPage";
import HelpPage from "./pages/HelpPage";
import LoginPage from "./pages/LoginPage";
import BookPage from "./pages/BookPage";
import "./App.css";
import "./theme.css";
import PlainLayout from "./layout/PlainLayout";

function App() {
  return (
    <Routes>
      {/* Login page (no layout) */}
      <Route path="/" element={<LoginPage />} />
      <Route path="/login" element={<LoginPage />} />

      {/* No Header layout pages */}
      <Route element={<PlainLayout/>}>
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/help" element={<HelpPage />} />
      </Route>

      {/* Authenticated pages (with layout) */}
      <Route element={<RootLayout />}>
        <Route path="/home" element={<HomePage />} />
        <Route path="/library" element={<LibraryPage />} />
      </Route>
    </Routes>
  );
}

export default App;

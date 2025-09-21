import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Home from "./pages/Home";
import "./theme.css";

function App() {
  return (
    <div>
      <Home />
    </div>

  );
}

export default App;
const path = require('path');

module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        'stellar-dark': '#1A1324',         // Deep violet-black
        'stellar-accent': '#B000FF',       // Vibrant violet
        'stellar-glow': '#FF6B00',         // Bright orange accent
        'stellar-light': '#E2E2E2',        // Text light
        'stellar-dim': '#A0A0A0',          // Dim text
        'stellar-bg-glass': 'rgba(26,19,36,0.6)', // translucentbg
      },
      boxShadow: {  
        'stellar-orange': '0 4px 20px rgba(255,107,0,0.15)',
        'stellar-violet': '0 0 15px rgba(176,0,255,0.25)',
      },
    },
  },
  plugins: [],
};

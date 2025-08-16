// Generate Tailwind CSS configuration file
const { tailwind } = require('tailwindcss')
const path = require('path')

module.exports = {
  content: [
    // Add your project files here
    './src/**/*.{js,jsx,ts,tsx,vue}',
    './index.html'
  ],
  theme: {
    extend: {},
  },
  plugins: [
    // Add custom plugins here
  ]
}

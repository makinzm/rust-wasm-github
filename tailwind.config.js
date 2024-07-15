/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html",
  ],
  theme: {
    extend: {},
    fontFamily: {
      'sans': ['Times New Roman', 'sans-serif'],
    },
  },
  plugins: [],
  darkMode: 'media',
}


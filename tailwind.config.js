/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./src/app/*.rs", "./index.html"]
  },
  theme: {
    colors: {
      "light": "#eae0d5",
      "base": "#c6ac8f",
      "dark": "#22333b",
      "border": "#0a0908",
      "other": "#5e503f",
      "highlight-red": "#b47e7f",
      "highlight-green": "#a8d3b1",
      "highlight-blue": "#b3c7d1",
      "highlight-yellow": "#d0d1b3",
      "focus-red": "#540b0e",
    },
  },
  plugins: [],
}
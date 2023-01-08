/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      fontSize: {
        "relative": "calc(8px + 1vmin)"
      }
    },
  },
  plugins: [],
}

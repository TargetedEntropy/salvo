/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'eve-dark': '#0a0e13',
        'eve-darker': '#060810',
        'eve-accent': '#00d9ff',
        'eve-gold': '#ffd700',
        'eve-gray': '#1a1f2e',
      }
    },
  },
  plugins: [],
}

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './assets/**/*.html',
    './assets/**/*.js',
    './assets/static/js/*.js',
  ],
  theme: {
    extend: {
      colors: {
        'aero-sky-light': '#74d2ff',
        'aero-sky-dark': '#008cff',
        'aero-grass-light': '#a1ff8b',
        'aero-grass-dark': '#38c172',
        'aero-glass': 'rgba(255, 255, 255, 0.3)',
        'aero-border': 'rgba(255, 255, 255, 0.5)',
      },
      backgroundImage: {
        'aero-gradient': 'linear-gradient(to bottom, #74d2ff, #008cff)',
        'gloss-gradient': 'linear-gradient(to bottom, rgba(255,255,255,0.6), rgba(255,255,255,0.1))',
      },
      borderRadius: {
        'aero': '12px',
      },
      boxShadow: {
        'aero': '0 4px 30px rgba(0, 0, 0, 0.1)',
      },},
  },
  plugins: [require('@tailwindcss/typography'), require('tailwindcss-motion')],
};

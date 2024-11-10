/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './assets/**/*.html',
    './assets/**/*.js',
    './assets/static/js/*.js',
  ],
  theme: {
    extend: {},
  },
  plugins: [require('@tailwindcss/typography')],
};

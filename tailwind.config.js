/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./src/**/*.rs", "./html/**/*.html"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
};

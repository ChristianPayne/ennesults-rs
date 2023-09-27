/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    colors: {
      gray: {
        400: "#3D3D3D",
        500: "#2f2f2f",
        600: "#292929",
        700: "#1F1F1F"
      },
      white: {
        DEFAULT: "#f6f6f6"
      }
    },
    extend: {},
  },
  plugins: [],
}


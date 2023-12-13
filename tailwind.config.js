import tailwindColors from "tailwindcss/colors";
import * as pallettes from "./colors";

const PALLETTE = pallettes.teal;

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  plugins: [require('flowbite/plugin')],
  darkMode: 'class',
  theme: {
    colors: {
      // Background color that flowbite uses.
      gray: {
        ...PALLETTE
      },
      staple: {
        dark: "#141414",
        light: "#F5F5F5"
      },
      primary: {
        ...PALLETTE,
      },
    }
  }
}
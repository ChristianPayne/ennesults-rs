import colors from "tailwindcss/colors";
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  plugins: [require('flowbite/plugin')],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        gray: {
          // Background color that flowbite uses.
          ...colors.zinc
        },
        white: {
          DEFAULT: "#f6f6f6"
        },
        // flowbite- svelte
        // primary: {
        //   50: '#FFF5F2',
        //   100: '#FFF1EE',
        //   200: '#FFE4DE',
        //   300: '#FFD5CC',
        //   400: '#FFBCAD',
        //   500: '#FE795D',
        //   600: '#EF562F',
        //   700: '#EB4F27',
        //   800: '#CC4522',
        //   900: '#A5371B'
        // }

        // zinc
        // primary: { "50": "#fafafa", "100": "#f4f4f5", "200": "#e4e4e7", "300": "#d4d4d8", "400": "#a1a1aa", "500": "#71717a", "600": "#52525b", "700": "#3f3f46", "800": "#27272a", "900": "#18181b" }
        // yellow
        // primary: { "50": "#fefce8", "100": "#fef9c3", "200": "#fef08a", "300": "#fde047", "400": "#facc15", "500": "#eab308", "600": "#ca8a04", "700": "#a16207", "800": "#854d0e", "900": "#713f12" }
        // teal
        primary: { "50": "#f0fdfa", "100": "#ccfbf1", "200": "#99f6e4", "300": "#5eead4", "400": "#2dd4bf", "500": "#14b8a6", "600": "#0d9488", "700": "#0f766e", "800": "#115e59", "900": "#134e4a" }


      }
    },
  },
}


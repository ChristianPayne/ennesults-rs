type ColorPalette = {
  label: string,
  value: string
}

export const colorPalettes: {[colorPaletteValue: string]: ColorPalette} = {
  "ennesults": {
    label: "Ennesults",
    value: ""
  },
  "tropical-paradise": {
    label: "Tropical Paradise",
    value: "tropical-paradise"
  },
  "cyberpunk-neon": {
    label: "Cyberpunk Neon",
    value: "cyberpunk-neon"
  },
  "pastel-kawaii": {
    label: "Pastel Kawaii",
    value: "pastel-kawaii"
  },
  "orange": {
    label: "Orange",
    value: "orange"
  },
  "steampunk-cogs": {
    label: "Steampunk Cogs",
    value: "steampunk-cogs"
  },
  "dracula": {
    label: "Dracula",
    value: "dracula"
  },
} as const;
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
  }
} as const;
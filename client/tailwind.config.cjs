const production = !process.env.ROLLUP_WATCH;

module.exports = {
  mode: 'jit',
  purge: {
    content: [
      "./src/**/*.svelte",
    ],
    enabled: production // disable purge in dev
  },
  darkMode: false,
  theme: {
    extend: {
      colors: {
        'c-yellow-1': '#EECF6D',
        'c-yellow-2': '#D5AC4E',
        'c-red': '#650D1B',
        'c-gray-1': '#7F7979',
        'c-gray-2': '#323031'
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
}

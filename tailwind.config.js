module.exports = {
  content: ["templates/*.html"],
  theme: {
     fontFamily: {
      "mono": ["Jetbrains Mono", "monospace"],
    },
  },
  plugins: [require("@catppuccin/tailwindcss")],
}

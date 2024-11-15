/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {
      fontSize: {
        "8xl": [
          "4.5rem",
          {
            lineHeight: "5rem",
          },
        ],
      },
      colors: {
        grey: {
          100: "#FFFFFF",
          90: "#F4F4F4",
          80: "#E3E3E3",
          70: "#A3A3A3",
          60: "#7D7D7D",
          50: "#4F4F4F",
          40: "#353535",
          30: "#2A2A2A",
          20: "#1D1D1D",
          10: "#0A0A0A",
        },
        primary: {
          100: "#FFF8F5",
          90: "#FFDACC",
          80: "#FFB699",
          70: "#FF9166",
          60: "#FF6C33",
          50: "#FF4700",
          40: "#CC3900",
          30: "#992B00",
          20: "#661D00",
          10: "#1A0700",
        },
        background: {
          main: "#FFF8F5",
          dark: "#030303",
        },
      },
      keyframes: {
        "fade-in": {
          from: {
            opacity: "0",
          },
          to: {
            opacity: "1",
          },
        },
        "scale-in": {
          from: {
            opacity: "0",
            scale: "0",
          },
          to: {
            opacity: "1",
            scale: "1",
          },
        },
        "translate-in-from-b": {
          from: {
            transform: "translateY(10px)",
            opacity: "0",
          },
          to: {
            transform: "translateY(0px)",
            opacity: "1",
          },
        },
      },
      animation: {
        "fade-in": "fade-in 0.3s forwards",
        "scale-in": "scale-in 0.3s forwards",
        "translate-in-from-b": "translate-in-from-b 0.3s forwards",
      },
    },
  },
  plugins: [],
};

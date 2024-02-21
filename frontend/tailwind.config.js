// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
      "./index.html",
      "./src/**/*.{rs,html}"
    ],
    theme: {
      extend: {
        colors: {
          'ct-dark-600': '#222',
          'ct-dark-200': '#e5e7eb',
          'ct-dark-100': '#f5f6f7',
          'ct-blue-600': '#2363eb',
          'ct-yellow-600': '#f9d13e',
          'ct-red-500': '#ef4444',
        },
        fontFamily: {
          Poppins: ['Poppins, sans-serif'],
        },
        container: {
          center: true,
          padding: '1rem',
          screens: {
            lg: '1125px',
            xl: '1125px',
            '2xl': '1125px',
          },
        },
      },
    },
    plugins: [],
  };

// https://github.com/trunk-rs/trunk/blob/6594336dead4b97e7f549dacc748eb3d4c0c160f/examples/yew-tailwindcss/tailwind.config.js
// module.exports = {
//     mode: "jit",
//     content: {
//       files: ["src/**/*.rs", "index.html"],
//     },
//     darkMode: "media", // 'media' or 'class'
//     theme: {
//       extend: {},
//     },
//     variants: {
//       extend: {},
//     },
//     plugins: [],
//   };
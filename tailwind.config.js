/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/{**/,}*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
      {
        app: {
          "primary": "#1d4ed8",
                    
          "secondary": "#2700ff",
                    
          "accent": "#00ddff",
                    
          "neutral": "#1f2937",
                    
          "base-100": "#fff",
                    
          "info": "#007cbd",
                    
          "success": "#22c55e",
                    
          "warning": "#d37d00",
                    
          "error": "#dc2626",
        },
      },
    ],
  },
}


/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                // Extend Tailwind with our CSS variables if needed, 
                // though we are using them directly in className="bg-[var(--color-bg-primary)]"
            }
        },
    },
    plugins: [],
}

/** @type {import('tailwindcss').Config} */
import catppuccin from '@catppuccin/daisyui'

module.exports = {
    content: ["./src/**/*.html"],
    plugins: [
        require("@tailwindcss/typography"),
        require("@catppuccin/tailwindcss")({
            defaultFlavour: "mocha",
        }),
        require('daisyui'),
    ],
    daisyui: {
        themes: [
            catppuccin("mocha"),
            catppuccin("macchiato"),
            catppuccin("frappe"),
            catppuccin("latte"),
            'dark',
        ]
    }
}

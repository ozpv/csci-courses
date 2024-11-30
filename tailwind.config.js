/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
		"./src/**/*.rs",
		"./htmx/**/*.{html,js}" 
	],
  },
/* catppuccin theme */
/* light: latte */
/* dark: mocha */
  theme: {
	colors: {
		rosewater: {
			DEFAULT: '#dc8a78',
			dark: '#f5e0dc',
		},
		flamingo: {
			DEFAULT: '#dd7878',
			dark: '#f2cdcd',
		},
		pink: {
			DEFAULT: '#ea76cb',
			dark: '#f5c2e7',
		},
		mauve: {
			DEFAULT: '#8839ef',
			dark: '#cba6f7',
		},
		red: {
			DEFAULT: '#d20f39',
			dark: '#f38ba8',
		},
		maroon: {
			DEFAULT: '#e64553',
			dark: '#eba0ac',
		},
		peach: {
			DEFAULT: '#fe640b',
			dark: '#fab387',
		},
		yellow: {
			DEFAULT: '#df8e1d',
			dark: '#f9e2af',
		},
		green: {
			DEFAULT: '#40a02b',
			dark: '#a6e3a1',
		},
		teal: {
			DEFAULT: '#179299',
			dark: '#94e2d5',
		},
		sky: {
			DEFAULT: '#04a5e5',
			dark: '#89dceb',
		},
		sapphire: {
			DEFAULT: '#209fb5',
			dark: '#74c7ec',
		},
		blue: {
			DEFAULT: '#1e66f5',
			dark: '#89b4fa',
		},
		lavender: {
			DEFAULT: '#7287fd',
			dark: '#b4befe',
		},
		text: { 
			DEFAULT: '#4c4f69',
			dark: '#cdd6f4',
		},
		'subtext': {
			100: '#5c5f77',
			DEFAULT: '#6c6f85',
		},
		'subtext-dark': {
			100: '#bac2de',
			DEFAULT: '#a6adc8',
		},
		'overlay': {
			200: '#7c7f93',
			100: '#8c8fa1',
			DEFAULT: '#9ca0b0',
		},
		'overlay-dark': {
			200: '#9399b2',
			100: '#7f849c',
			DEFAULT: '#6c7086',
		},
		'surface': {
			200: '#acb0be',
			100: '#bcc0cc',
			DEFAULT: '#ccd0da',
		},
		'surface-dark': {
			200: '#585b70',
			100: '#45475a',
			DEFAULT: '#313244',
		},
		base: {
			DEFAULT: '#eff1f5',
			dark: '#1e1e2e',
		},
		mantle: { 
			DEFAULT: '#e6e9ef',
			dark: '#181825',
		},
		crust: { 
			DEFAULT: '#dce0e8',
			dark: '#11111b',
		},
		transparent: 'transparent',
      	current: 'currentColor',
	},
	fontFamily: {
		inter: ['Inter', 'sans-serif'],
	},
  },
  plugins: [],
};

import rust from "@wasm-tool/rollup-plugin-rust";
import replace from 'rollup-plugin-re'

export default {
	input: 'index.js',
	output: {
		dir: 'dist',
	},
	plugins: [
		rust({
			verbose: true,
			inlineWasm: true
		}),
		replace({
			patterns: [
				{
					test: 'input = import.meta.url.replace(/\\.js$/, \'_bg.wasm\');',
					replace: ''
				}
			]
		})
	],
};

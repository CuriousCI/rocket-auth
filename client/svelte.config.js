// import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-static';
import sveltePreprocess from 'svelte-preprocess';
import makeAttractionsImporter from 'attractions/importer.js';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		sveltePreprocess({
			scss: {
				importer: makeAttractionsImporter({
					themeFile: 'D:/school/sistemi/kit/src/theme.scss',
				}),
				includePaths: ['D:/school/sistemi/kit/src'],
			},
			postcss: true
		}),
	],
	kit: {
		adapter: adapter({
			pages: 'public',
			assets: 'public',
			fallback: null,
			precompress: false
		}),

		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte'
	}
};

export default config;
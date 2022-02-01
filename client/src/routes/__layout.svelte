<slot />

<style global type="text/postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	input:not(.outline):invalid {
		background-color: whitesmoke !important;
		border-bottom-color: grey !important;
	}

	input:not(.outline):invalid + :global svg.item {
		stroke: grey !important;
		color: grey !important;
	}
</style>

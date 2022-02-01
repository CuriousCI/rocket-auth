<script>
	export let name,
		error = undefined,
		required = false;
</script>

<main class="w-72">
	<div class="relative mb-2 px-2 box-border field-name">
		{name}
		{#if required}
			<span class="required">*</span>
		{/if}
		{#if error}
			<span class="absolute right-0 bottom-0 italic text-sm error">
				{error}
			</span>
		{/if}
	</div>
	<div class="mb-5 slot">
		<slot />
	</div>
</main>

<style lang="sass">
	@use 'theme.scss'

	.required 
		color: theme.$main

	.error 
		color: #ec1c24
</style>

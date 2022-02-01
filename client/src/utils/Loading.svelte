<script>
	import { Loading } from 'attractions';

	export let loading;
</script>

<main class="relative">
	<slot />
	{#if loading}
		<div
			class="absolute top-0 left-0 w-full h-full grid place-items-center z-50 bg-white/50"
		>
			<Loading />
		</div>
	{/if}
</main>

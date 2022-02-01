<script>
	import { H1 } from 'attractions';
	import { onMount } from 'svelte';

	let email = '';
	onMount(async () => {
		const response = await fetch('/email', { method: 'GET' });
		email = await response.text();
	});
</script>

<main class="w-full h-full grid place-items-center">
	<H1>Benvenuto al registro elettronico {email}!</H1>
</main>

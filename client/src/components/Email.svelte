<script>
	import { TextField } from 'attractions';
	import { MailIcon } from 'svelte-feather-icons';
	import Field from './Field.svelte';
	export function isEmailValid(email) {
		return /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/.test(email);
	}

	export let email = '';
	let isFocused = false;
</script>

<Field
	name="E-mail"
	error={email && !isFocused && !isEmailValid(email) && 'E-mail non valida!'}
	required
>
	<TextField
		bind:value={email}
		class="w-72"
		on:focus={() => (isFocused = true)}
		on:blur={() => (isFocused = false)}
		name="email"
		withItem
		required
		autocomplete="email"
	>
		<MailIcon size="24" class="item" />
	</TextField>
</Field>

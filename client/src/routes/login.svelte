<script>
	import { H2, Divider, Card, Button, TextField } from 'attractions';
	import { LockIcon } from 'svelte-feather-icons';
	import Field from '../components/Field.svelte';
	import Email from '../components/Email.svelte';
</script>

<main class="w-full h-full grid place-items-center">
	<Card class="w-min">
		<H2 class="text-center">registro elettronico</H2>
		<Divider />
		<br />
		<form method="post" action="/request/login" class="w-min">
			<input class="hidden" type="text" name="id" value="0" />
			<Email />
			<Field name="Password">
				<TextField
					name="password"
					type="password"
					withItem
					required
					autocomplete="current-password"
				>
					<LockIcon size="24" class="item" />
				</TextField>
			</Field>
			<Button type="submit" class="w-full justify-center" outline>
				LogIn
			</Button>
		</form>
	</Card>
</main>

<script>
	import { Modal, Dialog } from 'attractions';
	import { AlertTriangleIcon } from 'svelte-feather-icons';

	export let warning = false;
</script>

<Modal bind:open={warning} let:closeCallback>
	<Dialog danger title="Attenzione!" {closeCallback}>
		<div slot="title-icon">
			<AlertTriangleIcon size="24" />
		</div>
		<slot />
	</Dialog>
</Modal>

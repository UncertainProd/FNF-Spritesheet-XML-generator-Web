<!-- https://svelte.dev/examples/modal -->
<script lang="ts">
	export let showModal: boolean;
	export let onClose = ()=>{};

	let dialog: HTMLDialogElement;

	export let closeButtonMsg = "Close";

	$: if (dialog){
		if(showModal)
		{
			dialog.showModal();
		}
		else
		{
			dialog.close();
		}
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
	bind:this={dialog}
	on:close={() => { showModal = false; onClose();}}
	on:click|self={() => dialog.close()}
>
	<div on:click|stopPropagation>
		<slot name="header" />
		<hr />
		<slot />
		<hr />
		<slot name="accept-btn" />
		<hr />
		<!-- svelte-ignore a11y-autofocus -->
		<button autofocus on:click={() => dialog.close()}>{ closeButtonMsg }</button>
	</div>
</dialog>

<style>
	dialog {
		/* max-width: 40em; */
		/* max-height: 80vh; */
		border-radius: 0.2em;
		border: none;
		padding: 0;
		background-color: var(--background);
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
	dialog > div {
		padding: 1em;
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	button {
		display: inline-block;
	}
</style>

<script lang="ts">
    import AppTab from "./components/AppTab.svelte";
    import IcongridGenerator from "./tabs/IcongridGenerator.svelte";
    import SpritesheetGenerator from "./tabs/SpritesheetGenerator.svelte";
	import type { Wasm_T } from './global';

	export let wasm: Wasm_T;
	let characterName: string;

	let curChoice = "spsh";

	// let test = wasm.greet("Friend!!");
</script>

<main>
	<h1 id="title">FNF Spritesheet and XML Generator Web (experimental version)</h1>

	<label for="dark-mode">
		<input type="checkbox" name="dark-mode" id="dark-mode" checked on:change={() => { document.body.classList.toggle('light-mode') }}>
		Dark Mode
	</label>
	
	<div id="character-name-control">
		<label for="char-name">
			Character Name:
		</label>
		<input type="text" name="char-name" id="char-name" bind:value={characterName} />
	</div>

	<div id="app-tabs">
		<AppTab id="spsh" bind:curChoice={curChoice} text="Spritesheet Generation" />
		<AppTab id="icongrid" bind:curChoice={curChoice} text="Icongrid Generation" />
	</div>

	<div id="tab-container">
		{#if curChoice == "spsh"}
			<SpritesheetGenerator charname={characterName} wasm={wasm} />
		{:else if curChoice == "icongrid"}
			<IcongridGenerator bind:charname={characterName} wasm={wasm} />
		{/if}
	</div>
</main>

<style>
	#title {
		text-align: center;
		padding-left: 5px;
		padding-right: 5px;
	}

	#app-tabs {
		display: grid;
		grid-template-columns: 1fr 1fr;
		height: 3rem;
	}

	#tab-container {
		border: 2px solid var(--border-color);
	}

	#character-name-control {
		display: grid;
		grid-template-columns: 9rem 80%;
	}

	#character-name-control > label {
		display: flex;
		align-items: center;
		transform: translateY(-10%);
	}
</style>
<script>
    export let id;
    export let curChoice;
    export let text;
</script>

<label for="tab-{id}" class="tabs" style="background-color: {curChoice == id ? 'blue' : 'white'};">
    <input type="radio" bind:group={curChoice} name="app-choice" id="tab-{id}" value={id} style="display: none;" />
    {text}
</label>

<style>
    .tabs {
        display: inline;
        text-align: center;
	}

	#app-choice-icongrid, #app-choice-spsh {
		display: none;
	}
</style>
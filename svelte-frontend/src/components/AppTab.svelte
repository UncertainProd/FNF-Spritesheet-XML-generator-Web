<script lang="ts">
    export let id: string;
    export let curChoice: string;
    export let text: string;

    let btn: HTMLButtonElement;

    $: {
        if(btn)
        {
            if(id === curChoice)
            {
                btn.classList.add('selected');
            }
            else
            {
                btn.classList.remove('selected');
            }
        }
    }
</script>

<button class="selected" bind:this={btn} on:click={() => { curChoice = id; }}>
    {text}
</button>

<style>
    .selected {
        background-color: var(--primary-button);
        color: black;
    }
</style>
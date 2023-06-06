<script lang="ts">
    import SpriteFrame from '../components/SpriteFrame.svelte';
    import SpritesheetGenControls from '../components/SpritesheetGenControls.svelte';
    import type { Wasm_T } from '../global';
    import { spriteframes } from '../stores';

    export let wasm: Wasm_T;
    export let charname: string;

    wasm.greet('hi');
</script>


<div id="sprite-frames">
    <!-- <RubberBandSelector /> -->
    {#each $spriteframes as dat (dat.sprId)}
        <SpriteFrame bind:frameInfo={dat} />
    {/each}
</div>


<SpritesheetGenControls {charname} wasm={wasm} />


<style>
    #sprite-frames {
        border: 2px solid grey;
        margin: 2px;
        height: 60vh;
        overflow-y: scroll;
    }
</style>
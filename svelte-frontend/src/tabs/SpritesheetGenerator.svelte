<script lang="ts">
    import SpriteFrame from '../components/SpriteFrame.svelte';
    import SpritesheetGenControls from '../components/SpritesheetGenControls.svelte';
    import type { Wasm_T } from '../global';
    import { spriteframes } from '../stores';

    export let wasm: Wasm_T;
    export let charname: string;

    let rangeSelectionMode = false;
    let rangeStartIdx: number | null = null;
    let rangeEndIdx: number | null = null;

    function onRangeClick(index: number)
    {
        if(rangeSelectionMode)
        {
            if(rangeStartIdx === null)
            {
                // we have selected the start of the range
                rangeStartIdx = index;
                return;
            }
            if(rangeEndIdx === null)
            {
                // we have selected the end of the range
                rangeEndIdx = index;
                rangeSelectionMode = false;
                document.getElementById('sprite-frames').style.backgroundColor = null;
                // loop and select this range
                spriteframes.update((prev) => {
                    let start: number, end: number;
                    if(rangeStartIdx <= rangeEndIdx)
                    {
                        start = rangeStartIdx;
                        end = rangeEndIdx;
                    }
                    else
                    {
                        start = rangeEndIdx;
                        end = rangeStartIdx;
                    }
                    for(let i = start; i <= end; i++)
                    {
                        prev[i].selected = !prev[i].selected;
                    }
                    return prev;
                });
                rangeStartIdx = null;
                rangeEndIdx = null;
                return;
            }
        }
    }
</script>


<div id="sprite-frames">
    <!-- <RubberBandSelector /> -->
    {#each $spriteframes as dat, idx (dat.sprId)}
        <SpriteFrame {rangeSelectionMode} onRangeClick={() => { onRangeClick(idx); }} bind:frameInfo={dat} />
    {/each}
</div>


<SpritesheetGenControls bind:rangeSelectionMode {charname} wasm={wasm} />


<style>
    #sprite-frames {
        border: 2px solid grey;
        margin: 2px;
        height: 60vh;
        overflow-y: scroll;
    }
</style>
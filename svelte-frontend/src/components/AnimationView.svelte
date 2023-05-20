<script lang="ts">
    import Modal from "./Modal.svelte";
    import { AnimationController } from '../animationcontroller';
    import { onMount } from "svelte";
    import type { SpriteFrameData } from "../spriteframedata";
    import { spriteframes } from '../stores'

    export let showView = false;

    let canvasElement:HTMLCanvasElement = null;

    let selectedOption: string;
    
    let prefixes = new Map<string, SpriteFrameData[]>();
    let prefixArray = [];

    let animController:AnimationController = null;
    onMount(()=>{
        animController = new AnimationController(canvasElement.getContext('2d'));
    });

    $: {
        if(animController !== null)
        {
            if(showView)
            {
                console.log("Clearing canvas: ", animController.isPlaying);
                animController.clearCanvas();
                // if(animController.isPlaying)
                // {
                //     console.log("Clearing canvas: ", animController.isPlaying);
                //     animController.clearCanvas();
                // }
            }
            else
            {
                animController.stopAnimation();
            }
            animController = animController; // to update svelte state
        }
    }

    $: {
        if(showView)
        {
            prefixes = new Map();
            prefixArray = [];
            for(let spr of $spriteframes)
            {
                if(!prefixes.has(spr.animationPrefix))
                {
                    prefixes.set(spr.animationPrefix, [ spr ]);
                }
                else
                {
                    let frames = prefixes.get(spr.animationPrefix);
                    prefixes.set(spr.animationPrefix, [ ...frames, spr ]);
                }
                
            }
            for(let k of prefixes.keys())
            {
                prefixArray = [ ...prefixArray, k ];
            }
        }
    }

    async function playAnimation()
    {
        if(animController.animId === null)
        {
            await animController.initFrames(prefixes.get(selectedOption));
            // animController.initFrames(prefixes.get(selectedOption));
            animController.play();
            animController.context.canvas.width = window.innerWidth * 0.9;
            animController.context.canvas.height = window.innerHeight * 0.9;
        }
        else
        {
            animController.stopAnimation();
        }
        console.log("Anim is " + animController.animId);
        animController = animController; // update ui
    }
</script>

<Modal bind:showModal={showView}>
    <h4 slot="header">View Animation</h4>
    <div>
        <select on:change={()=> {animController.stopAnimation(); animController = animController}} name="anim-prefix" id="anim-prefix" bind:value={selectedOption}>
            {#each prefixArray as prefix (prefix)}
                <option value="{prefix}">{prefix}</option>
            {/each}
        </select>
        <button on:click={playAnimation}>{ (animController?.isPlaying) ? 'Stop' : 'Play' } Animation</button>
        {#if animController}
            <label for="fps">
                <input type="number" name="fps" id="fps" min="1" max="120" bind:value={animController.fps} />
                FPS
            </label>
            <label for="fps">
                <input type="number" name="anim-scale" id="anim-scale" min="0.01" max="4" bind:value={animController.animationScale} step="0.01" />
                Animation Scale
            </label>
        {/if}
    </div>
    <canvas bind:this={canvasElement}></canvas>
</Modal>

<style>

</style>
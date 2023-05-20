<script lang="ts">
    import { onMount } from 'svelte';
    import type { SpriteFrameData } from '../spriteframedata';
    import { AnimationController } from '../animationcontroller';
    import { spriteframes } from '../stores';
    let frameCanvas: HTMLCanvasElement;
    let frameDiv: HTMLDivElement;
    export let frameInfo: SpriteFrameData;

    let spriteframesize = 120;
    let showImage = false;

    let animController: AnimationController;
    onMount(()=>{
        const obs = new IntersectionObserver(async (entries, observer)=>{
            const canv = entries[0];
            if(canv.isIntersecting && canv.intersectionRatio >= 0.20)
            {
                if(!showImage)
                {
                    // test_div.innerText += ' | showing';
                    await loadImage();
                }
            }
            else
            {
                showImage = false;
            }
        }, {
            root: document.getElementById('sprite-frames'),
            rootMargin: "0px",
            threshold: [0.1, 0.2] // [0.25, 0.5, 0.75]
        });
        obs.observe(frameDiv);


        // set up animation controller to draw the frame
        animController = new AnimationController(frameCanvas.getContext('2d'));
    });

    $: {
        if(showImage && frameInfo._changed)
        {
            loadImage().then(()=>{});
            frameInfo._changed = false;
            // console.log("Loading imageee for: " + frameInfo.sprId);
        }
    }

    async function loadImage() {
        await animController.initFrames([ frameInfo ]);
        animController.clearCanvas();
        animController.animationScale = spriteframesize/Math.max(frameInfo.frameRect.frameWidth, frameInfo.frameRect.frameHeight);
        animController.drawImage(animController.curImgs[0], frameInfo, false);
        showImage = true;
    }

    function handleClick()
    {
        frameInfo.selected = !frameInfo.selected;
    }

    function onDelete(sprframe: SpriteFrameData)
    {
        const ind = $spriteframes.indexOf(sprframe);
        if(ind !== -1)
        {
            $spriteframes.splice(ind, 1);
            spriteframes.set($spriteframes);
        }
        else
        {
            console.log("[ERROR] Could not find element: " + sprframe);
        }
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={handleClick} bind:this={frameDiv} class="frame-container" style="width: {spriteframesize}px;">
    <input type="checkbox" name="selected-chkbox" id="selected-chkbox" checked={frameInfo.selected} />
    <canvas style="display: {(showImage) ? 'block' : 'none'};" width="{spriteframesize}px" height="{spriteframesize}px" bind:this={frameCanvas}>
        Rendering Error!
        Please use a browser that supports canvas rendering! (like Chrome or Firefox)
    </canvas>
    {#if !showImage}
        <div style="width: {spriteframesize}; height: {spriteframesize}px;"></div>
    {/if}
    <p>  {frameInfo.animationPrefix}</p>
    <button on:click|stopPropagation={()=>{ onDelete(frameInfo) }}>Delete</button>
</div>

<style>
    .frame-container {
        border: 4px solid black;
        display: inline-block;
        overflow-x: hidden;
    }

    .frame-container:hover {
        border-color: red;
    }
</style>
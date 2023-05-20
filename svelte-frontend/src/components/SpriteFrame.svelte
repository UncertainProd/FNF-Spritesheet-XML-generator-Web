<script lang="ts">
    // import { base64EncArr } from '../b64utils'
    import { onMount } from 'svelte';
    import { makeImage } from '../utils';
    import type { SpriteFrameData } from '../spriteframedata';
    import { AnimationController } from '../animationcontroller';
    let frameCanvas: HTMLCanvasElement;
    let frameDiv: HTMLDivElement;
    // let test_div;
    export let frameInfo: SpriteFrameData;
    export let onDelete;

    let spriteframesize = 120;
    let showImage = false;
    // let frameSelected = false;

    // $: if(showImage) { console.log("Showing image") } else { console.log("Hiding image") }
    // $: {
    //     if(frameInfo && showImage)
    //     {
    //         loadImage().then(()=>{});
    //     }
    // }
    let animController: AnimationController;
    onMount(()=>{
        const obs = new IntersectionObserver(async (entries, observer)=>{
            const canv = entries[0];
            // test_div.innerText = canv.intersectionRatio.toPrecision(4);
            if(canv.isIntersecting && canv.intersectionRatio >= 0.20)
            {
                // console.log("This is visible: " + JSON.stringify(frameInfo.rect) + " | Intersection Ratio = " + canv.intersectionRatio);
                if(!showImage)
                {
                    // test_div.innerText += ' | showing';
                    await loadImage();
                }
                else
                {
                    // test_div.innerText += ' | shown';
                }
            }
            else
            {
                // console.log("Ratio: " + canv.intersectionRatio);
                // test_div.innerText += ' | hiding';
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
        if(showImage)
        {
            loadImage().then(()=>{});
            // console.log("Loading imageee");
        }
        if(frameInfo)
        {
            console.log("Spriteframe: " + frameInfo.sprId);
        }
    }

    async function loadImage() {
        await animController.initFrames([ frameInfo ]);
        animController.clearCanvas();
        animController.animationScale = spriteframesize/Math.max(frameInfo.frameRect.frameWidth, frameInfo.frameRect.frameHeight);
        animController.drawImage(animController.curImgs[0], frameInfo, false, false);

        animController.context.clearRect(
            frameInfo.frameRect.frameWidth * animController.animationScale * frameInfo.transform.scaleX, 
            0, 
            animController.context.canvas.width, 
            animController.context.canvas.height
        );
        animController.context.clearRect(
            0, 
            frameInfo.frameRect.frameHeight * animController.animationScale * frameInfo.transform.scaleY, 
            animController.context.canvas.width, 
            animController.context.canvas.height
        );
        showImage = true;
    }

    function handleClick()
    {
        frameInfo.selected = !frameInfo.selected;
        frameInfo = frameInfo;
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={handleClick} bind:this={frameDiv} class="frame-container" style="width: {spriteframesize}px;">
    <input type="checkbox" name="selected-chkbox" id="selected-chkbox" bind:checked={frameInfo.selected} />
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
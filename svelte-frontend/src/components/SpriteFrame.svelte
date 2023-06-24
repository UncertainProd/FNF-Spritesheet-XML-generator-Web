<script lang="ts">
    import { onMount } from 'svelte';
    import type { SpriteFrameData } from '../spriteframedata';
    import { AnimationController } from '../animationcontroller';
    import { spriteframes, spritesheet_map } from '../stores';
    let frameCanvas: HTMLCanvasElement;
    let frameDiv: HTMLDivElement;
    export let frameInfo: SpriteFrameData;
    export let onRangeClick: Function;
    export let rangeSelectionMode: boolean;

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
                    // showing image
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
        }

        if(frameDiv)
        {
            if(frameInfo.selected)
            {
                frameDiv.classList.add('frame-selected');
            }
            else
            {
                frameDiv.classList.remove('frame-selected');
            }
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
        if(onRangeClick != null)
        {
            onRangeClick();
        }

        if(!rangeSelectionMode)
        {
            frameInfo.selected = !frameInfo.selected;
        }
        else
        {
            let _count = 0;
            const hasSelectedClass = frameDiv.classList.contains('frame-selected');
            let interId = setInterval(() => {
                if(_count >= 2)
                {
                    clearInterval(interId);
                    if(hasSelectedClass)
                    {
                        frameDiv.classList.add('frame-selected');
                    }
                    else
                    {
                        frameDiv.classList.remove('frame-selected');
                    }
                }
                frameDiv.classList.toggle('frame-selected');
                _count++;
            }, 150);
        }
    }

    function onDelete(sprframe: SpriteFrameData)
    {
        const ind = $spriteframes.indexOf(sprframe);
        if(ind !== -1)
        {
            $spriteframes.splice(ind, 1);
            spriteframes.set($spriteframes);

            if(frameInfo.type === 'spritesheet_frame')
            {
                const hash = frameInfo.spritesheetId;
                const [spshdata, count] = $spritesheet_map.get(hash);
                const newCount = count - 1;
                if(newCount > 0)
                {
                    spritesheet_map.update((prev) => {
                        prev.set(hash, [spshdata, newCount])
                        return prev;
                    });
                }
                else
                {
                    spritesheet_map.update((prev) => {
                        prev.delete(hash);
                        return prev;
                    });
                }
            }
        }
        else
        {
            console.log("[ERROR] Could not find element: " + sprframe);
        }
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={handleClick} bind:this={frameDiv} class="frame-container frame-selected" style="width: {spriteframesize}px;">
    <input type="checkbox" name="selected-chkbox" id="selected-chkbox" checked={frameInfo.selected} />
    <canvas style="display: {(showImage) ? 'block' : 'none'};" width="{spriteframesize}px" height="{spriteframesize}px" bind:this={frameCanvas}>
        Rendering Error!
        Please use a browser that supports canvas rendering! (like Chrome or Firefox)
    </canvas>
    {#if !showImage}
        <div style="width: {spriteframesize}; height: {spriteframesize}px;"></div>
    {/if}
    <p>  {frameInfo.animationPrefix}  </p>
    <button on:click|stopPropagation={()=>{ onDelete(frameInfo) }}>Delete</button>
</div>

<style>
    .frame-container {
        border: 4px solid var(--frame-border-color);
        display: inline-block;
        overflow-x: hidden;
    }

    .frame-container:hover {
        border-color: var(--frame-hovered);
    }

    .frame-selected {
        background-color: var(--frame-selected);
    }
</style>
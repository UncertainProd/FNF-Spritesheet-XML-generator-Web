<script lang="ts">
    import Modal from "./Modal.svelte";
    import type { SpriteFrameData } from "../spriteframedata";
    import { AnimationController } from "../animationcontroller";
    import { onMount } from "svelte";
    import { spriteframes } from '../stores';

    export let showView = false;

    let currSelectedRow:number|null = null;
    let curSprFrame:SpriteFrameData = null;

    let _animScaleInput:HTMLInputElement = null;
    let _frameXInput:HTMLInputElement = null;
    let _frameYInput:HTMLInputElement = null;
    let _frameWidthInput:HTMLInputElement = null;
    let _frameHeightInput:HTMLInputElement = null;
    let _newWidthInput:HTMLInputElement = null;
    let _newHeightInput:HTMLInputElement = null;
    let _flipXInput:HTMLInputElement = null;
    let _flipYInput:HTMLInputElement = null;

    let rows:HTMLTableRowElement[] = [];
    let canvasElement:HTMLCanvasElement = null;
    let animController:AnimationController = new AnimationController(null);
    onMount(()=>{
        animController = new AnimationController(canvasElement.getContext('2d'));
        _animScaleInput.value = ''+animController.animationScale;
    });

    async function drawFrameWithBox(sprframe: SpriteFrameData)
    {
        if(canvasElement)
        {
            await animController.initFrames([ sprframe ]);
            animController.clearCanvas();
            animController.drawImage(animController.curImgs[0], sprframe, true);
        }
    }

    function _onAnimationScaleChange(e: Event)
    {
        animController.animationScale = parseFloat((e.target as HTMLInputElement).value);
        drawFrameWithBox(curSprFrame).then(()=>{});
        animController = animController;
    }

    function _handleRowClick(index: number)
    {
        currSelectedRow = index;
        curSprFrame = $spriteframes[index];
        for(const r of rows)
        {
            if(r)
            {
                r.classList.remove('cell-selected');
            }
        }
        rows[index].classList.add('cell-selected');
        _frameXInput.value = '' + curSprFrame.frameRect.frameX;
        _frameYInput.value = '' + curSprFrame.frameRect.frameY;
        _frameWidthInput.value = '' + curSprFrame.frameRect.frameWidth;
        _frameHeightInput.value = '' + curSprFrame.frameRect.frameHeight;
        _newWidthInput.value = '' + curSprFrame.transform.newWidth;
        _newHeightInput.value = '' + curSprFrame.transform.newHeight;
        _flipXInput.checked = curSprFrame.transform.flipX;
        _flipYInput.checked = curSprFrame.transform.flipY;
    }

    function onValueChange(e: Event)
    {
        curSprFrame.frameRect.frameX = +_frameXInput.value;
        curSprFrame.frameRect.frameY = +_frameYInput.value;
        curSprFrame.frameRect.frameWidth = +_frameWidthInput.value;
        curSprFrame.frameRect.frameHeight = +_frameHeightInput.value;
        curSprFrame.transform.newWidth = +_newWidthInput.value;
        curSprFrame.transform.newHeight = +_newHeightInput.value;
        curSprFrame.transform.flipX = _flipXInput.checked;
        curSprFrame.transform.flipY = _flipYInput.checked;
        curSprFrame._changed = true;
        spriteframes.set($spriteframes);
        drawFrameWithBox(curSprFrame).then(()=>{});
    }
</script>


<Modal bind:showModal={showView} closeButtonMsg={"Save and Close"}>
    <div slot="header">
        <h4>XML View</h4>
    </div>
    <div id="view-container">
        <div id="table-div">
            <table class="not-selectable">
                <thead>
                    <th></th>
                    <th>Prefix</th>
                    <th>Width</th>
                    <th>Height</th>
                    <th>Frame X</th>
                    <th>Frame Y</th>
                    <th>Frame Width</th>
                    <th>Frame Height</th>
                    <th>New Image Width</th>
                    <th>New Image Height</th>
                    <th>flip X</th>
                    <th>flip Y</th>
                </thead>

                <tbody>
                    {#each $spriteframes as spr,i (spr.sprId)}
                        <tr bind:this={rows[i]} on:click|stopPropagation={async (_)=>{ _handleRowClick(i); await drawFrameWithBox(spr); }}>
                            <td>
                                <input type="checkbox" name="select-{spr.sprId}" id="select-{spr.sprId}" on:change={async (_)=>{ _handleRowClick(i); await drawFrameWithBox(spr); }}>
                            </td>
                            <td>{spr.animationPrefix}</td>
                            <td>{spr.rect.width}</td>
                            <td>{spr.rect.height}</td>
                            <td>{spr.frameRect.frameX}</td>
                            <td>{spr.frameRect.frameY}</td>
                            <td>{spr.frameRect.frameWidth}</td>
                            <td>{spr.frameRect.frameHeight}</td>
                            <td>{spr.transform.newWidth}</td>
                            <td>{spr.transform.newHeight}</td>
                            <td>{spr.transform.flipX}</td>
                            <td>{spr.transform.flipY}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
        <div>
            <div class="controls-horizontal">
                <label class="xmlview-input" for="frame-x">
                    Frame X
                    <input type="number" name="frame-x" id="frame-x" on:input={onValueChange} bind:this={_frameXInput}>
                </label>
                <label class="xmlview-input" for="frame-y">
                    Frame Y
                    <input type="number" name="frame-y" id="frame-y" on:input={onValueChange} bind:this={_frameYInput}>
                </label>
                <label class="xmlview-input" for="frame-width">
                    Frame Width
                    <input type="number" name="frame-width" id="frame-width" on:input={onValueChange} bind:this={_frameWidthInput}>
                </label>
                <label class="xmlview-input" for="frame-height">
                    Frame Height
                    <input type="number" name="frame-height" id="frame-height" on:input={onValueChange} bind:this={_frameHeightInput}>
                </label>
                <label class="xmlview-input" for="scale-x">
                    Image Width
                    <input type="number" name="scale-x" id="scale-x" min="1" on:input={onValueChange} bind:this={_newWidthInput}>
                </label>
                <label class="xmlview-input" for="scale-y">
                    Image Height
                    <input type="number" name="scale-y" id="scale-y" min="1" on:input={onValueChange} bind:this={_newHeightInput}>
                </label>
                <label class="xmlview-input" for="flip-x">
                    Flip X
                    <input type="checkbox" name="flip-x" id="flip-x" on:input={onValueChange} bind:this={_flipXInput}>
                </label>
                <label class="xmlview-input" for="flip-y">
                    Flip Y
                    <input type="checkbox" name="flip-y" id="flip-y" on:input={onValueChange} bind:this={_flipYInput}>
                </label>

                <label class="xmlview-input" for="animation-scale">
                    Animation Scale
                    <input bind:this={_animScaleInput} type="number" name="animation-scale" id="animation-scale" min="0.01" step="0.01" on:input={_onAnimationScaleChange}>
                </label>
            </div>
            <div class="canvas-div">
                <canvas bind:this={canvasElement} width="550px" height="500px"></canvas>
            </div>
        </div>
    </div>
</Modal>

<style>
    .controls-horizontal {
        display: flex;
        flex-direction: row;
        width: 30rem;
    }

    .xmlview-input {
        width: 20%;
        margin-right: 1rem;
    }

    .xmlview-input > input {
        width: 5rem;
    }

    .canvas-div {
        height: 70vh;
        width: 600px;
        overflow-y: scroll;
    }

    #view-container {
        display: grid;
        grid-template-columns: 500px 100%;
        height: 80vh;
        /* width: 700px; */
        width: 90vw;
    }

    tr:global(.cell-selected) {
        background-color: var(--light-accent);
    }

    #table-div {
        height: inherit;
        overflow-y: scroll;
        position: relative;
    }

    table {
        border: 2px solid black;
    }

    td, th {
        border: 1px solid black;
    }

    canvas {
        border: 2px solid black;
    }

    .not-selectable {
        -webkit-touch-callout: none;
        -webkit-user-select: none;
        -khtml-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        user-select: none;
    }
</style>
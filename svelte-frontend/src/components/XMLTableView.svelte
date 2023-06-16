<script lang="ts">
    import Modal from "./Modal.svelte";
    import type { SpriteFrameData } from "../spriteframedata";
    import { AnimationController } from "../animationcontroller";
    import { onMount } from "svelte";
    import { spriteframes } from '../stores';
    import { Parser } from 'expr-eval';

    export let showView = false;

    let currSelectedRow:number|null = null;
    let curSprFrame:SpriteFrameData = null;

    // const frameProperties = [
    //     'frameX',
    //     'frameY',
    //     'frameWidth',
    //     'frameHeight',
    // ];
    // const transformProperties = [
    //     'newWidth',
    //     'newHeight',
    // ];
    // const boolProperties = [
    //     'flipX',
    //     'flipY',
    // ];

    // type Controls = {
    //     frameX: HTMLInputElement
    //     frameY: HTMLInputElement
    //     frameWidth: HTMLInputElement
    //     frameHeight: HTMLInputElement
    //     newWidth: HTMLInputElement
    //     newHeight: HTMLInputElement
    //     flipX: HTMLInputElement
    //     flipY: HTMLInputElement
    // };

    // const frameControls: Controls = {
    //     frameX: null,
    //     frameY: null,
    //     frameWidth: null,
    //     frameHeight: null,
    //     newWidth: null,
    //     newHeight: null,
    //     flipX: null,
    //     flipY: null
    // };
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

    let showGroupChangeModal = false;

    onMount(()=>{
        animController = new AnimationController(canvasElement.getContext('2d'));
        _animScaleInput.value = ''+animController.animationScale;
    });

    function onKeyPressTable(ev: KeyboardEvent)
    {
        ev.preventDefault();
        switch (ev.key)
        {
            case 'ArrowUp':
                let nextRow = (currSelectedRow - 1 >= 0) ? currSelectedRow - 1 : $spriteframes.length - 1;
                _handleRowClick(nextRow);
                break;
            case 'ArrowDown':
                let prevRow = (currSelectedRow + 1) % $spriteframes.length;
                _handleRowClick(prevRow);
                break;
            case 'Home':
                _handleRowClick(0);
                break;
            case 'End':
                _handleRowClick($spriteframes.length - 1);
                break;
            default:
                // console.log(`Selected rows are at: ${selectedRows}`);
                break;
        }
        drawFrameWithBox($spriteframes[currSelectedRow]).then(() => {}); // too lazy to make this whole function async :p
        rows[currSelectedRow].scrollIntoView(false);
    }

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
                r.style.backgroundColor = 'white';
            }
        }
        rows[index].style.backgroundColor = 'blue';

        // for(const prop of frameProperties)
        // {
        //     frameControls[prop].value = '' + curSprFrame.frameRect[prop];
        // }
        // for(const prop of transformProperties)
        // {
        //     frameControls[prop].value = '' + curSprFrame.transform[prop];
        // }
        // for(const prop of boolProperties)
        // {
        //     frameControls[prop].checked = curSprFrame.transform[prop];
        // }

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
        // for(const prop of frameProperties)
        // {
        //     curSprFrame.frameRect[prop] = frameControls[prop].value;
        // }
        // for(const prop of transformProperties)
        // {
        //     curSprFrame.transform[prop] = frameControls[prop].value;
        // }
        // for(const prop of boolProperties)
        // {
        //     curSprFrame.transform[prop] = frameControls[prop].checked;
        // }
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

    let selectedRows:number[] = [];
    function onRowChecked(e:Event, idx: number)
    {
        const t = e.target as HTMLInputElement;
        if(t.checked)
        {
            selectedRows = [ ...selectedRows, idx ];
        }
        else
        {
            const ind = selectedRows.indexOf(idx);
            if(ind >= 0)
            {
                selectedRows.splice(ind, 1);
            }
            selectedRows = selectedRows;
        }
    }

    // let modificationControls: Controls = {
    //     frameX: null,
    //     frameY: null,
    //     frameWidth: null,
    //     frameHeight: null,
    //     newWidth: null,
    //     newHeight: null,
    //     flipX: null,
    //     flipY: null,
    // };
    let _modifyframeXInput:HTMLInputElement = null;
    let _modifyframeYInput:HTMLInputElement = null;
    let _modifyframeWidthInput:HTMLInputElement = null;
    let _modifyframeHeightInput:HTMLInputElement = null;
    let _modifynewWidthInput:HTMLInputElement = null;
    let _modifynewHeightInput:HTMLInputElement = null;
    let _modifyflipXInput:HTMLInputElement = null;
    let _modifyflipYInput:HTMLInputElement = null;
    function applyModification()
    {
        const newFrameXValueRaw = _modifyframeXInput.value;
        const newFrameYValueRaw = _modifyframeYInput.value;
        const newFrameWidthValueRaw = _modifyframeWidthInput.value;
        const newFrameHeightValueRaw = _modifyframeHeightInput.value;
        const newImgWidthValueRaw = _modifynewWidthInput.value;
        const newImgHeightValueRaw = _modifynewHeightInput.value;
        const newFlipXValueRaw = _modifyflipXInput.value;
        const newFlipYValueRaw = _modifyflipYInput.value;

        const mathParser = new Parser({ operators: { logical: true } });
        const frameXExpr = mathParser.parse(newFrameXValueRaw);
        const frameYExpr = mathParser.parse(newFrameYValueRaw);
        const frameWidthExpr = mathParser.parse(newFrameWidthValueRaw);
        const frameHeightExpr = mathParser.parse(newFrameHeightValueRaw);
        const imgWidthExpr = mathParser.parse(newImgWidthValueRaw);
        const imgHeightExpr = mathParser.parse(newImgHeightValueRaw);
        const flipXExpr = mathParser.parse(newFlipXValueRaw);
        const flipYExpr = mathParser.parse(newFlipYValueRaw);
        
        spriteframes.update((prev) => {
            for(const idx of selectedRows)
            {
                console.log(typeof frameXExpr.evaluate({ value: prev[idx].frameRect.frameX }));
                console.log(typeof flipXExpr.evaluate({ value: (prev[idx].transform.flipX) ? 1 : 0 }));
                console.log(flipXExpr.evaluate({ value: (prev[idx].transform.flipX) ? 1 : 0 }));

                prev[idx].frameRect.frameX = Math.floor(frameXExpr.evaluate({ value: prev[idx].frameRect.frameX }));
                prev[idx].frameRect.frameY = Math.floor(frameYExpr.evaluate({ value: prev[idx].frameRect.frameY }));
                prev[idx].frameRect.frameWidth = Math.floor(frameWidthExpr.evaluate({ value: prev[idx].frameRect.frameWidth }));
                prev[idx].frameRect.frameHeight = Math.floor(frameHeightExpr.evaluate({ value: prev[idx].frameRect.frameHeight }));
                prev[idx].transform.newWidth = Math.floor(imgWidthExpr.evaluate({ value: prev[idx].transform.newWidth }));
                prev[idx].transform.newHeight = Math.floor(imgHeightExpr.evaluate({ value: prev[idx].transform.newHeight }));
                prev[idx].transform.flipX = !!(flipXExpr.evaluate({ value: (prev[idx].transform.flipX) ? 1 : 0 }));
                prev[idx].transform.flipY = !!(flipYExpr.evaluate({ value: (prev[idx].transform.flipY) ? 1 : 0 }));
                prev[idx]._changed = true;
            }
            return prev;
        });
        _handleRowClick(currSelectedRow);
        drawFrameWithBox($spriteframes[currSelectedRow]).then(() => {});
        selectedRows = [];
    }
</script>


<Modal bind:showModal={showView} closeButtonMsg={"Save and Close"}>
    <div slot="header">
        <h4>XML View</h4>
    </div>
    <div id="view-container">
        <div id="table-div">
            <table class="not-selectable" on:keydown={onKeyPressTable}>
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
                        <tr tabindex="{i}" bind:this={rows[i]} on:click|stopPropagation={async (_)=>{ _handleRowClick(i); await drawFrameWithBox(spr); }}>
                            <td>
                                <input type="checkbox" name="select-{spr.sprId}" id="select-{spr.sprId}" on:change={async (e)=>{ _handleRowClick(i); await drawFrameWithBox(spr); onRowChecked(e, i); }}>
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
    <button on:click={() => showGroupChangeModal = true}>Modify selected</button>
</Modal>

<Modal bind:showModal={showGroupChangeModal}>
    <div>
        <h3>Modify Selection</h3>
        <div class="controls-horizontal">
            <label class="xmlview-input" for="mod-frame-x">
                Frame X
                <input bind:this={_modifyframeXInput} type="text" name="mod-frame-x" id="mod-frame-x">
            </label>
            <label class="xmlview-input" for="mod-frame-y">
                Frame Y
                <input bind:this={_modifyframeYInput} type="text" name="mod-frame-y" id="mod-frame-y">
            </label>
            <label class="xmlview-input" for="mod-frame-width">
                Frame Width
                <input bind:this={_modifyframeWidthInput} type="text" name="mod-frame-width" id="mod-frame-width">
            </label>
            <label class="xmlview-input" for="mod-frame-height">
                Frame Height
                <input bind:this={_modifyframeHeightInput} type="text" name="mod-frame-height" id="mod-frame-height">
            </label>
            <label class="xmlview-input" for="mod-scale-x">
                Image Width
                <input bind:this={_modifynewWidthInput} type="text" name="mod-scale-x" id="mod-scale-x" min="1">
            </label>
            <label class="xmlview-input" for="mod-scale-y">
                Image Height
                <input bind:this={_modifynewHeightInput} type="text" name="mod-scale-y" id="mod-scale-y" min="1">
            </label>
            <label class="xmlview-input" for="mod-flip-x">
                Flip X
                <input bind:this={_modifyflipXInput} type="text" name="mod-flip-x" id="mod-flip-x">
            </label>
            <label class="xmlview-input" for="mod-flip-y">
                Flip Y
                <input bind:this={_modifyflipYInput} type="text" name="mod-flip-y" id="mod-flip-y">
            </label>
        </div>
        <button on:click={applyModification}>Modify</button>
        <p>Note: You can enter a number or an expression like <strong><code>value + 3</code></strong></p>
        <p>Expressions will be applied on the current value of each cell (by substituting '<code>value</code>')</p>
        <p>You can flip boolean values by using the expression: '<code><i>not</i> value</code>' (and the only values allowed are <code>true</code> and <code>false</code> )</p>
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
        max-width: 95%;
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
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

    const frameProperties = [
        'frameX',
        'frameY',
        'frameWidth',
        'frameHeight',
    ];
    const transformProperties = [
        'newWidth',
        'newHeight',
    ];
    const boolProperties = [
        'flipX',
        'flipY',
    ];

    type Controls = {
        frameX: HTMLInputElement
        frameY: HTMLInputElement
        frameWidth: HTMLInputElement
        frameHeight: HTMLInputElement
        newWidth: HTMLInputElement
        newHeight: HTMLInputElement
        flipX: HTMLInputElement
        flipY: HTMLInputElement
    };

    const frameControls: Controls = {
        frameX: null,
        frameY: null,
        frameWidth: null,
        frameHeight: null,
        newWidth: null,
        newHeight: null,
        flipX: null,
        flipY: null
    };
    let _animScaleInput:HTMLInputElement = null;

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
                r.classList.remove('cell-selected');
            }
        }
        rows[index].classList.add('cell-selected');

        for(const prop of frameProperties)
        {
            frameControls[prop].value = '' + curSprFrame.frameRect[prop];
        }
        for(const prop of transformProperties)
        {
            frameControls[prop].value = '' + curSprFrame.transform[prop];
        }
        for(const prop of boolProperties)
        {
            frameControls[prop].checked = curSprFrame.transform[prop];
        }
    }

    function onValueChange(e: Event)
    {
        for(const prop of frameProperties)
        {
            curSprFrame.frameRect[prop] = +frameControls[prop].value;
        }
        for(const prop of transformProperties)
        {
            curSprFrame.transform[prop] = +frameControls[prop].value;
        }
        for(const prop of boolProperties)
        {
            curSprFrame.transform[prop] = frameControls[prop].checked;
        }

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

    let modificationControls: Controls = {
        frameX: null,
        frameY: null,
        frameWidth: null,
        frameHeight: null,
        newWidth: null,
        newHeight: null,
        flipX: null,
        flipY: null,
    };
    function applyModification()
    {
        const rawValues = [];
        for(const prop of frameProperties)
        {
            rawValues.push([ prop, modificationControls[prop].value ]);
        }
        for(const prop of transformProperties)
        {
            rawValues.push([ prop, modificationControls[prop].value ]);
        }
        for(const prop of boolProperties)
        {
            rawValues.push([ prop, modificationControls[prop].value ]);
        }

        const mathParser = new Parser({ operators: { logical: true } });

        const exprs = new Map(rawValues.map(([prp, exprStr]) => {
            if(exprStr.length > 0)
            {
                return [prp, mathParser.parse(exprStr)];
            }
            else
            {
                return [ prp, null ];
            }
        }));
        // console.log(exprs);
        
        spriteframes.update((prev) => {
            for(const idx of selectedRows)
            {
                for(const prop of frameProperties)
                {
                    const expr = exprs.get(prop);
                    if(expr)
                    {
                        prev[idx].frameRect[prop] = Math.floor(expr.evaluate({ value: prev[idx].frameRect[prop] }));
                    }
                }
                for(const prop of transformProperties)
                {
                    const expr = exprs.get(prop);
                    if(expr)
                    {
                        prev[idx].transform[prop] = Math.floor(exprs.get(prop).evaluate({ value: prev[idx].transform[prop] }));
                    }
                }
                for(const prop of boolProperties)
                {
                    const expr = exprs.get(prop);
                    if(expr)
                    {
                        prev[idx].transform[prop] = !!(exprs.get(prop).evaluate({ value: prev[idx].transform[prop] }));
                    }
                }

                prev[idx]._changed = true;
            }
            return prev;
        });
        _handleRowClick(currSelectedRow);
        drawFrameWithBox($spriteframes[currSelectedRow]).then(() => {});

        showGroupChangeModal = false; // close modal

        const getCheckBox = (childElems: HTMLCollection) => { // helper to get checkbox element of a row
            for (const kid of childElems) {
                if(kid.children.length === 1)
                {
                    return kid.children[0];
                }
            }
            return null;
        };
        
        for (const rowIndex of selectedRows) {
            const _row = rows[rowIndex];
            const chkbx = getCheckBox(_row.children);
            if(chkbx)
            {
                const checkboxElem: HTMLInputElement = chkbx as HTMLInputElement;
                checkboxElem.checked = false;
            }
        }
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
            <div id="mini-table-view">
                <div class="mini-controls-horizontal">
                    <button disabled={$spriteframes.length < 1} on:click={async () => { _handleRowClick(((currSelectedRow - 1) < 0 ? $spriteframes.length - 1 : currSelectedRow - 1)); await drawFrameWithBox($spriteframes[currSelectedRow]); }}>&lt; Previous</button>
                    <table class="not-selectable">
                        <thead>
                            <th></th>
                            <th>Prefix</th>
                            <th>Width</th>
                            <th>Height</th>
                        </thead>
                        <tbody>
                            <tr>
                                <td><input disabled={$spriteframes.length < 1 || currSelectedRow === null} type="checkbox" name="test" id="test" checked={selectedRows.indexOf(currSelectedRow) >= 0} on:change={async (e)=>{ _handleRowClick(currSelectedRow); await drawFrameWithBox($spriteframes[currSelectedRow]); onRowChecked(e, currSelectedRow); }}></td>
                                <td>{curSprFrame && curSprFrame.animationPrefix}</td>
                                <td>{curSprFrame && curSprFrame.rect.width}</td>
                                <td>{curSprFrame && curSprFrame.rect.height}</td>
                            </tr>
                        </tbody>
                    </table>
                    <button disabled={$spriteframes.length < 1} on:click={async () => { _handleRowClick((currSelectedRow + 1) % $spriteframes.length); await drawFrameWithBox($spriteframes[currSelectedRow]); }}>Next &gt;</button>
                </div>
                <p>You have selected {selectedRows.length} frames</p>
                <button disabled={selectedRows.length < 1} on:click={() => showGroupChangeModal = true}>Modify selection</button>
            </div>
            <div class="controls-horizontal">
                <label class="xmlview-input" for="frame-x">
                    Frame X
                    <input type="number" name="frame-x" id="frame-x" on:input={onValueChange} bind:this={frameControls.frameX}>
                </label>
                <label class="xmlview-input" for="frame-y">
                    Frame Y
                    <input type="number" name="frame-y" id="frame-y" on:input={onValueChange} bind:this={frameControls.frameY}>
                </label>
                <label class="xmlview-input" for="frame-width">
                    Frame Width
                    <input type="number" name="frame-width" id="frame-width" on:input={onValueChange} bind:this={frameControls.frameWidth}>
                </label>
                <label class="xmlview-input" for="frame-height">
                    Frame Height
                    <input type="number" name="frame-height" id="frame-height" on:input={onValueChange} bind:this={frameControls.frameHeight}>
                </label>
                <label class="xmlview-input" for="scale-x">
                    Image Width
                    <input type="number" name="scale-x" id="scale-x" min="1" on:input={onValueChange} bind:this={frameControls.newWidth}>
                </label>
                <label class="xmlview-input" for="scale-y">
                    Image Height
                    <input type="number" name="scale-y" id="scale-y" min="1" on:input={onValueChange} bind:this={frameControls.newHeight}>
                </label>
                <label class="xmlview-input" for="flip-x">
                    Flip X
                    <input type="checkbox" name="flip-x" id="flip-x" on:input={onValueChange} bind:this={frameControls.flipX}>
                </label>
                <label class="xmlview-input" for="flip-y">
                    Flip Y
                    <input type="checkbox" name="flip-y" id="flip-y" on:input={onValueChange} bind:this={frameControls.flipY}>
                </label>

                <div class="xml-input-cell-container">
                    <label class="xmlview-input" for="animation-scale">
                        Animation Scale
                        <input bind:this={_animScaleInput} type="number" name="animation-scale" id="animation-scale" min="0.01" step="0.01" on:input={_onAnimationScaleChange}>
                    </label>
                </div>
            </div>
            <div class="canvas-div">
                <canvas bind:this={canvasElement} width="600px" height="500px"></canvas>
            </div>
        </div>
    </div>
    <button class="modify-btn-large-screen" on:click={() => showGroupChangeModal = true}>Modify selected</button>
</Modal>

<Modal bind:showModal={showGroupChangeModal}>
    <div>
        <h3>Modify Selection</h3>
        <div class="controls-horizontal">
            <label class="xmlview-input" for="mod-frame-x">
                Frame X
                <input bind:this={modificationControls.frameX} type="text" name="mod-frame-x" id="mod-frame-x">
            </label>
            <label class="xmlview-input" for="mod-frame-y">
                Frame Y
                <input bind:this={modificationControls.frameY} type="text" name="mod-frame-y" id="mod-frame-y">
            </label>
            <label class="xmlview-input" for="mod-frame-width">
                Frame Width
                <input bind:this={modificationControls.frameWidth} type="text" name="mod-frame-width" id="mod-frame-width">
            </label>
            <label class="xmlview-input" for="mod-frame-height">
                Frame Height
                <input bind:this={modificationControls.frameHeight} type="text" name="mod-frame-height" id="mod-frame-height">
            </label>
            <label class="xmlview-input" for="mod-scale-x">
                Image Width
                <input bind:this={modificationControls.newWidth} type="text" name="mod-scale-x" id="mod-scale-x" min="1">
            </label>
            <label class="xmlview-input" for="mod-scale-y">
                Image Height
                <input bind:this={modificationControls.newHeight} type="text" name="mod-scale-y" id="mod-scale-y" min="1">
            </label>
            <label class="xmlview-input" for="mod-flip-x">
                Flip X
                <input bind:this={modificationControls.flipX} type="text" name="mod-flip-x" id="mod-flip-x">
            </label>
            <label class="xmlview-input" for="mod-flip-y">
                Flip Y
                <input bind:this={modificationControls.flipY} type="text" name="mod-flip-y" id="mod-flip-y">
            </label>
        </div>
        <button on:click={applyModification}>Modify</button>
        <p>Note: You can enter a number or an expression like <strong><code>value + 3</code></strong></p>
        <p>Expressions will be applied on the current value of each cell (by substituting '<code>value</code>')</p>
        <p>You can flip boolean values by using the expression: '<code><i>not</i> value</code>' (and the only values allowed are <code>true</code> and <code>false</code> )</p>
    </div>
</Modal>

<style>
    tr:global(.cell-selected) {
        background-color: var(--table-row-selected);
    }

    .controls-horizontal {
        display: grid;
        grid-template-columns: repeat(6, 1fr);
        width: 30rem;
    }

    .mini-controls-horizontal {
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
        position: absolute;
        height: 65vh;
        width: 600px;
        overflow-y: scroll;
    }
    #view-container {
        display: grid;
        grid-template-columns: 500px 100%;
        height: 80vh;
        max-width: 50%;
    }
    #table-div {
        height: inherit;
        overflow-y: scroll;
        position: relative;
    }

    #mini-table-view {
        display: none;
    }

    #mini-table-view > * > table {
        width: 60%;
    }

    @media screen and (max-width: 700px) {
        #table-div {
            display: none;
        }

        #mini-table-view {
            display: block;
        }

        .modify-btn-large-screen {
            display: none;
        }

        .controls-horizontal {
            grid-template-columns: repeat(4, 1fr);
        }

        .xmlview-input {
            font-size: small;
        }

        .canvas-div {
            height: 30vh;
        }
    }

    table {
        border: 2px solid black;
    }
    td, th {
        border: 1px solid black;
    }
    canvas {
        border: 2px solid var(--border-color);
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
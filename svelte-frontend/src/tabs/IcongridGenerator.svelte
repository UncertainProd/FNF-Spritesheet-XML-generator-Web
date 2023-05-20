<script lang="ts">
    import { openFileDialog, saveFile } from '../utils'
    import { base64EncArr } from '../b64utils';
    import { onDestroy } from 'svelte';
    import type { Wasm_T } from '../global';
    export let wasm: Wasm_T;
    export let charname: string; // name of character

    let isLegacy = false;
    let selectedFiles: File[] = [];
    let displayImg: HTMLImageElement = null;

    let _controlsLayout: string;

    // makes `_controlsLayout` update every time `isLegacy` updates
    $: _controlsLayout = '2fr' + (isLegacy ? ' 1fr 1fr': '') + ' 1fr 1fr 1fr 2fr';

    async function makeIcongrid()
    {
        const { IconPacker } = wasm;
        const iconPacker = IconPacker.new();

        // let imagedata = [];
        if(selectedFiles.length > 0)
        {
            for(let f of selectedFiles)
            {
                iconPacker.add_image(await f.arrayBuffer());
            }
    
            let finalImageBytes = iconPacker.make_packed_image();
            displayImg.setAttribute('src', 'data:image/png;base64,' + base64EncArr(finalImageBytes))
    
            let iconFilename = `icon-${charname !== undefined ? charname : 'character'}.png`;
            saveFile(finalImageBytes, iconFilename);
        }
    }


    function clearFiles(_:any)
    {
        selectedFiles = [];
    }

    onDestroy(()=>{
        document.removeEventListener('clearIcons', clearFiles);
    });

    document.addEventListener('clearIcons', clearFiles);


    function onFileSelection(e:Event)
    {
        selectedFiles = [ ...(e.target as HTMLInputElement).files ];
    }
</script>

<div id="preview-div">
    <img bind:this={displayImg} src="" alt={"The " + ((isLegacy) ? "" : "final") + " icongrid will be displayed here."}>
</div>

<div id="controls" style="grid-template-columns: {_controlsLayout};">
    <label for="legacy-mode" id="legacy-mode-label">
        <input type="checkbox" bind:checked={isLegacy} name="legacy-mode" id="legacy-mode" />
        Legacy Mode
    </label>
    <p>Number of icons selected: {selectedFiles.length}</p>
    <button on:click={clearFiles}>Clear Icons</button>
    {#if isLegacy}
        <button id="add-icongrid">Add Icongrid</button>
        <button id="clear-icongrid">Clear Icongrid</button>
    {/if}
    <button id="add-icons" on:click={(_)=>openFileDialog(onFileSelection, 'image/png')}>Add Icons</button>
    <button id="generate-grid" on:click={makeIcongrid}>Generate Icongrid</button>
</div>

<style>
    #preview-div {
        border: 2px solid grey;
        margin: 2px;
        height: 60vh;
        overflow: scroll;
    }

    #controls {
        display: grid;
        border: 2px solid grey;
        margin: 2px;
        position: sticky;
        z-index: 5;
        bottom: 0;
        background-color: white;
    }

    #controls > * {
        display: flex;
        align-items: center;
    }

    #legacy-mode {
        transform: translateY(35%) translateX(-3px);
        z-index: -1;
    }

    #legacy-mode-label {
        padding: 0 5px;
    }
</style>
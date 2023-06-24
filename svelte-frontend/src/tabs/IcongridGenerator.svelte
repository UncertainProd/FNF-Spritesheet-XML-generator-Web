<script lang="ts">
    import { openFileDialog, saveFile } from '../utils'
    import { arrayBufferToBase64, base64EncArr } from '../b64utils';
    import { onDestroy } from 'svelte';
    import type { Wasm_T } from '../global';
    export let wasm: Wasm_T;
    export let charname: string; // name of character

    let isLegacy = false;
    let selectedFiles: File[] = [];
    let displayImg: HTMLImageElement = null;
    let loadingDlg: HTMLDialogElement = null;

    let controlsDiv: HTMLDivElement = null;

    // for legacy version
    let curIcongrid: File = null;

    $: {
        if(controlsDiv)
        {
            if(isLegacy)
            {
                controlsDiv.classList.add('layout-legacy');
                controlsDiv.classList.remove('layout-normal');
            }
            else
            {
                controlsDiv.classList.remove('layout-legacy');
                controlsDiv.classList.add('layout-normal');
            }
        }
    }

    async function makeIcongrid()
    {
        if(!isLegacy)
        {
            const { IconPacker } = wasm;
            const iconPacker = IconPacker.new();
    
            if(selectedFiles.length > 0)
            {
                loadingDlg.showModal();
                for(let f of selectedFiles)
                {
                    iconPacker.add_image(await f.arrayBuffer());
                }
        
                let finalImageBytes = iconPacker.make_packed_image();
                displayImg.setAttribute('src', 'data:image/png;base64,' + base64EncArr(finalImageBytes))
                loadingDlg.close();
        
                let iconFilename = `icon-${charname !== undefined ? charname : 'character'}.png`;
                saveFile(finalImageBytes, iconFilename);
            }
            else
            {
                alert("No icons selected! Please select some icons");
            }
        }
        else
        {
            if(selectedFiles.length > 0)
            {
                if(curIcongrid !== null)
                {
                    loadingDlg.showModal();
                    const icongridBytes = new Uint8Array(await curIcongrid.arrayBuffer());
                    const imgBytesArray = [];
                    for(let f of selectedFiles)
                    {
                        imgBytesArray.push(arrayBufferToBase64(await f.arrayBuffer()));
                    }
                    const finalBytes = wasm.make_icongrid_legacy(icongridBytes, imgBytesArray);
                    displayImg.setAttribute('src', 'data:image/png;base64,' + base64EncArr(finalBytes));
                    loadingDlg.close();
                    saveFile(finalBytes, 'iconGrid.png');
                }
                else
                {
                    alert("No Icongrid selected! Please select an icongrid");
                }
            }
            else
            {
                alert("No icons selected! Please select some icons");
            }
        }
    }

    async function onIcongridAdd(e: Event)
    {
        curIcongrid = (e.target as HTMLInputElement).files[0];
        displayImg.setAttribute('src', 'data:image/png;base64,' + arrayBufferToBase64(await curIcongrid.arrayBuffer()));
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

<dialog bind:this={loadingDlg}>
    <p>Creating Icongrid....</p>
</dialog>

<div id="preview-div">
    <img bind:this={displayImg} src="" alt={"The " + ((isLegacy) ? "" : "final") + " icongrid will be displayed here."}>
</div>

<div bind:this={controlsDiv} id="controls" class="layout-normal layout-legacy">
    <label for="legacy-mode" id="legacy-mode-label">
        <input type="checkbox" bind:checked={isLegacy} name="legacy-mode" id="legacy-mode" />
        Legacy Mode
    </label>
    <p>Number of icons selected: {selectedFiles.length}</p>
    <button on:click={clearFiles}>Clear Icons</button>
    {#if isLegacy}
        <button id="add-icongrid" on:click={(_) => openFileDialog(onIcongridAdd, 'image/png', false) }>Add Icongrid</button>
        <button id="clear-icongrid" on:click={()=>{ curIcongrid = null; }}>Clear Icongrid</button>
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
    }

    .layout-normal {
        grid-template-columns: repeat(5, 1fr);
    }

    .layout-legacy {
        grid-template-columns: repeat(7, 1fr);
    }

    @media screen and (max-width: 530px)
    {
        .layout-legacy {
            grid-template-columns: repeat(4, 1fr);
        }
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
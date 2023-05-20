<script lang="ts">
    import { UIdGen, stringFind, getImageDimensions, uidgen } from '../utils'
    import { arrayBufferToBase64 } from '../b64utils'
    import SpriteFrame from '../components/SpriteFrame.svelte';
    import SpritesheetGenControls from '../components/SpritesheetGenControls.svelte';
    import RubberBandSelector from '../components/RubberBandSelector.svelte';
    import { SpriteFrameData } from '../spriteframedata';
    import type { Wasm_T } from '../global';

    export let wasm: Wasm_T;

    wasm.greet('hi');

    // const uidgen = new UIdGen();

    let spriteFrameOrderData:SpriteFrameData[] = [];

    function removeSpriteFrame(sprframe: SpriteFrameData)
    {
        const ind = spriteFrameOrderData.indexOf(sprframe);
        if(ind !== -1)
        {
            spriteFrameOrderData.splice(ind, 1);
            spriteFrameOrderData = spriteFrameOrderData; // updates the state in svelte
        }
        else
        {
            console.log("[ERROR] Could not find element: " + sprframe);
        }
    }

    async function onPNGAdd(e: Event)
    {
        const target = e.target as HTMLInputElement;
        for(let f of target.files)
        {
            const [ imgWidth, imgHeight ] = await getImageDimensions(f);
            const newFrame = new SpriteFrameData(f.name + '::' + uidgen.getNewId(), 'single_frame', f, null, null);
            newFrame.rect.width = imgWidth;
            newFrame.rect.height = imgHeight;
            newFrame.frameRect.frameWidth = imgWidth;
            newFrame.frameRect.frameHeight = imgHeight;
            newFrame.animationPrefix = f.name.substring(0, f.name.indexOf('.png'));

            // spriteFrameOrderData = [ ...spriteFrameOrderData, newFrame ];
            spriteFrameOrderData.push(newFrame);
        }
        spriteFrameOrderData = spriteFrameOrderData;
    }

    async function addSpritesheetAndXML(e: Event, curSpritesheet: File, curXML: File)
    {
        if(curSpritesheet == null)
        {
            // TODO: Replace alerts with a message box or something
            alert('Please select a spritesheet');
            return true;
        }
        if(curXML == null)
        {
            // TODO: Replace alerts with a message box or something
            alert('Please select an XML');
            return true;
        }

        let spshData = arrayBufferToBase64(await curSpritesheet.arrayBuffer());

        let xmlstr = await curXML.text();
        let xmlparser = new DOMParser();
        let xmldoc = xmlparser.parseFromString(xmlstr, "text/xml");
        let subtextures = xmldoc.getElementsByTagName("SubTexture");
        for(let tex of subtextures)
        {
            const texname = tex.getAttribute('name');
            const animPrefixIndex = stringFind(texname, (ch) => { return !(ch >= '0' && ch <= '9') }, true);

            const newFrame = new SpriteFrameData(curSpritesheet.name + '::' + uidgen.getNewId(), 'spritesheet_frame', curSpritesheet, curXML, spshData);
            newFrame.animationPrefix = texname.substring(0, animPrefixIndex + 1);
            newFrame.rect = {
                x: parseInt(tex.getAttribute('x')),
                y: parseInt(tex.getAttribute('y')),
                width: parseInt(tex.getAttribute('width')),
                height: parseInt(tex.getAttribute('height')),
            };
            newFrame.frameRect = {
                frameX: parseInt(tex.getAttribute('frameX')) || 0,
                frameY: parseInt(tex.getAttribute('frameY')) || 0,
                frameWidth: parseInt(tex.getAttribute('frameWidth')) || newFrame.rect.width,
                frameHeight: parseInt(tex.getAttribute('frameHeight')) || newFrame.rect.height
            };

            // spriteFrameOrderData = [ ...spriteFrameOrderData, newFrame ];
            spriteFrameOrderData.push(newFrame);
        }

        spriteFrameOrderData = spriteFrameOrderData;

        curSpritesheet = null;
        curXML = null;
        return false;
    }

    //     saveFile(
    //         base64DecToArr(
    //             cnvs
    //             .toDataURL('image/png')
    //             .substring('data:image/png;base64,'.length)
    //         )
    //         , 'testing.png'
    //     );

    function setAnimationPrefix(prefix: string)
    {
        // for each selected, change it's animation prefix
        for(let i = 0; i < spriteFrameOrderData.length; i++)
        {
            let spr = spriteFrameOrderData[i];
            if(spr.selected)
            {
                // spriteFrameOrderData[i] = { ...spriteFrameOrderData[i], animationPrefix: prefix, selected: false };
                spriteFrameOrderData[i].animationPrefix = prefix;
                spriteFrameOrderData[i].selected = false;
                spriteFrameOrderData[i] = spriteFrameOrderData[i]; // update svelte state
            }
        }
    }
</script>


<!-- <div id="test" bind:this={rubberBandDiv}></div> -->

<div id="sprite-frames">
    <!-- <RubberBandSelector /> -->
    {#each spriteFrameOrderData as dat (dat.sprId)}
        <SpriteFrame bind:frameInfo={dat} onDelete={removeSpriteFrame} />
    {/each}
</div>


<SpritesheetGenControls bind:spriteframes={spriteFrameOrderData} onPNGAdd={onPNGAdd} setAnimationPrefix={setAnimationPrefix} addSpritesheetAndXML={addSpritesheetAndXML} />


<style>
    #sprite-frames {
        border: 2px solid grey;
        margin: 2px;
        height: 60vh;
        overflow-y: scroll;
    }

    /* #test {
        position: absolute;
        border: 5px solid black;
        width: 100px;
        height: 100px;
    } */
</style>
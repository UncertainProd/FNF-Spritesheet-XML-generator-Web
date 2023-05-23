import { arrayBufferToBase64, base64EncArr } from './b64utils'
import type { SpriteFrameData } from './spriteframedata';
import { get as getFromStore } from 'svelte/store';
import { spritesheet_map } from './stores';

/**
 * 
 * @param {(e: Event)=>void} onSelectFile 
 * @param {string} fileFilter 
 * @param {boolean} multpl 
 */
export function openFileDialog(onSelectFile: (e: Event)=>void, fileFilter: string, multpl:boolean = true):void
{
    let felem = document.createElement('input');
    felem.setAttribute('type', 'file');
    if(multpl)
    {
        felem.setAttribute('multiple', 'true');
    }
    felem.setAttribute('style', 'display: none');
    felem.setAttribute('accept', fileFilter);

    document.body.append(felem);

    felem.onchange = (e)=>{
        onSelectFile(e);
        document.body.removeChild(felem);
    };

    felem.click();

}

/**
 * 
 * @param {Uint8Array} content 
 * @param {string} name 
 */
export function saveFile(content: Uint8Array, name: string):void
{
    let contentBlob = new Blob([content]);
    let objUrl = URL.createObjectURL(contentBlob);

    let aElem = document.createElement('a');
    aElem.setAttribute('href', objUrl);
    aElem.setAttribute('download', name);
    aElem.click();
}

/**
 * Finds the first index where the given predicate function returns true
 * 
 * @param {string} str 
 * @param {(s: string)=>boolean} predicate 
 * @param {boolean} reverse
 * @returns {number|null} The first index where the predicate returns true. `null` if no such index exists
 */
export function stringFind(str: string, predicate: (s:string)=>boolean, reverse: boolean = false):number|null
{
    if(!reverse)
    {
        for(let i = 0; i < str.length; i++)
        {
            if(predicate(str[i]))
            {
                return i;
            }
        }
    }
    else
    {
        for(let i = str.length - 1; i >= 0; i--)
        {
            if(predicate(str[i]))
            {
                return i;
            }
        }
    }
    return null;
}

/**
 * 
 * @param {SpriteFrameData} frameInfo 
 * @returns {Promise<HTMLImageElement>}
 */
export async function makeImage(frameInfo: SpriteFrameData): Promise<HTMLImageElement>
{
    let imgdata: string;
    if(frameInfo.type == 'single_frame')
    {
        let img = frameInfo.imgfileref;
        imgdata = 'data:image/png;base64,' + base64EncArr(new Uint8Array(await img.arrayBuffer()));
    }
    else
    {
        const spshB64 = getFromStore(spritesheet_map).get(frameInfo.spritesheetId);
        if(spshB64 !== undefined)
        {
            imgdata = 'data:image/png;base64,' + spshB64[0];
        }
        else
        {
            imgdata = 'data:image/png;base64,';
        }
    }
    const imgelem = new Image();
    imgelem.src = imgdata;
    await imgelem.decode();
    return imgelem;
}

/**
 * 
 * @param {File} imgfileref 
 * @returns {Promise<number[]>}
 */
export async function getImageDimensions(imgfileref: File): Promise<number[]>
{
    const imgdata = 'data:image/png;base64,' + arrayBufferToBase64(await imgfileref.arrayBuffer());
    const imgelem = new Image();
    imgelem.src = imgdata;
    await imgelem.decode();
    return [ imgelem.naturalWidth, imgelem.naturalHeight ];
}

// Don't use this function for cryptography! This is only to generate image hashes for the hashmaps
export async function hashImage(imgdata: Uint8Array): Promise<string>
{
    const digest = await crypto.subtle.digest('SHA-1', imgdata)
    return base64EncArr(new Uint8Array(digest));
}

export class UIdGen{
    _val: number

    constructor()
    {
        this._val = -1;
    }

    getNewId()
    {
        this._val++;
        return this._val;
    }
}

export const uidgen = new UIdGen();
type SpriteframeType = 'single_frame' | 'spritesheet_frame'

export class SpriteFrameData
{
    sprId: string;
    type: SpriteframeType;
    imgfileref: File;
    xmlfileref: File | null;
    selected: boolean; // refactor?
    spritesheetDataB64: string | null;
    animationPrefix: string;
    rect: {
        x: number | null,
        y: number | null,
        width: number,
        height: number
    };
    transform: {
        scaleX: number,
        scaleY: number,
        flipX: boolean,
        flipY: boolean
    };
    frameRect: {
        frameX: number,
        frameY: number,
        frameWidth: number,
        frameHeight: number
    };
    _changed: boolean;

    constructor(id:string, type: SpriteframeType, imgfileref: File, xmlfileref: File|null, spritesheetDataB64: string|null)
    {
        this.sprId = id;
        this.type = type;
        this.imgfileref = imgfileref;
        this.xmlfileref = xmlfileref;
        this.selected = false;
        this.spritesheetDataB64 = spritesheetDataB64;
        this.animationPrefix = '';
        this.rect = {
            x: 0,
            y: 0,
            width: null,
            height: null
        };
        this.transform = {
            scaleX: 1.0,
            scaleY: 1.0,
            flipX: false,
            flipY: false
        };
        this.frameRect = {
            frameX: 0,
            frameY: 0,
            frameWidth: null,
            frameHeight: null
        };
        this._changed = false;
    }

    clone()
    {
        const clonedFrame = new SpriteFrameData(this.sprId, this.type, this.imgfileref, this.xmlfileref, this.spritesheetDataB64);
        clonedFrame.selected = this.selected;
        clonedFrame.animationPrefix = this.animationPrefix;
        clonedFrame.rect = this.rect;
        clonedFrame.transform = this.transform;
        clonedFrame.frameRect = this.frameRect;
        return clonedFrame;
    }
}
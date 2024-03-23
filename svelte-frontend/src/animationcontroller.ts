import type { SpriteFrameData } from "./spriteframedata";
import { makeImage } from "./utils";

export class AnimationController {
    context: CanvasRenderingContext2D;
    curFrames: SpriteFrameData[];
    curImgs: HTMLImageElement[];
    curFrameIndex: number;
    start: number; prev: number;
    ticks: number;
    animId: number;
    fps: number;
    animationScale: number;
    isPlaying: boolean;

    constructor(ctx2d: CanvasRenderingContext2D)
    {
        this.context = ctx2d;

        this.curFrames = [];
        this.curImgs = [];
        this.curFrameIndex = 0;
        this.start = null;
        this.prev = null;
        this.ticks = 0;
        this.animId = null;
        this.fps = 24;
        this.animationScale = 1;

        this.isPlaying = false;
    }

    resetAnimation()
    {
        this.curFrames = [];
        this.curImgs = [];
        this.curFrameIndex = 0;
        this.start = null;
        this.prev = null;
        this.ticks = 0;
    }

    tick(timestamp: number)
    {
        this.start = timestamp;
        if(this.prev === null)
        {
            this.prev = timestamp;
        }
        const elapsed = this.start - this.prev;
        this.ticks += elapsed;

        if(this.ticks > 1000/this.fps)
        {
            const curFrame = this.curFrames[this.curFrameIndex];
            const img = this.curImgs[this.curFrameIndex];
            this.curFrameIndex = (this.curFrameIndex + 1) % this.curFrames.length;

            this.clearCanvas();
            
            this.drawImage(img, curFrame);
            this.ticks = 0;
        }
        this.animId = requestAnimationFrame((ts)=>{ this.tick(ts); });
        this.prev = this.start;
    }

    drawImage(frameImg: HTMLImageElement, rectInfo: SpriteFrameData, withBoundingBox = false, withClipping = false)
    {
        const r = rectInfo.rect;
        const frame = rectInfo.frameRect;
        const transform = rectInfo.transform;

        const flipFactorX = (transform.flipX) ? -1 : 1;
        const flipFactorY = (transform.flipY) ? -1 : 1;

        const totalScaleToDrawX = flipFactorX * this.animationScale;
        const totalScaleToDrawY = flipFactorY * this.animationScale;

        this.context.save();
        
        this.context.scale(totalScaleToDrawX, totalScaleToDrawY);
        if(transform.flipX)
        {
            this.context.translate(-transform.newWidth, 0);
        }

        if(transform.flipY)
        {
            this.context.translate(0, -transform.newHeight);
        }

        this.context.drawImage(
            frameImg,
            r.x, r.y, r.width, r.height, 
            -frame.frameX * flipFactorX,
            -frame.frameY * flipFactorY,
            transform.newWidth,
            transform.newHeight
        );

        this.context.restore();

        if(withClipping)
        {
            // scuffed clipping logic
            this.context.clearRect(
                frame.frameWidth * this.animationScale, 
                0, 
                this.context.canvas.width, 
                this.context.canvas.height
            );
            this.context.clearRect(
                0, 
                frame.frameHeight * this.animationScale, 
                this.context.canvas.width, 
                this.context.canvas.height
            );
        }

        if(withBoundingBox)
        {
            this.context.beginPath();
            this.context.rect(0, 0, frame.frameWidth * this.animationScale, frame.frameHeight * this.animationScale);
            this.context.stroke();
        }

    }

    async _prepFrameImages()
    {
        // avoid repeatedly making html Image elements from big spritesheet images. cache them here instead
        const imgCache: Map<string, HTMLImageElement> = new Map();
        for (const frame of this.curFrames)
        {
            let curimg = null;
            if(frame.type === 'spritesheet_frame')
            {
                if(!imgCache.has(frame.spritesheetId))
                {
                    const himg = await makeImage(frame);
                    imgCache.set(frame.spritesheetId, himg);
                    curimg = himg;
                }
                else
                {
                    curimg = imgCache.get(frame.spritesheetId);
                }
            }
            else
            {
                curimg = await makeImage(frame);
            }

            this.curImgs.push(curimg);
        }
    }

    async initFrames(frames: SpriteFrameData[])
    {
        // console.log("Playing animation....")
        this.curFrames = frames;
        this.curFrameIndex = 0;
        this.curImgs = [];
        await this._prepFrameImages();
    }

    play()
    {
        this.animId = requestAnimationFrame((ts)=>{ this.tick(ts); });
        this.isPlaying = true;
    }

    stopAnimation()
    {
        if(this.animId !== null)
        {
            cancelAnimationFrame(this.animId);
        }
        this.resetAnimation();
        this.animId = null;
        this.isPlaying = false;
    }

    clearCanvas()
    {
        const widthToClear = this.context.canvas.width;
        const heightToClear = this.context.canvas.height;
        this.context.clearRect(0, 0, widthToClear, heightToClear);
    }
}
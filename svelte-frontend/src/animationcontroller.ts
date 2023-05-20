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

            // console.log("Clearing canvas");
            this.clearCanvas();
            
            // console.log(`Drawing: { x: ${curFrame.rect.x}, y: ${curFrame.rect.y}, width: ${curFrame.rect.width}, height: ${curFrame.rect.height} }`);
            // console.log(`Frame: { x: ${curFrame.frameRect.frameX}, y: ${curFrame.frameRect.frameY}, width: ${curFrame.frameRect.frameWidth}, height: ${curFrame.frameRect.frameHeight} }`);
            // drawFrameImageTo(img, { rect: curFrame.rect, frameRect: curFrame.frameRect }, this.context, this.animationScale);
            this.drawImage(img, curFrame);
            this.ticks = 0;
        }
        this.animId = requestAnimationFrame((ts)=>{ this.tick(ts); });
        this.prev = this.start;
    }

    drawImage(frameImg: HTMLImageElement, rectInfo: SpriteFrameData, withBoundingBox = false, withClipping = true)
    {
        const r = rectInfo.rect;
        const frame = rectInfo.frameRect;
        const transform = rectInfo.transform;

        const flipFactorX = (transform.flipX) ? -1 : 1;
        const flipFactorY = (transform.flipY) ? -1 : 1;

        const totalScaleToDrawX = flipFactorX * transform.scaleX * this.animationScale;
        const totalScaleToDrawY = flipFactorY * transform.scaleY * this.animationScale;

        this.context.save();
        
        this.context.scale(totalScaleToDrawX, totalScaleToDrawY);
        if(transform.flipX)
        {
            this.context.translate(-frame.frameWidth, 0);
        }

        if(transform.flipY)
        {
            this.context.translate(0, -frame.frameHeight);
        }

        this.context.drawImage(
            frameImg,
            r.x, r.y, r.width, r.height, 
            -frame.frameX,
            -frame.frameY,
            r.width,
            r.height
        );

        if(withBoundingBox)
        {
            this.context.beginPath();
            this.context.rect(0, 0, frame.frameWidth, frame.frameHeight);
            this.context.stroke();
        }

        // this.context.drawImage(
        //     frameImg,
        //     r.x, r.y, r.width, r.height, 
        //     -frame.frameX * this.animationScale,
        //     -frame.frameY * this.animationScale,
        //     r.width * this.animationScale,
        //     r.height * this.animationScale
        // );


        if(withClipping)
        {
            // scuffed clipping logic
            this.context.clearRect(frame.frameWidth, 0, this.context.canvas.width, this.context.canvas.height);
            this.context.clearRect(0, frame.frameHeight, frame.frameWidth, this.context.canvas.height);
        }
        // // scuffed clipping logic
        // this.context.clearRect(frame.frameWidth * this.animationScale, 0, this.context.canvas.width - frame.frameWidth * this.animationScale, this.context.canvas.height);
        // this.context.clearRect(0, frame.frameHeight * this.animationScale, frame.frameWidth * this.animationScale, this.context.canvas.height - frame.frameHeight * this.animationScale);
        this.context.restore();
    }

    async initFrames(frames: SpriteFrameData[])
    {
        // console.log("Playing animation....")
        this.curFrames = frames;
        this.curFrameIndex = 0;
        this.curImgs = await Promise.all(this.curFrames.map(async (elem) => { let img = await makeImage(elem); return img; }));
        // canvasContext.canvas.width = window.innerWidth * 0.9;
        // canvasContext.canvas.height = window.innerHeight * 0.9;
        // this.animId = requestAnimationFrame(this.tick);
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
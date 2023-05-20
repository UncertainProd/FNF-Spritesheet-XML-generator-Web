<script>
    import { onMount, onDestroy } from 'svelte';
    
    let rubberBandDiv = null;
    let isMouseDown = false;
    let mouseDownX = null;
    let mouseDownY = null;
    const handleMouseDown = (evt) => {
        isMouseDown = true;
        mouseDownX = evt.clientX;
        mouseDownY = evt.clientY;
        // console.log("Mouse down!" + `At x = ${mouseDownX} | y = ${mouseDownY}`);
    };
    const handleMouseUp = (evt) => {
        rubberBandDiv.style.display = 'none';
        // console.log("Mouse up!");
        isMouseDown = false;
        mouseDownX = null;
        mouseDownY = null;
    };
    const __makeRectangle = (topleft, bottomright) => {
        const [ left, top ] = topleft;
        const [ right, bottom ] = bottomright;
        rubberBandDiv.style.top = top + 'px';
        rubberBandDiv.style.left = left + 'px';
        rubberBandDiv.style.width = Math.abs(right - left) + 'px';
        rubberBandDiv.style.height = Math.abs(bottom - top) + 'px';
    };
    const handleMouseMove = (evt) => {
        if(isMouseDown)
        {
            rubberBandDiv.style.display = 'block';
            const { top, left } = rubberBandDiv.parentNode.getBoundingClientRect();
            const topCoord = Math.min(evt.clientY, mouseDownY);
            const leftCoord = Math.min(evt.clientX, mouseDownX);
            const bottomCoord = Math.max(evt.clientY, mouseDownY);
            const rightCoord = Math.max(evt.clientX, mouseDownX);

            __makeRectangle([ leftCoord, topCoord ], [ rightCoord - left, bottomCoord - top ]);
            // console.log(`__makeRectangle([ ${leftCoord}, ${topCoord} ], [ ${rightCoord - left}, ${bottomCoord - top} ])`);
        }
    };
    onMount(()=>{
        document.addEventListener('mousedown', handleMouseDown);
        document.addEventListener('mouseup', handleMouseUp);
        document.addEventListener('mousemove', handleMouseMove);
    });

    onDestroy(()=>{
        document.removeEventListener('mousedown', handleMouseDown);
        document.removeEventListener('mouseup', handleMouseUp);
        document.removeEventListener('mousemove', handleMouseMove);
    });
</script>


<div id="test" bind:this={rubberBandDiv}></div>

<style>
    #test {
        position: absolute;
        border: 5px solid black;
        width: 100px;
        height: 100px;
        display: none;
    }
</style>
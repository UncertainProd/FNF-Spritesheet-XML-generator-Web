<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { quadInOut } from "svelte/easing";

    let menuShown = false;
    export let buttonText: string;

    
    let menuDiv: HTMLDivElement = null;
    let dropupButton: HTMLButtonElement = null;

    let dropupLeft = 0;

    const setNewLeft = () => {
        dropupLeft = (dropupButton) ? dropupButton.getBoundingClientRect().left : 0;
    };
    onMount(() => {
        setNewLeft();
        window.addEventListener('resize', setNewLeft);
        document.addEventListener('click', onClickHandler);
    });
    onDestroy(() => {
        window.removeEventListener('resize', setNewLeft);
        document.removeEventListener('click', onClickHandler);
    });

    $: menuDiv && (menuDiv.style.transform = `translateX(${dropupLeft-15}px)`);

    function onClickHandler(e) {
        if(menuDiv && e.target !== menuDiv)
        {
            menuShown = false;
        }
    }

    function fadeNSlide(node, options)
    {
        const fadeAnim = fade(node, options);
        return {
            duration: options.duration,
            css: fadeAnim.css
        };
    }
</script>

<button bind:this={dropupButton} on:click|self|stopPropagation={() => { menuShown = !menuShown }}>{buttonText}</button>
{#if menuShown}
    <div bind:this={menuDiv} class="drop-up-menu" in:fadeNSlide="{{ duration: 100, easing: quadInOut }}" out:fadeNSlide="{{ duration: 100, easing: quadInOut }}">
        <slot />
    </div>
{/if}

<style>
    .drop-up-menu {
        position: absolute;
        display: flex;
        flex-direction: column;
        background-color: brown;
        padding: 2px;
        bottom: 100%;
    }
</style>
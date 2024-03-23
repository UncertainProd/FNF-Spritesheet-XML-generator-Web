<script lang="ts">
    import Modal from "./Modal.svelte";

    export let showView = false;

    export let settings = {
        padding: 2,
        prefixType: 'no-prefix',
        usePrefixOnXMLFrames: false,
        customPrefix: '',
        uniqueFramesOnly: false,
        clipToBoundingBox: true,
        xmlAnimPrefixTrimChars: -1,
    };
</script>

<Modal bind:showModal={showView}>
    <h2 slot="header">Settings</h2>
    <div id="modal-content">
        <fieldset>
            <legend>
                Image
            </legend>
            <label for="padding">
                Image Padding
                <input type="number" name="padding" id="padding" bind:value={settings.padding} min="0" max="20" />
            </label>
            <label for="clip-to-bb">
                <input type="checkbox" name="clip-to-bb" id="clip-to-bb" bind:checked={settings.clipToBoundingBox} />
                Clip to Bounding Box
            </label>
        </fieldset>
        <br />
        <fieldset>
            <legend>
                Animation Name Prefixing
            </legend>
            <label for="prefix-type-character-name">
                <input type="radio" name="prefix-type" id="prefix-type-character-name" value="character-name" bind:group={settings.prefixType}>
                Use character name as prefix
            </label>
            <label for="prefix-type-custom-prefix">
                <input type="radio" name="prefix-type" id="prefix-type-custom-prefix" value="custom-prefix" bind:group={settings.prefixType}>
                Use custom name as prefix
            </label>
            <label for="custom-prefix">Custom Prefix</label>
            <input type="text" id="custom-prefix" bind:value={settings.customPrefix} disabled={settings.prefixType !== 'custom-prefix'}>
            <label for="prefix-type-no-prefix">
                <input type="radio" name="prefix-type" id="prefix-type-no-prefix" value="no-prefix" bind:group={settings.prefixType}>
                Do not use a prefix
            </label>
    
            <br>
            <label for="use-prefix-on-existing-xmls">
                <input bind:checked={settings.usePrefixOnXMLFrames} type="checkbox" name="use-prefix-on-existing-xmls" id="use-prefix-on-existing-xmls">
                Use Prefix even if frame is imported from existing Spritesheet
            </label>
            <br>
            <label for="n-trim-chars-xml">
                <input type="number" name="n-trim-chars-xml" id="n-trim-chars-xml" min="-1" max="10" bind:value={settings.xmlAnimPrefixTrimChars}>
                Number of digits to trim from the end while importing existing spritesheets
            </label>
        </fieldset>
        <br />
        <fieldset>
            <legend>
                Image Sequence Generation
            </legend>
            <label for="img-seq-gen">
                <input bind:checked={settings.uniqueFramesOnly} type="checkbox" name="img-seq-gen" id="img-seq-gen">
                Only export Unique Frames when exporting as Image Sequence
            </label>
        </fieldset>
    </div>
</Modal>

<style>
    #modal-content {
        width: 60vw;
    }

    h2 {
        text-align: center;
    }
</style>
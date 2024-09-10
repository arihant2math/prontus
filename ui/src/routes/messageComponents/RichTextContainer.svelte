<script>
    import {rich} from "../api.js";
    import RichText from "./RichText.svelte";

    export let message;
    let richContent;
    $: message, init();

    function init() {
        rich(message).then((content) => {
            // console.log(content);
            richContent = content;
        });
    }

    init();
</script>
{#if richContent === undefined}
    <p>Loading...</p>
{:else}
    {#if richContent.data.t === "Document"}
        {#each richContent.children as child}
            <RichText content={child}/>
        {/each}
    {/if}
{/if}

<script>
    /** @type {{url: any, mimetype: any, type: any}} */
    let { url, mimetype, type } = $props();

    // TODO: hardcoded
    let realUrl = $derived(url.replace("https://files.chat.trypronto.com/", "http://localhost:10521/"));

    console.warn("MEDIA", type)
</script>
{#if type === "FILE"}
    <!--TODO: Fix-->
    <div>
        <a href="{realUrl}" target="_blank" class="text-blue-500 underline">Download attached file</a>
    </div>
{:else}
    <img src="{realUrl}" class="max-w-lg" alt="An user posted media item"/>
{/if}
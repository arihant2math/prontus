<script>
    export let url;
    export let mimetype;
    export let type;

    // TODO: hardcoded
    $: realUrl = url.replace("https://files.chat.trypronto.com/", "localhost:10521/");

    console.warn("MEDIA", type)
</script>

<img src="{realUrl}" class="max-w-lg" alt="An user posted media item"/>
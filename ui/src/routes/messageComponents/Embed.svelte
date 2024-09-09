<script>
    export let title = "";
    export let description = "";
    export let url = "";
    export let shortUrl = "";
    export let image = "";

    $: title = title;
    $: description = description;
    $: url = url;
    $: shortUrl = shortUrl;
</script>

<a target="_blank" href="{url}" class="bg-gray-50 dark:bg-slate-700 rounded-xl p-4 mb-2 hover:bg-gray-200 dark:hover:bg-slate-600">
    {#if image}
        <img src="{image}" class="rounded-lg mb-2 object-contain h-[40px]"/>
    {/if}
    <span class="text-sm font-medium text-gray-900 dark:text-white mb-2">{title}</span>
    <span class="text-xs text-gray-500 dark:text-gray-400 font-normal">{shortUrl}</span>
</a>
<script>
    /** @type {{target: any, svg?: import('svelte').Snippet, title?: import('svelte').Snippet, body?: import('svelte').Snippet}} */
    let {
        target,
        svg,
        title,
        body
    } = $props();
</script>

<label for="{target}"
       class="inline-flex items-center justify-between w-full p-5 text-gray-500 bg-white border-2 border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 peer-checked:border-blue-600 hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-slate-800 dark:hover:bg-gray-700">
    <div class="flex flex-col items-center justify-between">
        <div class="flex flex-row space-x-2">
            {@render svg?.()}
            <div class="w-full text-lg font-semibold">{@render title?.()}</div>
        </div>
        <div class="w-full text-sm">{@render body?.()}</div>
    </div>
</label>
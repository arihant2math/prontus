<script>
    /** @type {{target: any, svg?: import('svelte').Snippet, text?: import('svelte').Snippet}} */
    let { target, svg, text } = $props();
</script>

<label for="{target}"
       class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-slate-800 dark:hover:bg-gray-700">
    <div class="w-full text-lg font-semibold flex flex-row">
        {@render svg?.()}
        <span class="ml-3">{@render text?.()}</span>
    </div>
</label>
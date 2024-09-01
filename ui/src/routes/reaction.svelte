<script>
    export let id;
    export let count;
</script>

<button type="button"
        class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 text-sm">
    {#if id === 6}
        😲
    {:else if id === 5}
        😢
    {:else if id === 4}
        💓
    {:else if id === 3}
        😂
    {:else if id === 2}
        👎
    {:else if id === 1}
        👍
    {:else}
        ??
    {/if}
    {count}
</button>

<script>
    export let id;
    export let count;
    export let checked = false;

    function clicked() {
        // TODO: Propagate
        console.log("Clicked reaction " + id);
    }

    let classList = "text-gray-900 dark:text-white bg-white dark:bg-gray-800";
    if (checked) {
        classList = "text-white dark:text-gray-900 bg-blue-400 dark:bg-blue-500 hover:bg-gray-100";
    }
</script>

<button type="button"
        class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 font-medium rounded-lg text-sm px-2.5 py-1 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 text-sm">
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

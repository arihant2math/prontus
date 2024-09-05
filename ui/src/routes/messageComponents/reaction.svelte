<script>
    export let id;
    export let messageId;
    export let count;
    export let checked = false;

    $: checkBoxId = messageId + "Reaction" + id;

    function clicked() {
        // TODO: Propagate
        console.log("Clicked reaction " + id + " on message " + messageId + " to " + document.getElementById(checkBoxId).checked);
    }
</script>
<span>
    <input type="checkbox" id="{checkBoxId}" value="" class="hidden peer" required="" on:change={clicked}>
    <label for="{checkBoxId}"
       class="inline-flex items-center justify-between px-2.5 py-0.5 border-2 rounded-lg cursor-pointer
       text-gray-500 bg-white dark:hover:text-gray-300 border-gray-200 dark:border-gray-700 peer-checked:border-blue-600 peer-checked:bg-blue-600 peer-checked:text-white hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
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
    </label>
</span>
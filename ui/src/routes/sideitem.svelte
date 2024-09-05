<script>
    export let bubbleId = -1;
    export let name = "";
    export let notifications = 0;
    export let mention = false;
    export let buttonClick;

    function btnClick() {
        buttonClick(bubbleId);
    }

    $: bubbleId = bubbleId;
    $: name = name;
    $: notifications = notifications;
    $: mention = mention;
</script>
<li>
    <button on:click={btnClick} class="flex items-start p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700 w-full text-ellipsis">
        { #if notifications > 0 }
            <span class="text-left ms-3 flex-1 whitespace-nowrap text-truncate">{name}</span>
            {#if mention}
                <span class="inline-flex items-center justify-center w-3 h-3 p-3 ms-3 text-sm font-medium text-blue-800 bg-blue-100 rounded-full dark:bg-blue-900 dark:text-blue-300">{notifications}</span>
            {:else}
                <span class="inline-flex items-center justify-center px-2 ms-3 text-sm font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-700 dark:text-gray-300">{notifications}</span>
            {/if}
        {:else}
            <span class="text-left ms-3 flex-1 whitespace-nowrap text-truncate">{name}</span>
        {/if}
    </button>
</li>
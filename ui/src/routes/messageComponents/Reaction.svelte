<script>
    import {getCurrentUser, setReactionState} from "$lib/api.ts";

    export let id;
    export let messageId;
    export let users;
    export let count;
    export let currentUser;
    export let checked = users.includes(currentUser.id);

    $: checkBoxId = messageId + "Reaction" + id;

    async function clicked() {
        console.log("Clicked reaction " + id + " on message " + messageId + " to " + document.getElementById(checkBoxId).checked);
        if (document.getElementById(checkBoxId).checked) {
            count += 1;
        } else {
            count -= 1;
        }
        await setReactionState(messageId, id, document.getElementById(checkBoxId).checked);
    }
</script>
<span>
    <input type="checkbox" id="{checkBoxId}" value="" class="hidden peer" required="" on:change={clicked} checked={checked}>
    <label for="{checkBoxId}"
       class="inline-flex items-center justify-between px-2 py-0.5 border-2 rounded-lg cursor-pointer
       text-sm text-gray-500 bg-white dark:hover:text-gray-300 border-gray-200 dark:border-gray-700 peer-checked:border-blue-600 peer-checked:bg-blue-600 peer-checked:text-white hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
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
<script>
    import {getUser} from "$lib/api.ts";

    export let id;

    $: user = getUserConditional(id);

    function getUserConditional(id) {
        if (id !== 0) {
            return getUser(id);
        }

        return null;
    }
</script>

<span>
{#await user}
    loading
{:then user}
    {#if user === null}
        <span class="bg-blue-200 dark:bg-purple-800">@everyone</span>
    {:else}
        <span class="bg-blue-200 dark:bg-purple-800 hover:underline">@{user.fullname}</span>
    {/if}
{/await}
</span>

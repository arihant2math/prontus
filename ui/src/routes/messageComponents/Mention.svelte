<script>
    import {getUser} from "$lib/api.ts";

    /** @type {{id: any}} */
    let { id } = $props();


    function getUserConditional(id) {
        if (id !== 0) {
            return getUser(id);
        }

        return null;
    }
    let user = $derived(getUserConditional(id));
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

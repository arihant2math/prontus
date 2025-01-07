<script>
    import {getUser} from "$lib/api.ts";

    /** @type {{id: any}} */
    let { id } = $props();


    async function getUserConditional(id) {
        if (id !== 0) {
            try {
                return await getUser(id);
            } catch (e) {
                console.error(id, e);
                return -1;
            }
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
    {:else if user === -1}
        <span class="bg-red-200 dark:bg-red-800 hover:underline">@{id}</span>
    {:else}
        <span class="bg-blue-200 dark:bg-purple-800 hover:underline">@{user.fullname}</span>
    {/if}
{/await}
</span>

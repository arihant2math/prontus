<script>
    import ProfilePicture from "./ProfilePicture.svelte";

    export let user = null;
</script>

{#if user !== null}
    <div class="flex items-center justify-between mb-2">
        <ProfilePicture user="{user}"/>
        <div>
            <button type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-xs px-3 py-1.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800" disabled>Message</button>
        </div>
    </div>
    <p class="text-base font-semibold leading-none text-gray-900 dark:text-white">
        {user.fullname}
    </p>
    <p class="mb-3 text-sm font-normal">
        {#if user.isonline}
            <span class="text-green-500">Online</span>
        {:else}
            <span class="text-gray-500">Offline</span>
        {/if}
    </p>
{/if}
<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import ActionButton from "../ActionButton.svelte";

    /** @type {{user?: any}} */
    let { user = null, onCreateDm } = $props();
</script>

{#if user !== null}
    <div>
        <div class="flex items-center justify-between mb-2 space-x-4">
            <ProfilePicture user={user}/>
            <div>
                <ActionButton onclick={() => {onCreateDm()}} compact={true}>Message</ActionButton>
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
    </div>
{/if}
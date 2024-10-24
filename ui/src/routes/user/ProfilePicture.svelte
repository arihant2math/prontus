<script>
    /** @type {{user: any, small?: boolean}} */
    let { user, small = false } = $props();

    let sizeClasses = $derived(small ? "w-4 h-4" : "w-8 h-8");

    let flag = $state(false);

    let initials = $derived(user.fullname.split(" ").map((n) => n[0]).join(""));
    // TODO: fix
    let fontSize = $derived(small ? "text-sm" : "text-sm");

    // TODO: use hasProfilePic to detect image validity instead due to false negatives that force initials to load when they shouldn't
    function setFlag() {
        flag = true;
    }
</script>
{#if !flag}
    <img class="{sizeClasses} flex-none rounded-full select-none" src={user.profilepicurl} alt="{user.fullname} image" onerror={setFlag}>
{:else}
    <div class="{sizeClasses} flex-none relative inline-flex items-center justify-center overflow-hidden bg-gray-200 rounded-full dark:bg-gray-600">
        <span class="{fontSize} text-gray-600 dark:text-gray-300 select-none">{initials}</span>
    </div>
{/if}
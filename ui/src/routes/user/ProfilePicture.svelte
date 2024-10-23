<script>
    export let user;
    export let small = false;

    $: sizeClasses = small ? "w-4 h-4" : "w-8 h-8";

    let flag = false;

    $: initials = user.fullname.split(" ").map((n) => n[0]).join("");
    // TODO: fix
    $: fontSize = small ? "text-sm" : "text-sm";

    // TODO: use hasProfilePic to detect image validity instead due to false negatives that force initials to load when they shouldn't
    function setFlag() {
        flag = true;
    }
</script>
{#if !flag}
    <img class="{sizeClasses} flex-none rounded-full select-none" src="{user.profilepicurl}" alt="{user.fullname} image" on:error={setFlag}>
{:else}
    <div class="{sizeClasses} flex-none relative inline-flex items-center justify-center overflow-hidden bg-gray-200 rounded-full dark:bg-gray-600">
        <span class="{fontSize} text-gray-600 dark:text-gray-300 select-none">{initials}</span>
    </div>
{/if}
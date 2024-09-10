<script>
    export let user;

    let flag = false;

    $: user = user;
    $: initials = user.fullname.split(" ").map((n) => n[0]).join("");
    $: flag = flag;

    function setFlag() {
        flag = true;
    }
</script>
{#if !flag}
    <img class="w-8 h-8 rounded-full" src="{user.profilepicurl}" alt="{user.fullname} image" on:error={setFlag}>
{:else}
    <div class="relative inline-flex items-center justify-center w-8 h-8 overflow-hidden bg-gray-100 rounded-full dark:bg-gray-600">
        <span class="font-sm text-gray-600 dark:text-gray-300">{initials}</span>
    </div>
{/if}
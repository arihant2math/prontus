<script>
    import UserCard from "./user/UserCard.svelte";

    export let channelUsers;
</script>

<div class="w-max h-full overflow-y-scroll no-scrollbar">
    <ul class="flex flex-col w-max">
        {#each channelUsers as user}
            <UserCard user={user}/>
        {/each}
    </ul>
</div>
<script>
    import UserCard from "./UserCard.svelte";

    export let channelUsers;
</script>

<div class="w-fit h-full overflow-x-hidden overflow-y-scroll no-scrollbar">
    <ul class="flex flex-col w-fit">
        {#each channelUsers as user}
            <UserCard user={user}/>
        {/each}
    </ul>
</div>
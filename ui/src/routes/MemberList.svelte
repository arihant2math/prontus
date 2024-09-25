<script>
    import UserCard from "./user/UserCard.svelte";
    import {getChannelUsers, getCurrentChannelId, loadChannelUsers} from "$lib/api.ts";

    export let channelUsers;

    let lock = false;

    async function onScroll(event) {
        if (lock) {
            return
        }
        lock = true;
        if ((event.srcElement.scrollHeight - event.srcElement.scrollTop) < 900) {
            let id = await getCurrentChannelId().id;
            console.log(await loadChannelUsers(id));
            channelUsers = await getChannelUsers(id);
        }
        lock = false;
    }
</script>

<div class="w-max h-full overflow-y-scroll no-scrollbar" on:scroll={onScroll}>
    <ul class="flex flex-col w-max">
        {#each channelUsers as user}
            <UserCard user={user}/>
        {/each}
    </ul>
</div>
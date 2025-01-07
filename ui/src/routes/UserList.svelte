<script>
    import UserCard from "./user/UserListItem.svelte";
    import {getChannelUsers, getCurrentChannelId} from "$lib/api.ts";

    /** @type {{channelUsers: any}} */
    let { channelUsers = $bindable(), onCreateDm } = $props();

    let lock = false;

    async function onScroll(event) {
        if (lock) {
            return
        }
        lock = true;
        if ((event.srcElement.scrollHeight - event.srcElement.scrollTop) < 900) {
            let id = await getCurrentChannelId().id;
            channelUsers = await getChannelUsers(id);
        }
        lock = false;
    }
</script>

<div class="w-auto h-full overflow-y-scroll no-scrollbar" onscroll={onScroll}>
    <ul class="flex flex-col w-full">
        {#each channelUsers as user}
            <li class="w-full">
                <UserCard user={user} onCreateDm={onCreateDm}/>
            </li>
        {/each}
    </ul>
</div>
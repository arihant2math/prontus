<script>
    import CurrentUserCard from "./user/CurrentUserCard.svelte";
    import SideCategory from "./sidebar/SideCategory.svelte";
    import {listen} from "@tauri-apps/api/event";
    import {getChannelList} from "$lib/api.ts";
    import {parseDatetime} from "$lib/helpers.ts";
    import Sideitem from "./sidebar/ChannelCard.svelte";

    export let currentUser;
    export let showSettings;
    export let handleSidebarClick;
    export let settings;
    let channels = [];

    async function updateChannelList() {
        let unsortedChannels = await getChannelList();
        unsortedChannels.sort((a, b) => {
            if (a[1].latest_message_created_at === null) {
                return false;
            } else if (b[1].latest_message_created_at === null) {
                return true;
            }
            return parseDatetime(a[1].latest_message_created_at) < parseDatetime(b[1].latest_message_created_at)
        });
        channels = unsortedChannels.toReversed();
    }

    updateChannelList();

    listen('channelListUpdate', async (_event) => {
        await updateChannelList();
    });
</script>
<aside id="default-sidebar"
       aria-label="Sidebar"
       class="h-full">
    <div class="w-[375px] h-full z-40 bg-gray-50 dark:bg-slate-950">
        <!--TODO: maybe move this to the bottom-->
        <CurrentUserCard bind:user={currentUser} showSettings={showSettings}/>
        <ul class="space-y-2 font-medium px-3 h-full overflow-y-auto overflow-x-hidden no-scrollbar pb-20" id="sidebar-list">
            {#each channels as item}
                <li><Sideitem info={item[0]} stats={item[1]} membership={item[2]} buttonClick="{handleSidebarClick}"/></li>
            {/each}
        </ul>
    </div>
</aside>
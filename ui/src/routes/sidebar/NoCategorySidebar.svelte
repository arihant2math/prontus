<script>
    import CurrentUserCard from "../user/CurrentUserCard.svelte";
    import {listen} from "@tauri-apps/api/event";
    import {getChannelList} from "$lib/api.ts";
    import {parseDatetime} from "$lib/helpers.ts";
    import Sideitem from "./ChannelListItem.svelte";

    /** @type {{currentUser: any, handleSidebarClick: any, channelInfo: any}} */
    let { currentUser = $bindable(), onSidebarClick, channelInfo, ...on} = $props();

    let channels = $state([]);

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

    function doc_keyUp(e) {
        let altMod = e.altKey;
        let ctrlMod = e.ctrlKey || e.metaKey;
        if (altMod && e.code === 'ArrowDown') {
            // Find index
            let index = channels.findIndex((channel) => channel[0].id === channelInfo.id);
            if (index === -1) {
                index = 0;
            } else {
                index++;
            }
            if (index >= channels.length) {
                index = 0;
            }
            onSidebarClick(channels[index][0].id);
        } else if (altMod && e.code === 'ArrowUp') {
            // Find index
            let index = channels.findIndex((channel) => channel[0].id === channelInfo.id);
            if (index === -1) {
                index = 0;
            } else {
                index--;
            }
            if (index < 0) {
                index = channels.length - 1;
            }
            onSidebarClick(channels[index][0].id);
        }
    }

    document.addEventListener('keyup', doc_keyUp, false);
</script>
<aside id="default-sidebar"
       aria-label="Sidebar"
       class="h-full">
    <div style="height: 100vh" class="h-full z-40 bg-gray-50 dark:bg-slate-950">
        <!--TODO: maybe move this to the bottom-->
        <CurrentUserCard bind:user={currentUser} {...on}/>
        <ul class="space-y-2 font-medium px-3 h-full overflow-y-auto overflow-x-hidden no-scrollbar pb-20" id="sidebar-list">
            {#each channels as item}
                {#if channelInfo !== null && item[0].id === channelInfo.id}
                    <Sideitem info={item[0]} stats={item[1]} membership={item[2]} buttonClick={onSidebarClick} active={true}/>
                {:else}
                    <Sideitem info={item[0]} stats={item[1]} membership={item[2]} buttonClick={onSidebarClick}/>
                {/if}
            {/each}
        </ul>
    </div>
</aside>
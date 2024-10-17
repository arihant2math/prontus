<script>
    import CurrentUserCard from "../user/CurrentUserCard.svelte";
    import SideCategory from "./SideCategory.svelte";
    import {listen} from "@tauri-apps/api/event";
    import {getChannelList} from "$lib/api.ts";

    export let currentUser;
    export let handleSidebarClick;
    export let settings;
    export let channelInfo;

    let sidebarCategoriesInfo = {};
    let sidebarCategories = {}

    async function updateChannelList() {
        let channels = await getChannelList();
        let categories = {};
        let categoryInfo = {};
        for (let channel of channels) {
            let category = channel[0].category;
            if (category == null && channel[0].isdm) {
                category = {
                    "id": -2,
                    "title": "Direct Messages",
                };
            } else if (category == null) {
                category = {
                    "id": -1,
                    "title": "Uncategorized",
                };
            }
            if (categories[category.id] == null) {
                categories[category.id] = [];
                categoryInfo[category.id] = category;
            }
            categories[category.id].push(channel);

        }

        while (settings === null) {
            await new Promise(r => setTimeout(r, 10));
        }

        if (!settings.appearance.sidebar.hide_super_categories) {
            let recents = [];
            for (let channel of channels) {
                if (channel[1].unread > 0) {
                    recents.push(channel);
                }
            }
            categories[-3] = recents;
            categoryInfo[-3] = {
                "id": -3,
                "title": "Recents",
            };
            let pinned = [];
            for (let channel of channels) {
                if (channel[2].is_pinned) {
                    pinned.push(channel);
                }
            }

            categories[-4] = pinned;
            categoryInfo[-4] = {
                "id": -4,
                "title": "Pinned",
            };
        }

        sidebarCategoriesInfo = categoryInfo;
        sidebarCategories = categories;
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
        <CurrentUserCard bind:user={currentUser} on:showAnnouncements on:showTasks on:showDmDialog on:showSettings/>
        <ul class="space-y-2 font-medium px-3 h-full overflow-y-auto overflow-x-hidden no-scrollbar pb-20" id="sidebar-list">
            {#if sidebarCategories.hasOwnProperty(-4) && sidebarCategories[-4].length > 0}
                <SideCategory name="Pinned" items={sidebarCategories[-4]} buttonClick={handleSidebarClick} bind:channelInfo={channelInfo}/>
            {/if}
            {#if sidebarCategories.hasOwnProperty(-3) && sidebarCategories[-3].length > 0}
                <SideCategory name="Recents" items={sidebarCategories[-3]} buttonClick={handleSidebarClick} bind:channelInfo={channelInfo}/>
            {/if}
            {#each Object.keys(sidebarCategories) as category}
                {#if sidebarCategoriesInfo[category].id !== -3 && sidebarCategoriesInfo[category].id !== -4}
                    {#if sidebarCategoriesInfo[category].hasOwnProperty("user_category")}
                        <!--TODO: Fix alias not displaying properly (issue with json parsing it seems)-->
                        <SideCategory name={sidebarCategoriesInfo[category].title} items={sidebarCategories[category]} buttonClick={handleSidebarClick} bind:channelInfo={channelInfo}/>
                    {:else}
                        <SideCategory name={sidebarCategoriesInfo[category].title} items={sidebarCategories[category]} buttonClick={handleSidebarClick} bind:channelInfo={channelInfo}/>
                    {/if}
                {/if}
            {/each}
        </ul>
    </div>
</aside>
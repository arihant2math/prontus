<script>
    import {invoke} from "@tauri-apps/api/core";

    import ChannelCard from "./ChannelCard.svelte";
    import Message from "./Message.svelte";
    import Settings, {showSettings} from "./Settings.svelte";
    import SideCategory from "./SideCategory.svelte"
    import UserCard from "./UserCard.svelte"

    import {
        loadChannel,
        getChannelList,
        getMessages,
        getMoreMessages,
        loadMessages,
        sendMessage,
        getCurrentUser, getChannelInfo
    } from "./api.js";
    import {positionPopovers} from "./popup.js";

    let currentUser;
    let messages = [];
    let sidebarCategoriesInfo = {};
    let sidebarCategories = [];
    let channelInfo = null;

    async function appendMessages(newMessages) {
        messages = messages.concat(newMessages);
    }

    async function handleSidebarClick(id) {
        await loadChannel(id);
        await loadMessages();
        messages = await getMessages();
        // clear input
        document.querySelector("#messageInput").value = "";
        let messagesDiv = document.querySelector("#messages");
        messagesDiv.scrollTop = 0;
        // TODO: Below doesn't work for dms
        channelInfo = await getChannelInfo();
    }

    async function handleMessageKeyDown(event) {
        if (event.keyCode === 13) {
            await sendMessage(document.querySelector("#messageInput").value).then(async (message) => {
                document.querySelector("#messageInput").value = "";
                // TODO: Add message, but shade it to a grey color, to indicate it has not yet been sent
            });
        }
    }

    let updating = false;

    async function messageScroll(event) {
        // TODO: Fix hack, this should be global
        positionPopovers();
        if (event.target.scrollTop + event.target.scrollHeight < 1000) {
            if (updating) {
                return;
            }
            updating = true;
            console.info("Loading more messages");
            let messages = await getMessages();
            let last = messages[messages.length - 1].id;
            await getMoreMessages(last).then(async (messages) => {
                await appendMessages(messages);
            });
            updating = false;
        }
    }

    async function init() {
        currentUser = await getCurrentUser();
        let channels = await getChannelList();
        console.debug(channels);
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

        sidebarCategoriesInfo = categoryInfo;
        sidebarCategories = categories;
    }

    init().then(() => {
        console.log("Main init complete");
    });

    setInterval(async () => {
        messages = await getMessages();
    }, 10);
</script>

<Settings/>

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-slate-900 text-gray-900 dark:text-white">
    <aside id="default-sidebar"
           aria-label="Sidebar"
           class="h-full">
        <div class="w-[375px] h-full overflow-y-auto overflow-x-hidden pb-4 bg-gray-50 dark:bg-gray-800">
            <!--TODO: maybe move this to the bottom-->
            <UserCard user={currentUser} showSettings={showSettings}/>
            <ul class="space-y-2 font-medium px-3" id="sidebar-list">
                {#each Object.keys(sidebarCategories) as category}
                    <SideCategory name={sidebarCategoriesInfo[category].title} items={sidebarCategories[category]} buttonClick={handleSidebarClick}/>
                {/each}
            </ul>
        </div>
    </aside>
    <div id="content" class="h-full w-full bg-white dark:bg-slate-900 flex flex-col">
        <div>
            <ChannelCard info={channelInfo}/>
        </div>
        <div id="messages" class="overflow-y-auto bg-white dark:bg-slate-900 flex flex-col-reverse" on:scroll={messageScroll}>
            {#each messages as message}
                <!--TODO: Get repeat working-->
                <Message message={message} repeat={false} currentUser={currentUser}/>
            {/each}
        </div>
        <div class="w-full border-t border-gray-500 mt-auto bg-white dark:bg-slate-900 z-50">
            <input id="messageInput" type="text" class="text-gray-900 dark:text-white bg-white dark:bg-slate-900 outline-0 w-full border-0 h-[50px] text-base border-none px-4" on:keydown={handleMessageKeyDown}>
        </div>
    </div>
</div>

<style>
    @media (prefers-color-scheme: dark) {
        /* This part changes the scrollbar track (the part behind the scrollbar) */
        ::-webkit-scrollbar-track {
            background: #1E1E1E; /* Dark grey color, you can choose any color you like */
        }

        /* This part changes the scrollbar handle */
        ::-webkit-scrollbar-thumb {
            background: #555; /* Medium grey color, this is the actual scrollbar */
        }

        /* You can also change the scrollbar width and height */
        ::-webkit-scrollbar {
            width: 12px; /* Width of the vertical scrollbar */
            height: 12px; /* Height of the horizontal scrollbar */
        }
    }
</style>

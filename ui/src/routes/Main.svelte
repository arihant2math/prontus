<script>
    import {invoke} from "@tauri-apps/api/core";

    import ChannelCard from "./CurrentChannelCard.svelte";
    import Message from "./Message.svelte";
    import Settings, {showSettings} from "./Settings.svelte";
    import SideCategory from "./sidebar/SideCategory.svelte"
    import CurrentUserCard from "./CurrentUserCard.svelte"
    import UserCard from "./UserCard.svelte"
    import MemberList from "./MemberList.svelte";

    import {
        loadChannel,
        getChannelList,
        getMessages,
        getMoreMessages,
        loadMessages,
        sendMessage,
        getChannelUsers,
        loadChannelUsers,
        getCurrentUser, getChannelInfo
    } from "$lib/api.js";
    import {positionPopovers} from "$lib/popup.js";
    import RichTextEdit from "./messageComponents/RichTextEdit.svelte";
    import MessageList from "./MessageList.svelte";

    let currentUser;
    let messages = [];
    let sidebarCategoriesInfo = {};
    let sidebarCategories = [];
    let channelInfo = null;
    let channelUsers = [];
    let showMemberList = false;
    let showThread = false;

    async function handleSidebarClick(id) {
        if (id === await getChannelInfo().id) {
            return;
        }
        await loadChannel(id);
        await loadMessages();
        channelUsers = [];
        loadChannelUsers(id).then(async () => {
            channelUsers = await getChannelUsers(id);
        });
        messages = await getMessages();
        positionPopovers();
        // clear input
        document.querySelector("#messageInput").value = "";
        // TODO: Don't use selector
        let messagesDiv = document.querySelector("#messages");
        messagesDiv.scrollTop = 0;
        // TODO: Below doesn't work for dms
        channelInfo = await getChannelInfo();
        positionPopovers();
    }

    async function handleMessageKeyDown(event) {
        if (event.keyCode === 13 && !event.shiftKey) {
            await sendMessage(document.querySelector("#messageInput").value).then(async (message) => {
                document.querySelector("#messageInput").value = "";
                // TODO: Add message, but shade it to a grey color, to indicate it has not yet been sent
            });
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

    $: showThread = showThread;
    $: showMemberList = showMemberList;
</script>

<Settings/>

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-slate-900 text-gray-900 dark:text-white overflow-x-hidden overflow-y-hidden">
    <aside id="default-sidebar"
           aria-label="Sidebar"
           class="h-full">
        <div class="w-[375px] h-full overflow-y-auto overflow-x-hidden pb-4 bg-gray-50 dark:bg-gray-900 z-40">
            <!--TODO: maybe move this to the bottom-->
            <CurrentUserCard bind:user={currentUser} showSettings={showSettings}/>
            <ul class="space-y-2 font-medium px-3" id="sidebar-list">
                {#each Object.keys(sidebarCategories) as category}
                    <SideCategory name={sidebarCategoriesInfo[category].title} items={sidebarCategories[category]} buttonClick={handleSidebarClick}/>
                {/each}
            </ul>
        </div>
    </aside>
    <div id="content" class="h-full w-full bg-white dark:bg-slate-950 flex flex-col">
        <div>
            <ChannelCard bind:info={channelInfo} bind:memberListActive={showMemberList}/>
        </div>
        <div class="flex flex-row overflow-x-hidden overflow-y-hidden h-full">
            <div class="flex flex-col w-full overflow-x-hidden overflow-y-hidden">
                <MessageList bind:messages={messages} bind:currentUser={currentUser}/>
                <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                    <input id="messageInput" type="text" class="text-gray-900 dark:text-white bg-gray-100 dark:bg-slate-700 outline-0 w-full h-[50px] text-base border-none px-4 rounded-lg" on:keydown={handleMessageKeyDown}>
                </div>
            </div>
            {#if showMemberList}
                <MemberList bind:channelUsers={channelUsers}/>
            {/if}
        </div>
    </div>
    {#if showMemberList}
<!--        <div class="w-[350px] h-full">-->
<!--            <ul class="flex flex-col w-full">-->
<!--                {#each channelUsers as user}-->
<!--                    <UserCard user={user}/>-->
<!--                {/each}-->
<!--            </ul>-->
<!--        </div>-->
    {/if}
</div>

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }

    /* Hide scrollbar for IE, Edge and Firefox */
    .no-scrollbar {
        -ms-overflow-style: none; /* IE and Edge */
        scrollbar-width: none; /* Firefox */
    }
</style>

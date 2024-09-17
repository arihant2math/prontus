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
    import {show} from "@tauri-apps/api/app";

    let currentUser;
    let messages = [];
    let sidebarCategoriesInfo = {};
    let sidebarCategories = [];
    let channelInfo = null;
    let channelUsers = [];
    let showMemberList = false;
    let showThread = false;
    let threadParent = null;
    let messageInput;

    const getThreadMessages = () => {
        if (threadParent === null) {
            showThread = false;
            return;
        }
        showMemberList = false;
        showThread = true;

        let msgs = [];
        for (let message of messages) {
            if (message.parentmessage_id === threadParent) {
                console.log(message.parentmessage_id === threadParent)
                msgs.push(message);
            } else if (message.id === threadParent) {
                msgs.push(message);
            }
        }
        return msgs;
    }

    $: threadMessages = getThreadMessages(threadParent);



    // TODO: non-jank loading (bar on the top or smth and get channel card and messages load at once)
    async function handleSidebarClick(id) {
        if (id === await getChannelInfo().id) {
            return;
        }
        threadParent = null;
        showMemberList = true;
        await loadChannel(id);
        let messagesPromise = loadMessages().then(async () => {
            messages = await getMessages();
            positionPopovers();
        });
        channelUsers = [];
        let usersPromise = loadChannelUsers(id).then(async () => {
            channelUsers = await getChannelUsers(id);
            positionPopovers();
        });
        let channelPromise = getChannelInfo().then((info) => {
            channelInfo = info;
        });
        await Promise.all([messagesPromise, usersPromise, channelPromise]);
        // clear input
        messageInput.clear();
        // TODO: Below doesn't work for dms
        positionPopovers();
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

    function viewThread(parentId) {
        threadParent = parentId;
    }

    init().then(() => {
        console.log("Main init complete");
    });
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
    <div id="content" class="h-full w-full bg-white dark:bg-slate-950 flex flex-col overflow-x-hidden overflow-y-hidden">
        <div>
            <ChannelCard info={channelInfo} bind:memberListActive={showMemberList}/>
        </div>
        <div class="flex flex-row overflow-x-hidden overflow-y-hidden h-full bg-white dark:bg-slate-900">
            <div class="flex flex-col w-full overflow-x-hidden overflow-y-hidden ml-4">
                <MessageList id="messagesDiv" bind:messages={messages} bind:currentUser={currentUser} viewThread={viewThread}/>
                <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                    <RichTextEdit bind:this={messageInput} sendMessage={sendMessage}/>
                </div>
            </div>
            {#if showMemberList && !showThread}
                <MemberList bind:channelUsers={channelUsers}/>
            {/if}
            {#if showThread}
                <button class="fixed top-12 right-4 bg-white dark:bg-slate-800 hover:bg-gray-100 dark:hover:bg-slate-700 p-1" on:click={() => {showThread = false}}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                </button>
                <div class="w-max h-full overflow-x-hidden overflow-y-hidden">
                    <MessageList id="threadMessagesDiv" bind:messages={threadMessages} bind:currentUser={currentUser} inThread=true/>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .dark {
        color-scheme: dark;
    }

    .no-scrollbar {
      overflow-y: scroll;
      scrollbar-width: none; /* Firefox */
      -ms-overflow-style: none;  /* IE 10+ */
    }
    .no-scrollbar::-webkit-scrollbar { /* WebKit */
      width: 0px;
    }

</style>

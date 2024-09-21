<script>
    import ChannelCard from "./CurrentChannelCard.svelte";
    import Settings, {showSettings} from "./Settings.svelte";
    import MemberList from "./MemberList.svelte";

    import {
        loadChannel,
        getMessages,
        loadMessages,
        sendMessage,
        getChannelUsers,
        loadChannelUsers,
        getCurrentUser, getChannelInfo, getParentMessages, getSettings
    } from "$lib/api.ts";
    import {positionPopovers} from "$lib/popup.js";
    import RichTextEdit from "./messageComponents/RichTextEdit.svelte";
    import MessageList from "./MessageList.svelte";
    import {listen} from "@tauri-apps/api/event";
    import Sidebar from "./Sidebar.svelte";
    import {loadTheme} from "$lib/helpers.ts";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";

    let currentUser;
    let messages = [];
    let parentMessages = [];
    let channelInfo = null;
    let channelUsers = [];
    let showMemberList = false;
    let showThread = false;
    let threadParent = null;
    let messageInput;
    let settings = null;

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



    // TODO: progress bar on the top or smth
    async function handleSidebarClick(id) {
        if (id === await getChannelInfo().id) {
            return;
        }
        threadParent = null;
        showMemberList = true;
        await loadChannel(id);
        let messagesPromise = loadMessages().then(async () => {
            messages = await getMessages();
            parentMessages = await getParentMessages();
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
        getSettings().then((result) => {
            settings = result;
            loadTheme(settings);
        });
        currentUser = await getCurrentUser();
    }

    function viewThread(parentId) {
        threadParent = parentId;
    }

    async function queuedSendMessage(message, threadId) {
        sendMessage(message, threadId).then();
        // TODO: implement
    }

    init().then(() => {
        console.log("Main init complete");
    });

    listen('messageListUpdate', async (_event) => {
        messages = await getMessages();
        parentMessages = await getParentMessages();
    });
</script>
{#if settings !== null}
    <Settings bind:settings={settings}/>
{/if}

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-slate-900 text-gray-900 dark:text-white overflow-x-hidden overflow-y-hidden">
    {#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
        <NoCategorySidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}/>
    {:else}
        <Sidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}/>
    {/if}
    <div id="content" class="h-full w-full bg-white dark:bg-slate-950 flex flex-col overflow-x-hidden overflow-y-hidden">
        <div>
            <ChannelCard bind:info={channelInfo} bind:memberListActive={showMemberList}/>
        </div>
        <div class="flex flex-row overflow-x-hidden overflow-y-hidden h-full bg-white dark:bg-slate-900">
            <div class="flex flex-col w-full overflow-x-hidden overflow-y-hidden ml-4">
                <MessageList id="messagesDiv" bind:messages={messages} bind:parentMessages={parentMessages} bind:currentUser={currentUser} viewThread={viewThread}/>
                <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                    <RichTextEdit bind:this={messageInput} sendMessage={async (text) => {queuedSendMessage(text, null)}}/>
                </div>
            </div>
            {#if showMemberList && !showThread}
                <MemberList bind:channelUsers={channelUsers}/>
            {/if}
            {#if showThread}
                <div class="w-max h-full overflow-x-hidden overflow-y-hidden">
                    <button class="fixed top-4 right-4 bg-white dark:bg-slate-800 hover:bg-gray-100 dark:hover:bg-slate-700 p-1 rounded-lg" on:click={() => {showThread = false}}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                        </svg>
                    </button>
                    <div class="flex flex-col w-full h-full overflow-x-hidden overflow-y-hidden ml-4">
                        <MessageList id="threadMessagesDiv" bind:messages={threadMessages} bind:parentMessages={parentMessages} bind:currentUser={currentUser} inThread={true}/>
                        <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                            <RichTextEdit sendMessage={async (text) => {queuedSendMessage(text, threadParent)}}/>
                        </div>
                    </div>
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

    .no-scrollbar {
        scrollbar-color: transparent transparent;
    }
</style>
